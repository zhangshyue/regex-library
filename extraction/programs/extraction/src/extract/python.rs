use crate::utils::parse_utils::{
    extract_matching_calls, matches_to_extraction_results, strip_comments,
};
use crate::{split_string, ExtractionResult, LanguageContext};
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;
use std::ops::Deref;

lazy_static! {
    pub static ref SUPPORTED_FUNCTIONS: Vec<String> = vec![
        String::from("compile"),
        String::from("search"),
        String::from("match"),
        String::from("fullmatch"),
        String::from("split"),
        String::from("findall"),
        String::from("finditer"),
        String::from("sub"),
    ];
    static ref PY_IMPORT_PATT: Regex =
        Regex::new("(?:import|from) re\\b(?:\\n|(?: import)?)(?: as (.*))?").unwrap();
    static ref PY_REGEX_LIBRARY: String = String::from("re");
    pub static ref PY_CONTEXT: LanguageContext = LanguageContext {
        single_line_comment_starts: vec!["#"],
        multi_line_comment_starts: Vec::new(),
        single_line_string_starts: vec!["'", "\""],
        multi_line_string_starts: vec!["\"\"\""],
        call_open_sym: b'(',
        call_close_sym: b')',
        special_regex_char: None
    };
}

/// Helper function to get imports out of a possible as statment
///
/// # Examples
/// ```rust
/// use crate::extraction::extract::resolve_standalone_import;
/// let test_string = String::from("x as y, z");
/// assert_eq!(resolve_standalone_import(test_string), vec![String::from("y"), String::from("z")])
/// ```
pub fn resolve_standalone_import(all_imports: String) -> Vec<String> {
    let mut resolved_imports: Vec<String> = Vec::new();
    let imports = split_string!(all_imports, ",");
    for mut import in imports {
        import = import.trim();
        if import.contains("as") {
            let final_import = split_string!(import, "as", 1).trim();
            resolved_imports.push(final_import.to_string());
        } else {
            resolved_imports.push(import.to_string());
        }
    }
    resolved_imports
}

fn format_base(base: String, func: String) -> String {
    return if base.is_empty() {
        func
    } else {
        format!("{}.{}", base, func)
    };
}

pub fn parse_import_line(line: String) -> Option<Vec<String>> {
    let mut mod_line = line.clone();
    mod_line = mod_line.trim().to_string();
    if !mod_line.contains("import ") {
        return None;
    }

    let mut base = PY_REGEX_LIBRARY.deref().as_str();

    let after_mod_import = split_string!(mod_line, "import ", 1).to_string();
    let new_standalone = resolve_standalone_import(after_mod_import);
    let new_base = new_standalone.get(0).unwrap();

    if mod_line.starts_with("from") {
        base = "";
        mod_line = mod_line.trim_start_matches("from ").trim().to_string();
    } else {
        if mod_line.contains(" as ") {
            base = new_base.deref().clone();
        }
    }

    let post_import = split_string!(mod_line, "import ", 1).trim();
    let mut valid_refs: Vec<String> = Vec::new();
    if split_string!(post_import, " ").last().unwrap() == &base {
        for function in SUPPORTED_FUNCTIONS.deref() {
            valid_refs.push(format_base(base.to_string(), function.to_string()));
        }
    } else {
        for import_target in resolve_standalone_import(post_import.to_string()) {
            valid_refs.push(format_base(base.to_string(), import_target))
        }
    }

    Some(valid_refs.into())
}

pub fn extract_imports(contents: String) -> Option<Vec<String>> {
    let mut valid_imports: Vec<String> = Vec::new();
    for line in contents.lines() {
        if PY_IMPORT_PATT.is_match(line) {
            parse_import_line(line.to_string()).and_then(|res| Some(valid_imports.extend(res)));
        }
    }
    return if valid_imports.is_empty() {
        None
    } else {
        Some(valid_imports)
    };
}

pub fn extract_python(contents_raw: String) -> Option<Vec<ExtractionResult>> {
    // Remove all comments before beginning parsing
    let context: &LanguageContext = PY_CONTEXT.borrow().deref();
    let contents = strip_comments(contents_raw, context);
    let valid_imports = extract_imports(contents.clone());
    match valid_imports {
        Some(imports) => {
            let matches = extract_matching_calls(contents.as_str(), context, imports);
            if matches.len() > 0 {
                Some(matches_to_extraction_results(contents, matches))
            } else {
                None
            }
        }
        None => None,
    }
}
