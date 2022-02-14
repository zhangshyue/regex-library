use crate::utils::parse_utils::{
    extract_matching_calls, matches_to_extraction_results, strip_comments,
};
use crate::{ExtractionResult, LanguageContext};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::ops::Deref;

lazy_static! {
    pub static ref RUST_CONTEXT: LanguageContext = LanguageContext {
        single_line_string_starts: vec!["\"", "'"],
        multi_line_comment_starts: vec![("/*", "*/")],
        single_line_comment_starts: vec!["//"],
        multi_line_string_starts: Vec::new(),
        special_regex_char: None,
        call_close_sym: b')',
        call_open_sym: b'('
    };
    pub static ref RUST_SUPPORTED_FUNCTIONS: Vec<String> = vec!["Regex::new".to_string()];
}

pub fn extract_rust(contents_raw: String) -> Option<Vec<ExtractionResult>> {
    let context: &LanguageContext = RUST_CONTEXT.borrow().deref();
    let supported_functions: Vec<String> = RUST_SUPPORTED_FUNCTIONS.deref().clone();
    let contents = strip_comments(contents_raw, context);
    let matches = extract_matching_calls(contents.clone().as_str(), context, supported_functions);
    return if matches.len() > 0 {
        Some(matches_to_extraction_results(contents.clone(), matches))
    } else {
        None
    };
}
