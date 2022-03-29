use anyhow::Result;
use crate::compiler::ByteCode;

struct Vm {
    state: State
}

struct State {}

pub fn run(code: ByteCode) -> Result<()> {
    Ok(())
}
