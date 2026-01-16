use byteorder::{BigEndian, ReadBytesExt};
use colored::*;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;
use std::{collections::HashMap, process};

use crate::runtime_value::RuntimeValue;
use crate::{
    compiler::{
        compiler::{Bytecode, Compiler},
        disassembler::disassemble,
    },
    hydor_vm::vm::HydorVM,
    lexer::Lexer,
    parser::parser::Parser,
    utils::{self, print_info, print_success, throw_error},
};

type CommandFn = fn(&[String]);

struct Command {
    name: &'static str,
    description: &'static str,
    usage: &'static str,
    args_count: usize,
    function: CommandFn,
}

enum FileType {
    Source,
    Bytecode,
    Unknown,
}

fn get_commands() -> HashMap<&'static str, Command> {
    let mut commands = HashMap::new();

    commands.insert(
        "help",
        Command {
            name: "help",
            description: "Shows this help menu",
            usage: "help",
            args_count: 0,
            function: command_help,
        },
    );

    commands.insert(
        "run",
        Command {
            name: "run",
            description: "Runs a Hydor source file or bytecode",
            usage: "run <file>",
            args_count: 1,
            function: command_run,
        },
    );

    commands.insert(
        "disassemble",
        Command {
            name: "disassemble",
            description: "Disassembles bytecode from a Hydor file",
            usage: "disassemble <file>",
            args_count: 1,
            function: command_disassemble,
        },
    );

    commands.insert(
        "build",
        Command {
            name: "build",
            description: "Builds bytecode from a Hydor file",
            usage: "build <file>",
            args_count: 1,
            function: command_build,
        },
    );

    commands
}

pub fn run(args: Vec<String>) {
    if args.len() < 2 {
        throw_error(
            "Usage: hydor <command> [args...]\nRun 'hydor help' for more info",
            1,
        );
    }

    let commands = get_commands();
    let command_name = &args[1];

    // fetch commands
    let command = match commands.get(command_name.as_str()) {
        Some(cmd) => cmd,
        None => throw_error(
            &format!(
                "Unknown command '{}'. Run 'hydor help' for available commands",
                command_name
            ),
            1,
        ),
    };

    // calculate args
    let provided_args = args.len() - 2;
    if provided_args != command.args_count {
        throw_error(&format!("Usage: hydor {}", command.usage), 1);
    }

    (command.function)(&args[2..]);
}

fn command_help(_args: &[String]) {
    let commands = get_commands();

    println!("{}", "Usage:".bright_white().bold());
    println!(
        "  {} {} {}",
        "hydor".cyan(),
        "<command>".yellow(),
        "[args...]".bright_black()
    );
    println!();
    println!("{}", "Commands:".bright_white().bold());

    let mut cmd_list: Vec<_> = commands.values().collect();
    cmd_list.sort_by_key(|c| c.name);

    for cmd in cmd_list {
        println!(
            "  {:<40} {}",
            format!("hydor {}", cmd.usage).cyan(),
            cmd.description.bright_black()
        );
    }
    println!();
    println!("{}", "Examples:".bright_white().bold());
    println!(
        "  {} {}",
        "hydor run main.hyd".cyan(),
        "# Compile and run source".bright_black()
    );
    println!(
        "  {} {}",
        "hydor build app.hyd".cyan(),
        "# Build to bytecode".bright_black()
    );
    println!(
        "  {} {}",
        "hydor run app.hydc".cyan(),
        "# Run precompiled bytecode".bright_black()
    );
}

fn command_run(args: &[String]) {
    /* NOTE: This abomination is not permanent
     * Lets just stick with this until functions came into play
     * and we can finally refactor this
     * */
    let path = &args[0];

    let source = utils::read_file(path.to_string());
    let bytecode = match detect_file_type(path) {
        FileType::Bytecode => {
            print_info(&format!("Loading bytecode from '{}'", path));
            match Bytecode::load_from_file(path) {
                Ok(bc) => bc,
                Err(err) => throw_error(&format!("Failed to load bytecode: {}", err), 1),
            }
        }
        FileType::Source => {
            print_info(&format!("Compiling '{}'", path));
            compile_source(&source)
        }
        FileType::Unknown => throw_error(
            "File is neither valid Hydor source (.hyd) nor bytecode (.hydc)",
            1,
        ),
    };

    print_success("Execution started");
    let mut vm = HydorVM::new(bytecode);

    let start = Instant::now();
    let result = vm.execute_bytecode();
    let end = start.elapsed();

    match result {
        Ok(()) => match vm.last_popped() {
            Some(e) => match e {
                RuntimeValue::StringLiteral(s) => {
                    let val = HydorVM::resolve_string(&vm, s);
                    println!("Last popped: String({val})")
                }
                _ => {
                    println!("Last popped: {e:?}");
                    print_info(&format!("Program took {:?}", end));
                    process::exit(0)
                }
            },
            None => {
                print_info("No last element found");
                print_info(&format!("Program took {:?}", end));
                process::exit(0);
            }
        },
        Err(e) => e.report(&source),
    }

    print_info(&format!("Program took {:?}", end));
}

fn detect_file_type(path: &str) -> FileType {
    // Try to read the first 4 bytes (magic number)
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return FileType::Unknown,
    };

    // Check for magic number
    match file.read_u32::<BigEndian>() {
        Ok(magic) if magic == 0x48594452 => FileType::Bytecode, // "HYDR"
        _ => {
            // Not bytecode, assume it's source code
            // Could do additional validation here (check if it's valid UTF-8, etc.)
            FileType::Source
        }
    }
}

fn command_disassemble(args: &[String]) {
    let path = &args[0];

    match detect_file_type(path) {
        FileType::Bytecode => {
            print_info(&format!("Loading bytecode from '{}'", path));
            let bytecode = match Bytecode::load_from_file(path) {
                Ok(bc) => bc,
                Err(err) => throw_error(&format!("Failed to load bytecode: {}", err), 1),
            };
            println!();
            disassemble(&bytecode);
        }
        FileType::Source => {
            print_info(&format!("Compiling '{}'", path));
            let source = utils::read_file(path.to_string());
            let bytecode = compile_source(&source);
            println!();
            disassemble(&bytecode);
        }
        FileType::Unknown => throw_error(
            "File is neither valid Hydor source (.hyd) nor bytecode (.hydc)",
            1,
        ),
    }
}

fn command_build(args: &[String]) {
    let input_path = &args[0];

    print_info(&format!("Compiling '{}'", input_path));
    let source = utils::read_file(input_path.to_string());
    let bytecode = compile_source(&source);

    // Generate output path: replace extension with .hydc
    let mut output_path = PathBuf::from(input_path);
    output_path.set_extension("hydc");

    match bytecode.save_to_file(&output_path) {
        Ok(()) => {
            println!();
            print_success(&format!(
                "Built '{}' → '{}'",
                input_path.bright_white(),
                output_path.display().to_string().bright_white()
            ));
        }
        Err(err) => throw_error(&format!("Cannot save to file: {}", err), 1),
    }
}

fn compile_source(source: &str) -> Bytecode {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(errors) => {
            errors.report_all(source);
            std::process::exit(1);
        }
    };

    let mut compiler = Compiler::new();
    match compiler.compile_program(program) {
        Ok(bytecode) => bytecode,
        Err(errors) => {
            errors.report_all(source);
            std::process::exit(1);
        }
    }
}
