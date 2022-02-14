mod utils;

#[cfg(test)]
pub mod php_test {
    use crate::reference_file_path;
    use extraction::extract::php::extract_php;
    use extraction::extract::ExtractionResult;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    #[test]
    pub fn test_php_extraction() {
        let expected_values: HashMap<String, Option<Vec<ExtractionResult>>> = HashMap::from([(
            reference_file_path!("php/test_basic.php"),
            Some(vec![ExtractionResult {
                expression: "/\\d+/".to_string(),
                line_number: 3,
            }]),
        )]);
        for (path_str, expected_res) in expected_values {
            let path = Path::new(&path_str);
            assert_eq!(extract_php(fs::read_to_string(path).unwrap()), expected_res)
        }
    }
}
