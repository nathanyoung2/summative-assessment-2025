use crate::Context;
use std::fmt::Debug;
use crate::parser::{SyntaxError, NodePath, NodePathSegment};

const EXPECTED_ARG_COUNT: usize = 1;

#[derive(Debug)]
pub struct CdCmd {
    path: NodePath,
}

impl super::Command for CdCmd {
    fn build(arguments: &[NodePath]) -> Result<Self, SyntaxError> {
        if arguments.len() != EXPECTED_ARG_COUNT {
            return Err(SyntaxError::InvalidArguments);
        }

        if let NodePathSegment::File(..) = arguments[0].last().unwrap() {
            return Err(SyntaxError::InvalidType);
        }
        
        Ok(Self {
            path: arguments[0].clone(),
        })
    }

    fn execute(&self, ctx: Context) {}
}
