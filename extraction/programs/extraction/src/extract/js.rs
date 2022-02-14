use crate::utils::parse_utils::{
    extract_matching_calls, matches_to_extraction_results, replace_regex_char, strip_comments,
};
use crate::{ExtractionResult, LanguageContext};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::ops::Deref;

lazy_static! {
    pub static ref JS_CONTEXT: LanguageContext = LanguageContext {
        single_line_comment_starts: vec!["//"],
        multi_line_comment_starts: vec![("/*", "*/")],
        single_line_string_starts: vec!["'", "\""],
        multi_line_string_starts: vec!["`"],
        call_open_sym: b'(',
        call_close_sym: b')',
        special_regex_char: Some(b'/')
    };
    pub static ref JS_SUPPORTED_FUNCTIONS: Vec<String> = vec![
        "exec",
        "test",
        "match",
        "matchAll",
        "search",
        "replace",
        "replaceAll",
        "split",
    ].iter().map(|i| {
        format!(".{}(", i)
    }).collect();
}

pub fn extract_js(contents_raw: String) -> Option<Vec<ExtractionResult>> {
    let context: &LanguageContext = JS_CONTEXT.borrow().deref();

    let mut contents = strip_comments(contents_raw, context);
    // Replace any instances of for ex. /(.*)/ with new RegExp("(.*)"). This call is expensive, so
    // before doing it, check to see if regexes are even used in this files
    let supported_functions: Vec<String> = JS_SUPPORTED_FUNCTIONS.deref().clone();
    let mut found_usage: bool = false;
    for func in supported_functions {
        if contents.contains(&func) {
            found_usage = true;
            break;
        }
    }
    if !found_usage {
        return None;
    }
    contents = replace_regex_char(contents, context, "new RegExp");
    let valid_imports = vec!["new RegExp".to_string()];
    let matches = extract_matching_calls(contents.as_str(), context, valid_imports);
    if matches.len() > 0 {
        Some(matches_to_extraction_results(contents, matches))
    } else {
        None
    }
}
