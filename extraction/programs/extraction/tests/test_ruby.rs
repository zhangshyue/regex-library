mod utils;

#[cfg(test)]
pub mod ruby_test {
    use crate::reference_file_path;
    use extraction::extract::ruby::extract_ruby;
    use extraction::extract::ExtractionResult;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    #[test]
    pub fn test_ruby_extraction() {
        let expected_values: HashMap<String, Option<Vec<ExtractionResult>>> = HashMap::from([(
            reference_file_path!("ruby/test_basic.rb"),
            Some(vec![ExtractionResult {
                expression: "test+".to_string(),
                line_number: 2,
            }]),
        )]);
        for (path_str, expected_res) in expected_values {
            let path = Path::new(&path_str);
            assert_eq!(
                extract_ruby(fs::read_to_string(path).unwrap()),
                expected_res
            )
        }
    }
}
