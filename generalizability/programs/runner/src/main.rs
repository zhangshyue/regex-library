use generalizability::generalizability;
use generalizability::root::*;
use protobuf::Message;
use std::env;

fn main() {
    let args = env::args();
    if args.len() >= 1 {
        let last_arg = args.last().unwrap();
        let expr_raw = base64::decode(last_arg.clone())
            .expect("Input should be a b64 encoded Expression object");
        // Parse the protobuf command line input into an Expression object
        let expr = Root::parse_from_bytes(expr_raw.as_ref())
            .expect(format!("Couldn't parse message from input {}", last_arg).as_str());
        let gen_res = generalizability(expr.expression.unwrap());
        match gen_res {
            Ok(o) => println!("{}", base64::encode(o.write_to_bytes().unwrap())),
            Err(e) => eprintln!("{}", e),
        };
    } else {
        eprintln!("No expression supplied");
    }
}
