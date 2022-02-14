mod utils;

#[cfg(test)]
pub mod go_test {
    use crate::reference_file_path;
    use extraction::extract::{extract_go, program_imports_regexp, ExtractionResult};
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    #[test]
    pub fn test_go_import_check() {
        let has_import = reference_file_path!("go/test_basic.go");
        let not_has_import = reference_file_path!("go/test_no_regex.go");
        let has_import_file = fs::read_to_string(Path::new(&has_import)).unwrap();
        let not_has_import_file = fs::read_to_string(Path::new(&not_has_import)).unwrap();

        assert!(program_imports_regexp(has_import_file));
        assert!(!program_imports_regexp(not_has_import_file));
    }

    #[test]
    pub fn test_go_extraction() {
        let expected_values: HashMap<String, Option<Vec<ExtractionResult>>> = HashMap::from([(
            reference_file_path!("go/test_basic.go"),
            Some(vec![
                ExtractionResult {
                    expression: "foo.*".to_string(),
                    line_number: 7,
                },
                ExtractionResult {
                    expression: "bar.*".to_string(),
                    line_number: 9,
                },
                ExtractionResult {
                    expression: "a(b".to_string(),
                    line_number: 11,
                },
            ]),
        )]);

        for (path_str, expected_res) in expected_values {
            let path = Path::new(&path_str);
            assert_eq!(extract_go(fs::read_to_string(path).unwrap()), expected_res)
        }
    }
}
