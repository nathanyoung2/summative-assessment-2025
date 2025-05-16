use crate::Context;
use std::fmt::Debug;
use std::rc::Rc;
use crate::parser::{SyntaxError, Argument, NodePath, NodePathSegment};

#[derive(Debug)]
pub struct RmdirCmd {
    path: NodePath,
    name: String,
}

impl super::Command for RmdirCmd {
    /// Build an RmdirCmd.
    /// Takes in an array of arguments. This function validates the arguments
    /// and returns a syntax error if they are invalid.
    fn build(arguments: &[Argument]) -> Result<Self, SyntaxError> {
        // make sure that the supplied argument count is correct
        if arguments.len() != 1 {
            return Err(SyntaxError::InvalidArguments);
        }

        // make sure that the type of the argument is a path
        let path = match &arguments[0] {
            Argument::Path(path) => path,
            _ => return Err(SyntaxError::InvalidType),
        };

        // get the name of the folder
        let name = match path.last().unwrap() {
            // make sure that the path resolves to a dir
            NodePathSegment::Dir(name) => name.to_string(),
            _ => return Err(SyntaxError::InvalidType),
        };

        Ok(Self {
            path: path[..path.len() - 1].to_vec(),
            name,
        })
    }

    /// Execute the Rmdir command.
    fn execute(&self, ctx: Rc<Context>) {
        if let Ok(target) = ctx.node_from_path(&self.path) {
            // check that the depth of the target is not less than the current directory.
            if target.depth() < ctx.current_dir().borrow().depth() {
                println!("Cannot remove directory as it is less deep than the current directory");
                return;
            }

            // remove the target
            if let Err(e) = target.remove(&self.name) {
                println!("{}", e);
            }
        } else {
            println!("Invalid path");
        }
    }
}
