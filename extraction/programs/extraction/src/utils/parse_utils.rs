use crate::utils::{tokenize, TokenizationState};
use crate::{cursor_from_string, ExtractionResult, LanguageContext, CHAR_SENTINEL};
use std::io::{BufRead, Cursor};
use std::str;

/// Calculate the line number that corresponds to a given offset
///
/// # Examples
///
/// ```rust
/// use extraction::utils::parse_utils::line_no_from_offset;
///
/// let str: &str = "l1\nl2\nl3";
/// assert_eq!(line_no_from_offset(str.to_string(), 4), 2);
/// ```
pub fn line_no_from_offset(contents: String, start: usize) -> i32 {
    let mut lines = 0;
    let mut offset: usize = 0;

    let mut buffer: Cursor<&[u8]> = cursor_from_string!(contents);
    let mut line_buf = String::new();

    while offset < start {
        match buffer.read_line(&mut line_buf) {
            Ok(chars_read) => offset += chars_read,
            Err(_) => return lines,
        }
        lines += 1
    }

    lines
}

pub fn strip_comments(contents: String, language_context: &LanguageContext) -> String {
    tokenize(contents, language_context, None)
}

fn get_balanced_function_call(contents: String, language_context: &LanguageContext) -> String {
    let new_contents = contents.clone();
    let mut cursor: Cursor<&[u8]> = cursor_from_string!(new_contents);

    let mut start_buf: Vec<u8> = Vec::new();
    let read_res = cursor.read_until(language_context.call_open_sym, &mut start_buf);

    match read_res {
        Ok(_) => tokenize(
            contents,
            language_context,
            Some(&mut |state, curr, _| {
                state == TokenizationState::Code && curr == language_context.call_close_sym
            }),
        ),
        Err(_) => "".to_string(),
    }
}

fn strip_match(found_match: String, language_context: &LanguageContext) -> Option<String> {
    let mut end_str: String = String::new();
    let mut match_work: String;
    let mut seen_string = false;
    match_work = tokenize(
        found_match.clone(),
        language_context,
        Some(&mut |state, _, _| match state {
            TokenizationState::String(s) | TokenizationState::MultiLineString(s) => {
                if !seen_string {
                    end_str = s.clone();
                    seen_string = true;
                }
                false
            }
            TokenizationState::Code => seen_string,
            _ => false,
        }),
    );
    // This is referring to a variable in this case which we won't be able to statically
    // retrieve the value of, so don't include it in our matches
    if !seen_string {
        return None;
    }
    while !match_work.starts_with(&end_str) {
        match_work.remove(0);
    }
    while !match_work.ends_with(&end_str) {
        match_work.pop();
    }
    match_work = match_work.strip_prefix(&end_str).unwrap().to_string();
    Some(match_work.strip_suffix(&end_str).unwrap().to_string())
}

/// Helper function to extract matching regex calls from the source string,
/// with the help of the mini tokenizer. Assume for the sake of simplicity that the regex
/// will always be the first argument to the relevant function (it is for all of the
/// standard libraries of the languages used here)
pub fn extract_matching_calls(
    contents: &str,
    language_context: &LanguageContext,
    relevant_functions: Vec<String>,
) -> Vec<(String, usize)> {
    let mut matches: Vec<(String, usize)> = Vec::new();
    let mut track_contents: String = contents.to_string();
    let mut idx = 0;
    while !track_contents.is_empty() {
        track_contents.remove(0);
        idx += 1;
        for rel_func in &relevant_functions {
            if let Some(new_str) = track_contents.strip_prefix(rel_func) {
                track_contents = new_str.to_string();
                let func_args =
                    get_balanced_function_call(track_contents.to_string(), language_context);
                let match_res = strip_match(func_args, language_context);

                match match_res {
                    Some(m) => matches.push((m, idx)),
                    None => {}
                }

                idx += rel_func.len();
            }
        }
    }
    matches
}

pub fn matches_to_extraction_results(
    contents: String,
    matches: Vec<(String, usize)>,
) -> Vec<ExtractionResult> {
    matches
        .iter()
        .map(|m| ExtractionResult {
            expression: m.0.clone(),
            line_number: line_no_from_offset(contents.clone(), m.1),
        })
        .collect()
}

