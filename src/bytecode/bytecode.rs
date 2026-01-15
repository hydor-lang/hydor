use byteorder::{BigEndian, ByteOrder};
use num_enum::IntoPrimitive;

#[derive(IntoPrimitive, Clone, Copy)]
#[repr(u8)]
enum OpCode {
    Halt = 0x01,
    Pop = 0x02,
    LoadConstant = 0x03,
}

struct Definition {
    name: &'static str,
    operands_width: Vec<usize>,
}

impl OpCode {
    pub fn make(opcode: OpCode, operands: Vec<i32>) {
        let definition = OpCode::get_definition(opcode);
        let mut instruction_length: usize = 1; /* 1 for opcode itself */

        for width in definition.operands_width.iter() {
            instruction_length += width;
        }

        let mut instructions: Vec<u8> = vec![0; instruction_length];
        instructions[0] = opcode.into();

        let mut offset = 1;
        for (i, operand) in operands.iter().enumerate() {
            let width = definition.operands_width[i];

            match width {
                2 => BigEndian::write_i16(&mut instructions[offset..], *operand as i16),

                _ => unreachable!(
                    "Cannot make new instruction operand with operand width of {width}"
                ),
            }

            offset += width;
        }
    }

    fn get_definition(opcode: OpCode) -> Definition {
        match opcode {
            OpCode::LoadConstant => Definition {
                name: "LOAD_CONSTANT",
                operands_width: vec![2],
            },
            OpCode::Halt => Definition {
                name: "HALT",
                operands_width: vec![],
            },
            OpCode::Pop => Definition {
                name: "POP",
                operands_width: vec![],
            },
        }
    }
}

trait ToOpcode {
    fn to_opcode(self) -> OpCode;
}

impl ToOpcode for u8 {
    fn to_opcode(self) -> OpCode {
        match self {
            0x01 => OpCode::Halt,
            0x02 => OpCode::Pop,
            0x03 => OpCode::LoadConstant,

            _ => unreachable!("Cannot convert byte '{}' to an opcode", self),
        }
    }
}
