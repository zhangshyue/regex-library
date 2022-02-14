use crate::extract::python::extract_python;
use crate::java::extract_java;
use crate::php::extract_php;
use crate::ruby::extract_ruby;
use crate::rust::extract_rust;
use crate::{extract_go, extract_js, FoundExpresssion, SupportedLanguage, LANGUAGE_EXTENSIONS};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractionResult {
    pub expression: String,
    pub line_number: i32,
}

pub fn extract(file: &Path) -> Result<Option<Vec<FoundExpresssion>>, &'static str> {
    if !file.is_file() {
        return Err("Can only extract from file");
    }

    let language: SupportedLanguage;

    match file.extension() {
        Some(e) if LANGUAGE_EXTENSIONS.contains_key(e.to_str().unwrap()) => {
            language = LANGUAGE_EXTENSIONS[e.to_str().unwrap()];
        }
        _ => return Ok(None),
    }
    let file_contents = fs::read_to_string(file).unwrap();

    let res: Option<Vec<ExtractionResult>>;

    match language {
        SupportedLanguage::Python => res = extract_python(file_contents),
        SupportedLanguage::JavaScript => res = extract_js(file_contents),
        SupportedLanguage::Golang => res = extract_go(file_contents),
        SupportedLanguage::Java => res = extract_java(file_contents),
        SupportedLanguage::Rust => res = extract_rust(file_contents),
        SupportedLanguage::PHP => res = extract_php(file_contents),
        SupportedLanguage::Ruby => res = extract_ruby(file_contents),
    }

    match res {
        Some(extract_res) => {
            let mut found_expressions: Vec<FoundExpresssion> = Vec::new();
            for expression in extract_res {
                let mut found = FoundExpresssion::new();
                found.set_expression(expression.expression);
                found.set_line_number(expression.line_number);
                found.set_file(file.to_str().unwrap().to_string());
                found.set_language(language);
                found_expressions.push(found);
            }
            Ok(Some(found_expressions))
        }
        None => Ok(None),
    }
}