/// Helper function that, for languages like JS and PHP that have special syntactic
/// sugar for regular expressions, will replace instances of it with their standard invocation,
/// so they can be parsed
pub fn replace_regex_char(
    contents: String,
    context: &LanguageContext,
    replacement: &str,
) -> String {
    // Choose a value as a sentinel that's not present in the single line chars
    if contents.contains(CHAR_SENTINEL) {
        panic!("Sentinel clash");
    }
    let special_regex_char: u8;
    if let Some(r) = context.special_regex_char {
        special_regex_char = r;
    } else {
        return contents;
    }
    let contents_str = contents.as_str();
    let mut new_contents = String::from(contents_str);
    let mut prev_contents: String = String::new();
    let mut seen_beginning = false;
    let orig_content_lines: Vec<&str> = contents.lines().collect();
    let mut started_replace: Option<i32> = None;

    // Tokenize this section in a loop until we've replaced all instances.
    // This section of the code is pretty ugly - basically, I'm over-using the tokenizer in an
    // attempt to replace instances of a given char, used in a regex, with the corresponding
    // regex function. Where things get bad is determining the difference between that char (usually '/')
    // in normal usage, such as division, and for a regex. From the limited perspective of the tokenizer,
    // it's very hard to distinguish between let i = x / 2 / 3;  and let i = /\d+/;.
    // The solution is to add a series of checks to correlated parts of the state, like the line that the
    // program saw the first slash on, etc., but it makes this code really messy.
    loop {
        let mut triggered = false;
        let mut new_partial = tokenize(
            new_contents.to_string(),
            context,
            Some(&mut |state, curr, line_no: i32| {
                // If we've started a replacement, but we realize that
                // the character was being used in some other way,
                // revert back to the previous state
                if let Some(start_line) = started_replace {
                    let mut valid_continuation: bool = true;
                    if line_no > start_line {
                        valid_continuation = false;
                    } else {
                        match state {
                            TokenizationState::MultiLineComment(_) | TokenizationState::SingleLineComment => {
                                valid_continuation = false;
                            }
                            _ => {}
                        }
                    }
                    if !valid_continuation {
                        new_contents = prev_contents.clone();
                        started_replace = None;
                        triggered = false;
                        seen_beginning = false;
                        return false;
                    }
                }
                // If we hit the special regex character in normal code, break
                if state == TokenizationState::Code && curr == special_regex_char {
                    // Before even starting a replace, find the line that this occured on
                    let relevant_line: &str = orig_content_lines[line_no as usize];
                    // Count the occurrences of the special char in the line
                    let mut occurrences = 0;
                    for char in relevant_line.chars() {
                        let char_byte = char as u8;
                        if char_byte == special_regex_char {
                            occurrences += 1;
                        }
                    }
                    // Sanity check
                    if occurrences == 0 {
                        panic!("Something's wrong with special character replacement")
                    }

                    if occurrences %2 != 0 {
                        return false
                    }

                    started_replace = Some(line_no);
                    prev_contents = new_contents.clone();
                    triggered = true;
                    true
                } else {
                    false
                }
            }),
        );
        let partial_idx = new_partial.len();
        if triggered {
            new_partial.pop();
            if seen_beginning {
                // If we're here, we've seen an end, so finish up the string
                new_partial.push_str(CHAR_SENTINEL);
                new_partial.push(context.call_close_sym.into());
                seen_beginning = false;
                started_replace = None;
            } else {
                new_partial.push_str(replacement);
                new_partial.push(context.call_open_sym.into());
                new_partial.push_str(CHAR_SENTINEL);
                seen_beginning = true;
            }
            if partial_idx < new_contents.len() {
                let end = &new_contents[partial_idx..new_contents.len()];
                new_contents = String::from(new_partial.to_string() + end);
            } else {
                break;
            }
        } else {
            // Once the tokenizer didn't see any instances of the special regex character on this line,
            // we're done
            break;
        }
    }

    // Determine which string start to use
    let string_start: &str;

    if context.single_line_string_starts.len() > 0 {
        string_start = context.single_line_string_starts.first().unwrap();
    } else {
        if context.multi_line_string_starts.len() > 0 {
            string_start = context.multi_line_string_starts.first().unwrap();
        } else {
            panic!("Language has no valid string starts");
        }
    }

    new_contents.replace(CHAR_SENTINEL, string_start)
}
