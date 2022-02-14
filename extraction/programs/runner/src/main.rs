use extraction::extract_path;
use protobuf::Message;
use std::env::args;
use std::path::Path;

fn main() {
    let args = args();
    if args.len() > 1 {
        let entrypoint = args.last().unwrap();
        let expressions_res = extract_path(&Path::new(entrypoint.as_str()));

        match expressions_res {
            Ok(found) => {
                for expr_bytes in found.iter().map(|f| f.write_to_bytes().unwrap()) {
                    println!("{}", base64::encode(expr_bytes));
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        };
    } else {
        eprintln!(
            "Must supply as an argument a directory or file in which to find regular expressions"
        );
    }
}
