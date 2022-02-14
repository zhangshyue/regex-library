mod utils;

#[cfg(test)]
pub mod js_test {
    use crate::reference_file_path;
    use extraction::extract::{extract_js, ExtractionResult, JS_CONTEXT};
    use extraction::languages::LanguageContext;
    use extraction::utils::parse_utils::{replace_regex_char, strip_comments};
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use std::fs;
    use std::ops::Deref;
    use std::path::Path;

    #[test]
    pub fn test_js_comment_removal() {
        let path_str = reference_file_path!("js/test_comment_removal.ts");
        let js_comment_file = Path::new(&path_str);
        let context: &LanguageContext = JS_CONTEXT.borrow().deref();
        let no_comments = strip_comments(
            fs::read_to_string(js_comment_file.clone()).unwrap(),
            context,
        );
        assert_eq!(
            no_comments,
            String::from("\n\n\nconsole.log(\"Hello, world!\");\n\n")
        )
    }

    #[test]
    pub fn test_js_special_char_replacement() {
        let path_str = reference_file_path!("js/test_special_char.js");
        let js_char_file = Path::new(&path_str);
        let context: &LanguageContext = JS_CONTEXT.borrow().deref();
        let chars_replaced = replace_regex_char(
            strip_comments(fs::read_to_string(js_char_file).unwrap(), context),
            context,
            "new RegExp",
        );
        assert_eq!(
            chars_replaced,
            "let x = new RegExp('(.*)');\nlet y = new RegExp(\"\\w+\");\ny.test(\"hello\");\n"
        )
    }

    #[test]
    pub fn test_js_extraction() {
        let expected_values: HashMap<String, Option<Vec<ExtractionResult>>> = HashMap::from([
            (
                reference_file_path!("js/test_special_char.js"),
                Some(vec![
                    ExtractionResult {
                        expression: "(.*)".to_string(),
                        line_number: 1,
                    },
                    ExtractionResult {
                        expression: "\\w+".to_string(),
                        line_number: 2,
                    },
                ]),
            ),
            (
                reference_file_path!("js/test_complex_example.js"),
                Some(vec![
                    ExtractionResult {
                        expression: "\\\\s+".to_string(),
                        line_number: 116,
                    },
                    ExtractionResult {
                        expression: "+".to_string(),
                        line_number: 117,
                    },
                    ExtractionResult {
                        expression: "\\\\(.*?\\\\)+|\\\\[.*?\\\\]+|\\\\S+".to_string(),
                        line_number: 356,
                    },
                    ExtractionResult {
                        expression: "[".to_string(),
                        line_number: 749,
                    },
                ]),
            ),
            (
                reference_file_path!("js/controller.bar.tests.js"),
                None
            ),
            (
                reference_file_path!("js/core.scale.js"),
                None
            ),
            (
                reference_file_path!("js/plugin.tooltip.js"),
                None
            ),
            (
                reference_file_path!("js/res.attachment.js"),
                None
            )
        ]);

        for (path_str, expected_res) in expected_values {
            let path = Path::new(&path_str);
            assert_eq!(extract_js(fs::read_to_string(path).unwrap()), expected_res)
        }
    }
}
