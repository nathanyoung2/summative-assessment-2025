use crate::Context;
use std::fmt::Debug;
use std::rc::Rc;
use crate::parser::{SyntaxError, Argument, NodePath, NodePathSegment};
use crate::tree::Node;

#[derive(Debug)]
pub struct LsCmd {
    path: NodePath,
}

impl super::Command for LsCmd {
    /// Build an LsCmd.
    /// Takes in an array of arguments. This function validates the arguments and
    /// returns a SyntaxError if they are invalid.
    fn build(arguments: &[Argument]) -> Result<Self, SyntaxError> {
        // check that the supplied argument count is correct.
        if arguments.len() != 1 && arguments.len() != 0 {
            return Err(SyntaxError::InvalidArguments);
        }

        // handle the case where a path to list is supplied
        if arguments.len() == 1 {
            if let Argument::Path(node_path) = &arguments[0] {
                // make sure that the path resolves to a folder
                if let NodePathSegment::File(..) | NodePathSegment::Root = node_path.last().unwrap() {
                    return Err(SyntaxError::InvalidType);
                }

                return Ok(Self {
                    path: node_path.clone(),
                });
            } else {
                return Err(SyntaxError::InvalidType);
            }
        }

        Ok(Self {
            path: Vec::new(),
        })
    }

    /// Execute the ls command, this lists all files and folders in a directory.
    fn execute(&self, ctx: Rc<Context>) {
        let target = ctx.node_from_path(&self.path);
        
        if let Ok(target) = target {
            // print the node and its size for each node in the target dir
            for node in target.children().unwrap().borrow().iter() {
                let mut slash_buf = "";
                if let Node::Folder { .. } = **node {
                    slash_buf = "/";
                }
                println!("{}{} {}KB", node.name().unwrap(), slash_buf, node.size().unwrap());
            }
        }
    }
}
