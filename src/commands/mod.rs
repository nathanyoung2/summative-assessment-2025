use crate::Context;
use crate::parser::{NodePath, NodePathSegment, SyntaxError};
use std::fmt::Debug;
mod cd_command;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CommandType {
    Cd,
    Ls,
    Mv,
    Touch,
}

pub struct CommandBuilder {
    command_type: CommandType,
    arguments: Vec<NodePath>,
}

impl CommandBuilder {
    pub fn new(command_type: CommandType) -> Self {
        Self {
            command_type,
            arguments: Vec::new(),
        }
    }

    pub fn add_argument(&mut self, arg: NodePath) {
        self.arguments.push(arg);
    }

    pub fn build(&self) -> Result<Box<dyn Command>, SyntaxError> {
        match self.command_type {
            CommandType::Cd => Ok(Box::new(cd_command::CdCmd::build(&self.arguments)?)),
            _ => Ok(Box::new(cd_command::CdCmd::build(&self.arguments)?)),
        }
    }
}

pub trait Command : Debug {
    fn build(arguments: &[NodePath]) -> Result<Self, SyntaxError> where Self: Sized;
    fn execute(&self, ctx: Context);
}
