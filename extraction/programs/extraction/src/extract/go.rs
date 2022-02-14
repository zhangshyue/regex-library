use crate::utils::parse_utils::{
    extract_matching_calls, matches_to_extraction_results, strip_comments,
};
use crate::{split_string, ExtractionResult, LanguageContext};
use lazy_static::lazy_static;
use std::borrow::Borrow;
use std::ops::Deref;

lazy_static! {
    pub static ref GO_CONTEXT: LanguageContext = LanguageContext {
        single_line_comment_starts: vec!["//"],
        multi_line_comment_starts: vec![("/*", "*/")],
        single_line_string_starts: vec!["'", "\""],
        multi_line_string_starts: vec!["`"],
        call_open_sym: b'(',
        call_close_sym: b')',
        special_regex_char: None
    };
    // https://pkg.go.dev/regexp
    pub static ref GO_SUPPORTED_FUNCTIONS: Vec<String> = vec![
        "Compile",
        "CompilePOSIX",
        "MustCompile",
        "Copy",
        "Expand",
        "ExpandString",
        "Find",
        "FindAll",
        "FindAllIndex",
        "FindAllString",
        "FindAllStringIndex",
        "FindAllStringSubmatch",
        "FindAllStringSubmatchIndex",
        "FindAllSubmatch",
        "FindAllSubmatchIndex",
        "FindIndex",
        "FindReaderIndex",
        "FindReaderSubmatchIndex",
        "FindString",
        "FindStringIndex",
        "FindStringSubmatch",
        "FindStringSubmatchIndex",
        "FindSubmatch",
        "FindSubmatchIndex",
        "LiteralPrefix",
        "Longest",
        "Match",
        "MatchReader",
        "MatchString",
        "NumSubexp",
        "ReplaceAll",
        "ReplaceAllFunc",
        "ReplaceAllLiteral",
        "ReplaceAllString",
        "ReplaceAllStringFunc",
        "Split",
        "String",
        "SubexpIndex",
        "SubexpNames"
    ].iter().map(|i| format!("regexp.{}", i.to_string()).to_string()).collect();
}

pub fn program_imports_regexp(contents: String) -> bool {
    return if contents.contains("import") {
        let post_import = split_string!(contents, "import", 1).trim().to_string();
        if post_import.starts_with("\"regexp\"") {
            true
        } else {
            return if post_import.starts_with("(") {
                let mut imports_collection = split_string!(post_import, "(", 1).to_string();
                imports_collection = split_string!(imports_collection, ")", 0).to_string();
                let imports: Vec<String> = split_string!(imports_collection, ",")
                    .iter()
                    .map(|i| i.to_string().trim_matches('\n').trim().to_string())
                    .collect();

                imports.contains(&"\"regexp\"".to_string())
            } else {
                false
            };
        }
    } else {
        false
    };
}

pub fn extract_go(contents_raw: String) -> Option<Vec<ExtractionResult>> {
    let context: &LanguageContext = GO_CONTEXT.borrow().deref();
    let valid_imports = GO_SUPPORTED_FUNCTIONS.deref().clone();
    let contents = strip_comments(contents_raw, context);
    if !program_imports_regexp(contents.clone()) {
        return None;
    }
    let matches = extract_matching_calls(contents.clone().as_str(), context, valid_imports);
    return if matches.len() > 0 {
        Some(matches_to_extraction_results(contents.clone(), matches))
    } else {
        None
    };
}
