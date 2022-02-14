use crate::utils::parse_utils::{
    extract_matching_calls, matches_to_extraction_results, strip_comments,
};
use crate::{ExtractionResult, LanguageContext};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::ops::Deref;

lazy_static! {
    pub static ref PHP_CONTEXT: LanguageContext = LanguageContext {
        single_line_comment_starts: vec!["//"],
        multi_line_comment_starts: vec![("/*", "*/")],
        single_line_string_starts: Vec::new(),
        multi_line_string_starts: vec!["'", "\""],
        call_open_sym: b'(',
        call_close_sym: b')',
        special_regex_char: None
    };
    pub static ref PHP_SUPPORTED_FUNCTIONS: Vec<String> = vec![
        "preg_match".to_string(),
        "preg_match_all".to_string(),
        "preg_replace".to_string(),
        "preg_filter".to_string(),
        "preg_grep".to_string(),
        "preg_last_error_msg".to_string(),
        "preg_last_error".to_string(),
        "preg_quote".to_string(),
        "preg_replace_callback_array".to_string(),
        "preg_replace_callback".to_string(),
        "preg_split".to_string()
    ];
}

pub fn extract_php(contents_raw: String) -> Option<Vec<ExtractionResult>> {
    let context: &LanguageContext = PHP_CONTEXT.borrow().deref();
    let supported_functions: Vec<String> = PHP_SUPPORTED_FUNCTIONS.deref().clone();
    let contents = strip_comments(contents_raw, context);
    let matches = extract_matching_calls(contents.clone().as_str(), context, supported_functions);
    return if matches.len() > 0 {
        Some(matches_to_extraction_results(contents.clone(), matches))
    } else {
        None
    };
}
