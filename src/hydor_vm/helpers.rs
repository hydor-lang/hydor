use crate::{bytecode::bytecode::OpCode, hydor_vm::vm::HydorVM, runtime_value::RuntimeValue};

impl HydorVM {
    pub(crate) fn is_truthy(&self, rv: RuntimeValue) -> bool {
        match rv {
            RuntimeValue::BooleanLiteral(b) => b,
            RuntimeValue::IntegerLiteral(n) => n != 0,
            RuntimeValue::FloatLiteral(n) => n != 0.0,
            RuntimeValue::NilLiteral => false,
            RuntimeValue::StringLiteral(idx) => {
                let content = self.resolve_string(idx);

                content != ""
            }

            _ => true,
        }
    }
}

pub(crate) fn opcode_to_operator(opcode: OpCode) -> String {
    match opcode {
        OpCode::CompareLess => "<",
        OpCode::CompareLessEqual => "<=",
        OpCode::CompareGreater => ">",
        OpCode::CompareGreaterEqual => ">=",
        OpCode::CompareEqual => "==",
        OpCode::CompareNotEqual => "!=",
        _ => "?",
    }
    .to_string()
}
