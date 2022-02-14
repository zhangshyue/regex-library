use crate::LanguageContext;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum TokenizationState {
    MultiLineComment((String, String)),
    SingleLineComment,
    String(String),
    MultiLineString(String),
    StringEscape(Rc<TokenizationState>),
    Code,
}

/// Helper function to, in a non-panicking way, check if the upcoming characters
/// in a string, plus the current, are equal to a given string
///
/// # Examples
/// ```rust
/// use extraction::utils::upcoming_chars_match_sequence;
///
/// let my_str: &str = "Hello, world";
/// // After "Hello, "
/// let idx: usize = 7;
/// assert!(upcoming_chars_match_sequence(my_str, idx, "world"))
/// ```
pub fn upcoming_chars_match_sequence(text: &str, idx: usize, patt: &str) -> bool {
    let patt_len = patt.len();
    let upcoming = text.get(idx..idx + patt_len);
    match upcoming {
        Some(m) if m == patt => true,
        _ => false,
    }
}

macro_rules! consume_if_match {
    ($text: expr, $idx: expr, $patt: expr) => {
        if upcoming_chars_match_sequence($text, $idx, $patt) {
            $idx += $patt.len();
            true
        } else {
            false
        }
    };
}

// The following is a somewhat opaque helper macro that just serves to help with
// state transitions in the tokenization. Basically, rust has weird
// rules about how you can use these value enum types in a loop,
// and this is ended up being one of the better ways to express the
// transitions I wanted

macro_rules! transition_if_matching_member {
    ($text: expr,
     $idx: expr,
     $state: expr,
     $new_state: expr,
     $members: expr,
     $member_accessor: expr) => {{
        let mut val: bool = false;
        for member in $members.clone() {
            if upcoming_chars_match_sequence($text, $idx, $member_accessor(member)) {
                $state = $new_state(member);
                val = true;
                break;
            }
        }
        val
    }};
    ($text: expr,
     $idx: expr,
     $state: expr,
     $new_state: expr,
     $members: expr) => {
        transition_if_matching_member!($text, $idx, $state, $new_state, $members, |m| { m })
    };
    ($text: expr,
     $idx: expr,
     $state: expr,
     $new_state: expr,
     ^$members: expr) => {
        transition_if_matching_member!($text, $idx, $state, $new_state, $members, |m: (_, _)| {
            m.0
        })
    };
    ($text: expr,
     $idx: expr,
     $state: expr,
     ~$new_state: expr,
     $members: expr) => {
        transition_if_matching_member!($text, $idx, $state, |_| { $new_state }, $members)
    };
}

