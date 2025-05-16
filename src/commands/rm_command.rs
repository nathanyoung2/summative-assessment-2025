use crate::Context;
use std::fmt::Debug;
use std::rc::Rc;
use crate::parser::{SyntaxError, Argument, NodePath, NodePathSegment};

#[derive(Debug)]
pub struct RmCmd {
    path: NodePath,
    name: String,
}

impl super::Command for RmCmd {
    /// Build a new RmCmd.
    /// Takes in an array of arguments.
    /// The build function fails if the conditions for the arguments are invalid such
    /// as invalid type or the wrong number of arguments supplied.
    fn build(arguments: &[Argument]) -> Result<Self, SyntaxError> {
        // check the argument count is correct
        if arguments.len() != 1 {
            return Err(SyntaxError::InvalidArguments);
        }

        // check that the type is a path
        let path = match &arguments[0] {
            Argument::Path(path) => path,
            _ => return Err(SyntaxError::InvalidType),
        };

        // check that the path type resolves to a file
        let name = match path.last().unwrap() {
            NodePathSegment::File(name) => name.to_string(),
            _ => return Err(SyntaxError::InvalidType),
        };

        Ok(Self {
            path: path[..path.len() - 1].to_vec(),
            name,
        })
    }

    /// Execute the rm command and remove a file based on self.path
    fn execute(&self, ctx: Rc<Context>) {
        if let Ok(target) = ctx.node_from_path(&self.path) {
            if let Err(e) = target.remove(&self.name) {
                // no file with supplied name is found in the parent folder
                println!("{}", e);
            }
        } else {
            // parent path is not found in the file tree
            println!("Invalid path");
        }
    }
}
