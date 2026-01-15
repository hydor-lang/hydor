use hydor::{run::run::run_file, utils};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        utils::throw_error("Usage: hydor run <path>", 0);
    }

    let command = &args[1];
    let path = &args[2];

    match command.as_str() {
        "run" => {
            let content = utils::read_file(path.clone());
            run_file(content);
        }
        _ => {
            utils::throw_error("Unknown command. Use: hydor run <path>", 1);
        }
    }
}
