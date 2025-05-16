use crate::Context;
use std::fmt::Debug;
use std::rc::Rc;
use crate::parser::{SyntaxError, Argument, NodePath, NodePathSegment};
use crate::tree::Node;

#[derive(Debug)]
pub struct MkdirCmd{
    path: NodePath,
    dir_name: String,
}

impl super::Command for MkdirCmd {
    /// Build a MkdirCmd.
    /// Takes in an array of arguments. The function also validates the arguments
    /// and returns a SyntaxError if they are invalid.
    fn build(arguments: &[Argument]) -> Result<Self, SyntaxError> {
        // validate argument count
        if arguments.len() != 1 {
            return Err(SyntaxError::InvalidArguments);
        }

        // validate that the argument is a path
        let path = match &arguments[0] {
            Argument::Path(path) => path,
            _ => return Err(SyntaxError::InvalidType),
        };

        // get the dir name from the path 
        let dir_name = match path.last().unwrap() {
            NodePathSegment::Dir(name) => name.clone(),
            _ => return Err(SyntaxError::InvalidType),
        };

        Ok(Self {
            path: path[..path.len() - 1].to_vec(),
            dir_name,
        })
    }

    /// Execute the mkdir command. This creates a new directory.
    fn execute(&self, ctx: Rc<Context>) {
        // assure the directory name is not over the maximum allowed character count
        if self.dir_name.len() > 12 {
            println!("The dir name cannot be over 12 characters");
            return;
        }

        // create the new directory.
        if let Ok(target) = ctx.node_from_path(&self.path) {
            let new_dir = Rc::new(Node::new_folder(&self.dir_name));
            target.add(new_dir).unwrap();
        }
    }
}
