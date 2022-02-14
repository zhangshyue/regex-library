pub mod extract;
pub mod languages;
pub mod macros;
pub mod root;
pub mod utils;

use extract::*;
use languages::*;
use root::*;
use std::path::Path;
use walkdir::WalkDir;

pub fn extract_path(path: &Path) -> Result<Vec<FoundExpresssion>, &'static str> {
    let mut expressions: Vec<FoundExpresssion> = Vec::new();
    let walk_dir = WalkDir::new(path);
    for file in walk_dir {
        match file {
            Ok(valid_file) if valid_file.path().is_file() => {
                if let Some(e) = extract(valid_file.path())? {
                    expressions.append(&mut e.clone());
                }
            }
            _ => {}
        }
    }

    Ok(expressions)
}
