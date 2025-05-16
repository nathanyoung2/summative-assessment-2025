use crate::Context;
use crate::parser::{Argument, SyntaxError};
use std::fmt::Debug;
use std::rc::Rc;
mod cd_command;
mod ls_command;
mod touch_command;
mod mkdir_command;
mod rm_command;
mod rmdir_command;

#[derive(Debug, Eq, PartialEq, Clone)]
/// Represents a type of command
pub enum CommandType {
    Cd,
    Ls,
    Touch,
    Mkdir,
    Rm,
    Rmdir,
}

/// CommandBuilder is used for building a command.
pub struct CommandBuilder {
    /// The type of the command to build.
    command_type: CommandType,

    /// The arguments supplied into the command.
    arguments: Vec<Argument>,
}

impl CommandBuilder {
    /// Create a new CommandBuilder instance with a provided command type.
    pub fn new(command_type: CommandType) -> Self {
        Self {
            command_type,
            arguments: Vec::new(),
        }
    }

    /// Add an argument to the command
    pub fn add_argument(&mut self, arg: Argument) {
        self.arguments.push(arg);
    }

    /// Build the final command. Uses the arguments previously provided with
    /// the add_argument associated function.
    pub fn build(&self) -> Result<Box<dyn Command>, SyntaxError> {
        match self.command_type {
            CommandType::Cd => Ok(Box::new(cd_command::CdCmd::build(&self.arguments)?)),
            CommandType::Ls => Ok(Box::new(ls_command::LsCmd::build(&self.arguments)?)),
            CommandType::Touch => Ok(Box::new(touch_command::TouchCmd::build(&self.arguments)?)),
            CommandType::Mkdir => Ok(Box::new(mkdir_command::MkdirCmd::build(&self.arguments)?)),
            CommandType::Rm => Ok(Box::new(rm_command::RmCmd::build(&self.arguments)?)),
            CommandType::Rmdir => Ok(Box::new(rmdir_command::RmdirCmd::build(&self.arguments)?)),
        }
    }
}

/// Represents any command.
pub trait Command : Debug {
    fn build(arguments: &[Argument]) -> Result<Self, SyntaxError> where Self: Sized;
    fn execute(&self, ctx: Rc<Context>);
}
