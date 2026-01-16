use crate::{hydor_vm::vm::HydorVM, runtime_value::RuntimeValue};

impl HydorVM {
    pub fn is_truthy(&self, rv: RuntimeValue) -> bool {
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
