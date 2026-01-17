use crate::{
    bytecode::bytecode::read_uint16, errors::HydorError, hydor_vm::vm::HydorVM,
    runtime_value::RuntimeValue, utils::Span,
};

impl HydorVM {
    pub(crate) fn load_constant(&mut self, span: Span) -> Result<(), HydorError> {
        let const_index = read_uint16(&self.instructions, self.ip + 1);
        self.ip += 2;

        let constant = self.constants[const_index as usize];
        self.push(constant, span)?;

        Ok(())
    }

    pub(crate) fn load_string(&mut self, span: Span) -> Result<(), HydorError> {
        let str_index = read_uint16(&self.instructions, self.ip + 1);
        self.ip += 2;

        self.push(RuntimeValue::StringLiteral(str_index as usize), span)?;

        Ok(())
    }

    /// Intern a string into the string table (with deduplication)
    pub(crate) fn intern_string(&mut self, s: String) -> usize {
        // Check if string already exists
        if let Some(pos) = self.string_table.iter().position(|existing| existing == &s) {
            return pos;
        }

        // Add new string
        self.string_table.push(s);
        self.string_table.len() - 1
    }
}
