use byteorder::{BigEndian, ByteOrder};
use num_enum::IntoPrimitive;

#[derive(IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Halt = 0x01,
    Pop = 0x02,

    // Integer arithmetic
    AddInt = 0x04,
    SubtractInt = 0x05,
    MultiplyInt = 0x06,
    DivideInt = 0x07,
    ExponentInt = 0x08,

    // Float arithmetic
    AddFloat = 0x09,
    SubtractFloat = 0x0A,
    MultiplyFloat = 0x0B,
    DivideFloat = 0x0C,
    ExponentFloat = 0x0D,

    // String operations
    ConcatString = 0x0E,

    // Unary operations
    UnaryNegateInt = 0x0F,
    UnaryNegateFloat = 0x10,
    UnaryNot = 0x11,

    // Load operations
    LoadConstant = 0x03,
    LoadString = 0x12,
    LoadNil = 0x13,
    LoadBoolTrue = 0x14,
    LoadBoolFalse = 0x15,

    // Integer comparisons
    CompareLessInt = 0x16,
    CompareLessEqualInt = 0x17,
    CompareGreaterInt = 0x18,
    CompareGreaterEqualInt = 0x19,

    // Float comparisons
    CompareLessFloat = 0x1A,
    CompareLessEqualFloat = 0x1B,
    CompareGreaterFloat = 0x1C,
    CompareGreaterEqualFloat = 0x1D,

    // General equality (works on any type)
    CompareEqual = 0x1E,
    CompareNotEqual = 0x1F,
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

            // Integer arithmetic
            OpCode::AddInt => Definition {
                name: "ADD_INT",
                operands_width: vec![],
            },
            OpCode::SubtractInt => Definition {
                name: "SUBTRACT_INT",
                operands_width: vec![],
            },
            OpCode::MultiplyInt => Definition {
                name: "MULTIPLY_INT",
                operands_width: vec![],
            },
            OpCode::DivideInt => Definition {
                name: "DIVIDE_INT",
                operands_width: vec![],
            },
            OpCode::ExponentInt => Definition {
                name: "EXPONENT_INT",
                operands_width: vec![],
            },

            // Float arithmetic
            OpCode::AddFloat => Definition {
                name: "ADD_FLOAT",
                operands_width: vec![],
            },
            OpCode::SubtractFloat => Definition {
                name: "SUBTRACT_FLOAT",
                operands_width: vec![],
            },
            OpCode::MultiplyFloat => Definition {
                name: "MULTIPLY_FLOAT",
                operands_width: vec![],
            },
            OpCode::DivideFloat => Definition {
                name: "DIVIDE_FLOAT",
                operands_width: vec![],
            },
            OpCode::ExponentFloat => Definition {
                name: "EXPONENT_FLOAT",
                operands_width: vec![],
            },

            // String operations
            OpCode::ConcatString => Definition {
                name: "CONCAT_STRING",
                operands_width: vec![],
            },

            // Unary operations
            OpCode::UnaryNegateInt => Definition {
                name: "UNARY_NEGATE_INT",
                operands_width: vec![],
            },
            OpCode::UnaryNegateFloat => Definition {
                name: "UNARY_NEGATE_FLOAT",
                operands_width: vec![],
            },
            OpCode::UnaryNot => Definition {
                name: "UNARY_NOT",
                operands_width: vec![],
            },

            // Integer comparisons
            OpCode::CompareLessInt => Definition {
                name: "COMPARE_LESS_INT",
                operands_width: vec![],
            },
            OpCode::CompareLessEqualInt => Definition {
                name: "COMPARE_LESS_EQUAL_INT",
                operands_width: vec![],
            },
            OpCode::CompareGreaterInt => Definition {
                name: "COMPARE_GREATER_INT",
                operands_width: vec![],
            },
            OpCode::CompareGreaterEqualInt => Definition {
                name: "COMPARE_GREATER_EQUAL_INT",
                operands_width: vec![],
            },

            // Float comparisons
            OpCode::CompareLessFloat => Definition {
                name: "COMPARE_LESS_FLOAT",
                operands_width: vec![],
            },
            OpCode::CompareLessEqualFloat => Definition {
                name: "COMPARE_LESS_EQUAL_FLOAT",
                operands_width: vec![],
            },
            OpCode::CompareGreaterFloat => Definition {
                name: "COMPARE_GREATER_FLOAT",
                operands_width: vec![],
            },
            OpCode::CompareGreaterEqualFloat => Definition {
                name: "COMPARE_GREATER_EQUAL_FLOAT",
                operands_width: vec![],
            },

            // General equality
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

            // Integer arithmetic
            0x04 => OpCode::AddInt,
            0x05 => OpCode::SubtractInt,
            0x06 => OpCode::MultiplyInt,
            0x07 => OpCode::DivideInt,
            0x08 => OpCode::ExponentInt,

            // Float arithmetic
            0x09 => OpCode::AddFloat,
            0x0A => OpCode::SubtractFloat,
            0x0B => OpCode::MultiplyFloat,
            0x0C => OpCode::DivideFloat,
            0x0D => OpCode::ExponentFloat,

            // String operations
            0x0E => OpCode::ConcatString,

            // Unary operations
            0x0F => OpCode::UnaryNegateInt,
            0x10 => OpCode::UnaryNegateFloat,
            0x11 => OpCode::UnaryNot,

            // Load operations
            0x12 => OpCode::LoadString,
            0x13 => OpCode::LoadNil,
            0x14 => OpCode::LoadBoolTrue,
            0x15 => OpCode::LoadBoolFalse,

            // Integer comparisons
            0x16 => OpCode::CompareLessInt,
            0x17 => OpCode::CompareLessEqualInt,
            0x18 => OpCode::CompareGreaterInt,
            0x19 => OpCode::CompareGreaterEqualInt,

            // Float comparisons
            0x1A => OpCode::CompareLessFloat,
            0x1B => OpCode::CompareLessEqualFloat,
            0x1C => OpCode::CompareGreaterFloat,
            0x1D => OpCode::CompareGreaterEqualFloat,

            // General equality
            0x1E => OpCode::CompareEqual,
            0x1F => OpCode::CompareNotEqual,

            _ => unreachable!("Cannot convert byte '{}' to an opcode", self),
        }
    }
}

pub fn read_uint16(instructions: &Instructions, offset: usize) -> u16 {
    BigEndian::read_u16(&instructions[offset..offset + 2])
}