/// A mini tokenizer that I hacked together to help with different program functionality,
/// like removing comments from a file, and finding the argument that a function is called with.
/// The base case of this is removing comments, and the tokenizer always has the side effect of
/// removing comments from the output string. In addition, it can be provided a custom stop condition,
/// or be allowed to run to the end
///
/// Assume '\' is the universal escape character,
/// for ease of use. It is with the languages used in this project
pub fn tokenize(
    contents: String,
    language_context: &LanguageContext,
    end_condition: Option<&mut dyn FnMut(TokenizationState, u8, i32) -> bool>,
) -> String {
    let mut new_content = String::new();
    let mut state = TokenizationState::Code;
    let escape_chars = vec!["\\"];

    // This has to be brought out of the loop because of the way the rost borrow system
    // treats mutable closure references
    let end_fn: &mut dyn FnMut(TokenizationState, u8, i32) -> bool;
    let temp_fn = &mut |_, _, _| false;
    match end_condition {
        Some(f) => end_fn = f,
        None => end_fn = temp_fn,
    }

    let mut line_no: i32 = 0;

    // Break it down into lines so we can deal with things that
    // are exclusively single or multi line
    for line in contents.lines() {
        let mut idx = 0;
        let line_len = line.len();
        'line: while idx < line_len {
            // Tokenization is done through a series of state transitions -
            // essentially, the states that are tracked are types of comments,
            // and types of sequences which can preclude a comment character from being
            // a comment, namely a string
            match state.clone() {
                // If we're in the code, check a series of possible transition triggers
                TokenizationState::Code => {
                    if !transition_if_matching_member!(line,
                        idx,
                        state,
                        ~TokenizationState::SingleLineComment,
                        language_context.single_line_comment_starts
                    ) {
                        if !transition_if_matching_member!(line,
                            idx,
                            state,
                            |m: (&str, &str)| {
                                TokenizationState::MultiLineComment(
                                    (String::from(m.0), String::from(m.1)))
                            },
                            ^language_context.multi_line_comment_starts
                        ) {
                            if !transition_if_matching_member!(
                                line,
                                idx,
                                state,
                                |m: &str| { TokenizationState::String(String::from(m)) },
                                language_context.single_line_string_starts
                            ) {
                                if !transition_if_matching_member!(
                                    line,
                                    idx,
                                    state,
                                    |m: &str| {
                                        TokenizationState::MultiLineString(String::from(m))
                                    },
                                    language_context.multi_line_string_starts
                                ) {
                                    if !transition_if_matching_member!(
                                        line,
                                        idx,
                                        state,
                                        |_| { TokenizationState::StringEscape(Rc::new(state)) },
                                        escape_chars
                                    ) {
                                        // Sanity check
                                        assert_eq!(state, TokenizationState::Code);
                                    }
                                }
                            }
                        }
                    }
                }
                TokenizationState::StringEscape(prev) => {
                    // Write whatever's next regardless, skip 1, and return to the previous state
                    let line_opt = line.chars().nth(idx);
                    if line_opt == None {
                        // This can sometimes happen with non-ascii characters - find to just ignore
                        break 'line;
                    }
                    new_content.push(line_opt.unwrap());
                    idx += 1;
                    match prev.deref() {
                        TokenizationState::String(s) => TokenizationState::String(s.clone()),
                        TokenizationState::MultiLineString(s) => {
                            TokenizationState::MultiLineString(s.clone())
                        }
                        TokenizationState::Code => TokenizationState::Code,
                        TokenizationState::MultiLineComment((s0, s1)) => {
                            TokenizationState::MultiLineComment((s0.clone(), s1.clone()))
                        }
                        TokenizationState::SingleLineComment => {
                            TokenizationState::SingleLineComment
                        }
                        _ => {
                            panic!("Can't parse string escape");
                        }
                    };
                }
                // If we reach the end of a multi line comment, or any type of string,
                // transition back to code
                TokenizationState::MultiLineComment((_, end)) => {
                    if consume_if_match!(line, idx, end.as_str()) {
                        state = TokenizationState::Code;
                    }
                }
                TokenizationState::String(s) => {
                    if upcoming_chars_match_sequence(line, idx, s.as_str()) {
                        state = TokenizationState::Code
                    }
                }
                TokenizationState::MultiLineString(s) => {
                    if upcoming_chars_match_sequence(line, idx, s.as_str()) {
                        state = TokenizationState::Code
                    }
                }
                _ => {}
            }
            // If we're in anything other than one of the comment states,
            // write a character to the output
            match state {
                TokenizationState::MultiLineComment(_) => {}
                TokenizationState::SingleLineComment => {}
                _ => {
                    let char = line.chars().nth(idx);
                    match char {
                        Some(c) => new_content.push(c),
                        // If we're at the end of the line, don't write anything
                        None => {}
                    }
                }
            }
            if !new_content.is_empty() {
                if end_fn(state.clone(), new_content.chars().last().unwrap() as u8, line_no) {
                    return new_content;
                };
            }

            // Go to the next character
            idx += 1;
        }

        // Clear any single line state
        match state {
            TokenizationState::SingleLineComment => state = TokenizationState::Code,
            TokenizationState::String(_) => state = TokenizationState::Code,
            _ => {}
        }

        // Make up for the EOF or \n we left out - we don't care about replacing an
        // EOF with a \n for these purposes
        new_content.push('\n');
        line_no+=1;
    }

    new_content
}
