use crate::Context;
use std::fmt::Debug;
use std::rc::Rc;
use crate::parser::{SyntaxError, Argument, NodePath, NodePathSegment};
use crate::tree::Node;

#[derive(Debug)]
pub struct TouchCmd {
    path: NodePath,
    file_name: String,
    size: usize,
}

impl super::Command for TouchCmd {
    /// Build a new TouchCmd.
    /// Takes in an array of arguments. This function also validates the
    /// arguments and returns a SyntaxError if they are invalid.
    fn build(arguments: &[Argument]) -> Result<Self, SyntaxError> {
        // check that the supplied argument count is correct.
        if arguments.len() != 2 && arguments.len() != 1 {
            return Err(SyntaxError::InvalidArguments);
        }

        // check that the first argument is a path
        let path = match &arguments[0] {
            Argument::Path(path) => path,
            _ => return Err(SyntaxError::InvalidType),
        };

        // get the file name
        let file_name = match path.last().unwrap() {
            NodePathSegment::File(name) => name.clone(),
            _ => return Err(SyntaxError::InvalidType),
        };

        // check that the second argument is a size, set size to 1 if not supplied
        let size = match arguments.get(1) {
            Some(Argument::Number(n)) => *n,
            Some(_) => return Err(SyntaxError::InvalidType),
            None => 1,
        };

        Ok(Self {
            path: path[..path.len() - 1].to_vec(),
            file_name,
            size,
        })
    }

    /// Execute the touch command, this creates a new file.
    fn execute(&self, ctx: Rc<Context>) {
        // validate various things about file name.
        if self.file_name.contains(" ") {
            println!("The file name cannot contain spaces");
            return;
        }

        if self.file_name.len() > 12 {
            println!("The file name cannot be over 12 characters");
            return;
        }

        if self.size >= 4194304 {
            println!("The file size can only be up to 4GB");
            return;
        }

        if self.size == 0 {
            println!("Cannot create a file with 0 size");
            return;
        }
        
        if self.file_name.split(".").last().unwrap().len() != 3 {
            println!("File extension must be 3 characters because Doc said so.");
            return;
        }

        // create the new file in target.
        if let Ok(target) = ctx.node_from_path(&self.path) {
            let new_file = Rc::new(Node::new_file(&self.file_name, self.size));
            target.add(new_file).unwrap();
        }
    }
}
