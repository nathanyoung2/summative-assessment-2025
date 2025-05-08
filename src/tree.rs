use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Context {
    root: Rc<Node>,
    current_dir: Rc<Node>,
}

pub enum Node {
    Root {
        children: RefCell<Vec<Rc<Node>>>,
    },
    Folder {
        name: String,
        size: usize,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    },
    File {
        name: String,
        size: usize,
        parent: RefCell<Weak<Node>>,
    },
}

impl Node {
    pub fn parent(&self) -> Option<&RefCell<Weak<Node>>> {
        match &*self {
            Node::Folder { parent, .. } => Some(parent),
            Node::File { parent, .. } => Some(parent),
            Node::Root { .. } => None,
        }
    }

    pub fn add(self: Rc<Self>, child: Rc<Self>) -> Result<(), NodeTypeError> {
        let add_child_to_children =
            |children: &RefCell<Vec<Rc<Self>>>| -> Result<(), NodeTypeError> {
                *child.parent().ok_or(NodeTypeError)?.borrow_mut() = Rc::downgrade(&self);
                children.borrow_mut().push(child);
                Ok(())
            };

        match &*self {
            Node::Root { children, .. } => add_child_to_children(children)?,
            Node::Folder { children, .. } => add_child_to_children(children)?,
            Node::File { .. } => return Err(NodeTypeError),
        }

        Ok(())
    }
}

struct NodeTypeError;

fn build_tree() {}
