use crate::{Expression, Object, Program, ProgramState, RemapError, Value};

#[derive(Debug, Default)]
pub struct Runtime {
    state: ProgramState,
}

impl Runtime {
    pub fn new(state: ProgramState) -> Self {
        Self { state }
    }

    /// Given the provided [`Object`], run the provided [`Program`] to
    /// completion.
    pub fn execute(
        &mut self,
        object: &mut impl Object,
        program: &Program,
    ) -> Result<Option<Value>, RemapError> {
        let mut values = program
            .expressions
            .iter()
            .map(|expression| expression.execute(&mut self.state, object))
            .collect::<crate::Result<Vec<Option<Value>>>>()
            .map_err(RemapError)?;

        Ok(values.pop().flatten())
    }
}
