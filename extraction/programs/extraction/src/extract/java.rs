use crate::utils::parse_utils::{
    extract_matching_calls, matches_to_extraction_results, strip_comments,
};
use crate::{ExtractionResult, LanguageContext};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::ops::Deref;

lazy_static! {
    pub static ref JAVA_CONTEXT: LanguageContext = LanguageContext {
        single_line_comment_starts: vec!["//"],
        multi_line_comment_starts: vec![("/*", "*/")],
        single_line_string_starts: vec!["'", "\""],
        multi_line_string_starts: Vec::new(),
        call_open_sym: b'(',
        call_close_sym: b')',
        special_regex_char: None
    };
    pub static ref JAVA_SUPPORTED_FUNCTIONS: Vec<String> = vec![
        "Pattern.compile".to_string(),
        "regex.Pattern.compile".to_string()
    ];
}

pub fn extract_java(contents_raw: String) -> Option<Vec<ExtractionResult>> {
    let context: &LanguageContext = JAVA_CONTEXT.borrow().deref();
    let supported_functions: Vec<String> = JAVA_SUPPORTED_FUNCTIONS.deref().clone();
    let contents = strip_comments(contents_raw, context);
    let matches = extract_matching_calls(contents.clone().as_str(), context, supported_functions);
    return if matches.len() > 0 {
        Some(matches_to_extraction_results(contents.clone(), matches))
    } else {
        None
    };
}
