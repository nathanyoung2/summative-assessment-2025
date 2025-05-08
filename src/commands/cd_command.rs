use crate::Context;
use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct CdCmd {
    ctx: String,
}

impl super::Command for CdCmd {
    fn required_arg_count(&self) -> usize {
        1
    }

    fn execute(&self, ctx: &mut Context) {}
}

pub fn get() -> CdCmd {
    CdCmd::default()
}
