use crate::Context;
use std::fmt::Debug;
mod cd_command;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CommandType {
    Cd,
    Ls,
    Mv,
    Touch,
}

pub trait Command : Debug {
    fn required_arg_count(&self) -> usize;
    fn execute(&self, ctx: &mut Context);
}

pub fn get_command(command_type: CommandType) -> Option<Box<dyn Command>> {
    match command_type {
        CommandType::Cd => Some(Box::new(cd_command::get())),
        _ => None,
    }
}
