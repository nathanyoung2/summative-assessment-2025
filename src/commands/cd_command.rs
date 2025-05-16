use std::fmt::Debug;
use std::rc::Rc;

use crate::Context;
use crate::tree::Node;
use crate::parser::{SyntaxError, Argument, NodePath, NodePathSegment};

#[derive(Debug)]
pub struct CdCmd {
    path: NodePath,
}

impl super::Command for CdCmd {
    /// Builds a CdCmd.
    /// Takes in a an array of arguments.
    /// This function validates the arguments and returns a SyntaxError if the arguments
    /// are invalid.
    fn build(arguments: &[Argument]) -> Result<Self, SyntaxError> {
        // ensure the argument count is correct.
        if arguments.len() != 1 {
            return Err(SyntaxError::InvalidArguments);
        }

        // make sure the type is a path
        if let Argument::Path(node_path) = &arguments[0] {
            // Assure that we are not changing directory to a file or to the tree root.
            // The home folder, which is a child of the root, should be the root that is accessible
            // to a user.
            if let NodePathSegment::File(..) | NodePathSegment::Root = node_path.last().unwrap() {
                return Err(SyntaxError::InvalidType);
            }
        
            return Ok(Self {
                path: node_path.clone(),
            })
        }

        Err(SyntaxError::InvalidType)
    }

    /// Execute the CdCmd. This changes the current directory to the path supplied
    fn execute(&self, ctx: Rc<Context>) {
        let target = ctx.node_from_path(&self.path);
        if let Ok(target) = target {
            if let Node::Root { .. } = *target {
                println!("No parent folder");
            } else {
                ctx.set_current_dir(target);
            }
        }
    }
}
