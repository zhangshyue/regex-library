mod utils;

#[cfg(test)]
pub mod python_test {
    use crate::reference_file_path;
    use extraction::extract::{
        extract_imports, extract_python, parse_import_line, ExtractionResult, PY_CONTEXT,
        SUPPORTED_FUNCTIONS,
    };
    use extraction::languages::LanguageContext;
    use extraction::utils::parse_utils::strip_comments;
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use std::fs;
    use std::ops::Deref;
    use std::path::Path;

    fn map_supported_functions(base: String) -> Vec<String> {
        SUPPORTED_FUNCTIONS
            .deref()
            .iter()
            .map(|x| format!("{}.{}", base, x))
            .collect::<Vec<String>>()
    }

    #[test]
    pub fn test_find_import_variant() {
        let correct_import_variants: HashMap<String, Option<Vec<String>>> = HashMap::from([
            (
                String::from("import re"),
                Some(map_supported_functions("re".to_string())),
            ),
            (
                String::from("from re import compile, sub as oth"),
                Some(vec!["compile".to_string(), "oth".to_string()]),
            ),
            (
                String::from("import re as x"),
                Some(map_supported_functions("x".to_string())),
            ),
        ]);

        for (import_variant, expected) in correct_import_variants {
            assert_eq!(parse_import_line(import_variant), expected);
        }
    }

    #[test]
    pub fn test_parse_python_file_import() {
        let path_str = reference_file_path!("python/test_standard_usage.py");
        let python_file = Path::new(&path_str);
        assert_eq!(
            extract_imports(fs::read_to_string(python_file).unwrap()),
            Some(map_supported_functions("re".to_string()))
        );
    }

    #[test]
    pub fn test_python_comment_removal() {
        let path_str = reference_file_path!("python/test_comment_removal.py");
        let python_comment_file = Path::new(&path_str);
        let context: &LanguageContext = PY_CONTEXT.borrow().deref();
        let no_comments = strip_comments(
            fs::read_to_string(python_comment_file.clone()).unwrap(),
            context,
        );
        assert_eq!(no_comments, String::from("print('Hello, world!')\n\n"))
    }

    #[test]
    pub fn test_python_extraction() {
        let expected_values: HashMap<String, Option<Vec<ExtractionResult>>> =
            HashMap::from([(
                reference_file_path!("python/test_standard_usage.py"),
                Some(vec![ExtractionResult {
                    expression: "{.*}".to_string(),
                    line_number: 4,
                }])),
                // This references a huge file I pulled from the bs4 library, with lots of
                // regular expressions
                (reference_file_path!("python/test_complex_example.py"),
                    Some(vec![
                        ExtractionResult {
                            expression: "\\s+".to_string(), line_number: 15
                        },
                        ExtractionResult {
                            expression: "((^|;)\\s*charset=)([^;]*)".to_string(), line_number: 72
                        }, ExtractionResult {
                            expression: "^[a-zA-Z0-9][-.a-zA-Z0-9:_]*$".to_string(), line_number: 602
                        }, ExtractionResult {
                            expression: "^(?P<tag>[a-zA-Z0-9][-.a-zA-Z0-9:_]*)?\\[(?P<attribute>[\\w-]+)(?P<operator>[=~\\|\\^\\$\\*]?)".to_string(),
                            line_number: 611
                        }, ExtractionResult {
                            expression: "\"[^\"]*:[^\"]*\"".to_string(),
                            line_number: 1337
                        }, ExtractionResult {
                            expression: "([a-zA-Z\\d-]+)\\(([a-zA-Z\\d]+)\\)".to_string(), line_number: 1421
                        },
                      ])),
                (reference_file_path!("python/test_additional.py"),
                 Some(vec![
                    ExtractionResult {
                         expression: "\\/\\?.*tls=true".to_string(),
                         line_number: 24
                    },
                    ExtractionResult {
                        expression: "\\/\\?.*tls=false".to_string(),
                        line_number: 27
                    },
                    ExtractionResult {
                        expression: ".*\\.mongodb.net".to_string(),
                        line_number: 30
                    }])),
                (reference_file_path!("python/test_rerun.py"), None)
            ]);
        for (path_str, expected_res) in expected_values {
            let path = Path::new(&path_str);
            assert_eq!(
                extract_python(fs::read_to_string(path).unwrap()),
                expected_res
            )
        }
    }
}
