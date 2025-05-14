use crate::Context;
use std::fmt::Debug;
use std::rc::Rc;
use crate::parser::{SyntaxError, NodePath, NodePathSegment};
use crate::tree::Node;

#[derive(Debug)]
pub struct LsCmd {
    path: NodePath,
}

impl super::Command for LsCmd {
    fn build(arguments: &[NodePath]) -> Result<Self, SyntaxError> {
        if arguments.len() != 1 && arguments.len() != 0 {
            return Err(SyntaxError::InvalidArguments);
        }

        if arguments.len() > 0 {
            if let NodePathSegment::File(..) | NodePathSegment::Root = arguments[0].last().unwrap() {
                return Err(SyntaxError::InvalidType);
            }

            return Ok(Self {
                path: arguments[0].clone(),
            });
        }

        Ok(Self {
            path: Vec::new(),
        })
    }

    fn execute(&self, ctx: Rc<Context>) {
        let target = ctx.node_from_path(&self.path);
        
        if let Ok(target) = target {
            for node in target.children().unwrap().borrow().iter() {
                let mut slash_buf = "";
                if let Node::Folder { .. } = **node {
                    slash_buf = "/";
                }
                println!("{}{}", node.name().unwrap(), slash_buf);
            }
        }
    }
}
