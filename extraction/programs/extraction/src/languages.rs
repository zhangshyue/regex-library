use crate::SupportedLanguage;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::Path;

// Constants that need to be known about a language
// to do the tokenization tasks necessary to get the regexs
pub struct LanguageContext {
    pub single_line_comment_starts: Vec<&'static str>,
    pub multi_line_comment_starts: Vec<(&'static str, &'static str)>,

    pub single_line_string_starts: Vec<&'static str>,
    pub multi_line_string_starts: Vec<&'static str>,

    pub call_open_sym: u8,
    pub call_close_sym: u8,

    pub special_regex_char: Option<u8>,
}

// A string that's unlikely to be used as a single string opener
pub const CHAR_SENTINEL: &str = "!!~~REGEX_LIBRARY_SENTINEL!!~~";

lazy_static! {
    pub static ref LANGUAGE_EXTENSIONS: HashMap<String, SupportedLanguage> = HashMap::from([
        (String::from("py"), SupportedLanguage::Python),
        (String::from("java"), SupportedLanguage::Java),
        (String::from("rs"), SupportedLanguage::Rust),
        (String::from("go"), SupportedLanguage::Golang),
        (String::from("php"), SupportedLanguage::PHP),
        (String::from("rb"), SupportedLanguage::Ruby),
        (String::from("js"), SupportedLanguage::JavaScript),
        (String::from("ts"), SupportedLanguage::JavaScript),
    ]);
}

pub fn is_supported(path: &Path) -> bool {
    match path.extension() {
        Some(e) => LANGUAGE_EXTENSIONS.contains_key(e.to_str().unwrap()),
        None => false,
    }
}
