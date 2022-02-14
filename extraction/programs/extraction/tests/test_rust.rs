mod utils;

#[cfg(test)]
pub mod rust_test {
    use crate::reference_file_path;
    use extraction::extract::rust::extract_rust;
    use extraction::extract::ExtractionResult;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    #[test]
    pub fn test_rust_extraction() {
        let expected_values: HashMap<String, Option<Vec<ExtractionResult>>> = HashMap::from([(
            reference_file_path!("rust/test_basic.rs"),
            Some(vec![ExtractionResult {
                expression: "\\d+".to_string(),
                line_number: 3,
            }]),
        )]);
        for (path_str, expected_res) in expected_values {
            let path = Path::new(&path_str);
            assert_eq!(
                extract_rust(fs::read_to_string(path).unwrap()),
                expected_res
            )
        }
    }
}
