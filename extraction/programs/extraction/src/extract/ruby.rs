use crate::utils::parse_utils::{
    extract_matching_calls, matches_to_extraction_results, replace_regex_char, strip_comments,
};
use crate::{ExtractionResult, LanguageContext};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::ops::Deref;

// For convenience, we don't support the full range of Ruby's esoteric regex features,
// just a few common usages of it
lazy_static! {
    pub static ref RUBY_CONTEXT: LanguageContext = LanguageContext {
        single_line_comment_starts: vec!["#"],
        multi_line_comment_starts: Vec::new(),
        multi_line_string_starts: vec!["'", "\"", "EOS", "HEREDOC"],
        single_line_string_starts: Vec::new(),
        call_open_sym: b'(', // This isn't really true for ruby, but we only care about the .match() case -
        call_close_sym: b')', // to support ruby method calls without parens, the tokenizer would have to be rewritten
        special_regex_char: Some(b'/')
    };
    pub static ref RUBY_SUPPORTED_FUNCTIONS: Vec<String> = vec![
        "Regexp.new".to_string()
    ];
}

pub fn extract_ruby(contents_raw: String) -> Option<Vec<ExtractionResult>> {
    let context: &LanguageContext = RUBY_CONTEXT.borrow().deref();
    let supported_functions: Vec<String> = RUBY_SUPPORTED_FUNCTIONS.deref().clone();
    let mut contents = strip_comments(contents_raw, context);
    contents = replace_regex_char(contents, context, "Regexp.new");
    let matches = extract_matching_calls(contents.clone().as_str(), context, supported_functions);
    return if matches.len() > 0 {
        Some(matches_to_extraction_results(contents.clone(), matches))
    } else {
        None
    };
}
