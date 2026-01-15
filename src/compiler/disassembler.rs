use colored::*;

use crate::{
    bytecode::bytecode::{Instructions, OpCode, ToOpcode, read_uint16},
    compiler::compiler::Bytecode,
    runtime_value::RuntimeValue,
};

pub fn disassemble(bytecode: &Bytecode) {
    println!("{}", "--== Hydor Assembly ==--".bright_yellow().bold());
    disassemble_instructions(&bytecode.instructions, &bytecode.debug_info);
    println!();
    disassemble_constants(&bytecode.constants);
}

fn disassemble_instructions(
    instructions: &Instructions,
    debug_info: &crate::compiler::compiler::DebugInfo,
) {
    let mut offset = 0;

    while offset < instructions.len() {
        let opcode_byte = instructions[offset];
        let opcode = opcode_byte.to_opcode();
        let definition = OpCode::get_definition(opcode);

        // Get span for this instruction
        let span = debug_info.get_span(offset);

        print!(
            "{} {} {} {}",
            format!("{:04}", offset).cyan(),
            format!("{}:{}-{}", span.line, span.start_column, span.end_column).bright_black(), // or .yellow().dimmed() for more visibility
            definition.name.bright_white(),
            format!("({:#04x})", opcode_byte).cyan()
        );

        offset += 1;

        if !definition.operands_width.is_empty() {
            print!(" {}", "[".white().dimmed());

            for (i, &width) in definition.operands_width.iter().enumerate() {
                if i > 0 {
                    print!("{}", ", ".white().dimmed());
                }

                match width {
                    2 => {
                        let operand = read_uint16(instructions, offset);
                        print!("{}", format!("{:#04x}", operand).white());
                        offset += 2;
                    }
                    _ => unreachable!(),
                }
            }

            print!("{}", "]".white().dimmed());
        }

        println!();
    }
}

fn disassemble_constants(constants: &Vec<RuntimeValue>) {
    println!("{}", "--== Constants ==--".bright_yellow().bold());

    for (i, constant) in constants.iter().enumerate() {
        println!(
            "{} {}",
            format!("{:#04x}", i).cyan(),
            format!("{:?}", constant).bright_white()
        );
    }
}
