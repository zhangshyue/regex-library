mod utils;

#[cfg(test)]
pub mod java_test {
    use crate::reference_file_path;
    use extraction::extract::java::extract_java;
    use extraction::extract::ExtractionResult;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    #[test]
    pub fn test_java_extraction() {
        let expected_values: HashMap<String, Option<Vec<ExtractionResult>>> = HashMap::from([(
            reference_file_path!("java/test_basic.java"),
            Some(vec![ExtractionResult {
                expression: "\\\\d+".to_string(),
                line_number: 6,
            }]),
        )]);
        for (path_str, expected_res) in expected_values {
            let path = Path::new(&path_str);
            assert_eq!(
                extract_java(fs::read_to_string(path).unwrap()),
                expected_res
            )
        }
    }
}
