use byteorder::{BigEndian, ByteOrder};
use num_enum::IntoPrimitive;

#[derive(IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Halt = 0x01,
    Pop = 0x02,

    Add = 0x04,
    Subtract = 0x05,
    Multiply = 0x06,
    Divide = 0x07,
    Exponent = 0x08,

    UnaryNegate = 0x09,
    UnaryNot = 0xA,

    LoadConstant = 0x03,
    LoadString = 0xB,
    LoadNil = 0xC,
    LoadBoolTrue = 0xD,
    LoadBoolFalse = 0xE,

    CompareLess = 0xF,
    CompareLessEqual = 0x10,
    CompareGreater = 0x11,
    CompareGreaterEqual = 0x012,
    CompareEqual = 0x13,
    CompareNotEqual = 0x14,
}

pub struct Definition {
    pub name: &'static str,
    pub operands_width: Vec<usize>,
}

pub type Instructions = Vec<u8>;

impl OpCode {
    pub fn make(opcode: OpCode, operands: Vec<usize>) -> Instructions {
        let definition = OpCode::get_definition(opcode);
        let mut instruction_length = 1; /* 1 for opcode itself */

        for width in definition.operands_width.iter() {
            instruction_length += width;
        }

        let mut instructions: Instructions = vec![0; instruction_length];
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

        instructions
    }

    pub fn get_definition(opcode: OpCode) -> Definition {
        match opcode {
            OpCode::LoadConstant => Definition {
                name: "LOAD_CONSTANT",
                operands_width: vec![2],
            },
            OpCode::LoadString => Definition {
                name: "LOAD_STRING",
                operands_width: vec![2],
            },
            OpCode::LoadNil => Definition {
                name: "LOAD_NIL",
                operands_width: vec![],
            },
            OpCode::LoadBoolTrue => Definition {
                name: "LOAD_BOOL_TRUE",
                operands_width: vec![],
            },
            OpCode::LoadBoolFalse => Definition {
                name: "LOAD_BOOL_FALSE",
                operands_width: vec![],
            },
            OpCode::Halt => Definition {
                name: "HALT",
                operands_width: vec![],
            },
            OpCode::Pop => Definition {
                name: "POP",
                operands_width: vec![],
            },
            OpCode::Add => Definition {
                name: "ADD",
                operands_width: vec![],
            },
            OpCode::Subtract => Definition {
                name: "SUCTRACT",
                operands_width: vec![],
            },
            OpCode::Multiply => Definition {
                name: "MULTIPLY",
                operands_width: vec![],
            },
            OpCode::Divide => Definition {
                name: "DIVIDE",
                operands_width: vec![],
            },
            OpCode::Exponent => Definition {
                name: "EXPONENT",
                operands_width: vec![],
            },
            OpCode::UnaryNegate => Definition {
                name: "UNARY_NEGATE",
                operands_width: vec![],
            },
            OpCode::UnaryNot => Definition {
                name: "UNARY_NOT",
                operands_width: vec![],
            },
            OpCode::CompareLess => Definition {
                name: "COMPARE_LESS",
                operands_width: vec![],
            },
            OpCode::CompareLessEqual => Definition {
                name: "COMPARE_LESS_EQUAL",
                operands_width: vec![],
            },
            OpCode::CompareGreater => Definition {
                name: "COMPARE_GREATER",
                operands_width: vec![],
            },
            OpCode::CompareGreaterEqual => Definition {
                name: "COMPARE_GREATER_EQUAL",
                operands_width: vec![],
            },
            OpCode::CompareEqual => Definition {
                name: "COMPARE_EQUAL",
                operands_width: vec![],
            },
            OpCode::CompareNotEqual => Definition {
                name: "COMPARE_NOT_EQUAL",
                operands_width: vec![],
            },
        }
    }
}

pub trait ToOpcode {
    fn to_opcode(self) -> OpCode;
}

impl ToOpcode for u8 {
    fn to_opcode(self) -> OpCode {
        match self {
            0x01 => OpCode::Halt,
            0x02 => OpCode::Pop,
            0x03 => OpCode::LoadConstant,
            0x04 => OpCode::Add,
            0x05 => OpCode::Subtract,
            0x06 => OpCode::Multiply,
            0x07 => OpCode::Divide,
            0x08 => OpCode::Exponent,
            0x09 => OpCode::UnaryNegate,
            0xA => OpCode::UnaryNot,
            0xB => OpCode::LoadString,
            0xC => OpCode::LoadNil,
            0xD => OpCode::LoadBoolTrue,
            0xE => OpCode::LoadBoolFalse,
            0xF => OpCode::CompareLess,
            0x10 => OpCode::CompareLessEqual,
            0x11 => OpCode::CompareGreater,
            0x12 => OpCode::CompareGreaterEqual,
            0x13 => OpCode::CompareEqual,
            0x14 => OpCode::CompareNotEqual,

            _ => unreachable!("Cannot convert byte '{}' to an opcode", self),
        }
    }
}

pub fn read_uint16(instructions: &Instructions, offset: usize) -> u16 {
    BigEndian::read_u16(&instructions[offset..offset + 2])
}
