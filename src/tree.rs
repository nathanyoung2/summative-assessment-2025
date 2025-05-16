use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt;

use crate::parser::{NodePath, NodePathSegment};


#[derive(Debug, Clone)]
/// The holds important references to nodes on the tree
/// And contains methods for tree manipulation.
pub struct Context {
    root: RefCell<Rc<Node>>,
    current_dir: RefCell<Rc<Node>>,
}

impl Context {
    /// Create a new context.
    pub fn new(root: Rc<Node>, current_dir: Rc<Node>) -> Self {
        Self {
            root: RefCell::new(root),
            current_dir: RefCell::new(current_dir),
        }
    }

    /// Get the current directory.
    pub fn current_dir(&self) -> &RefCell<Rc<Node>> {
        &self.current_dir
    }

    /// Set the curent directory to `new_dir`
    pub fn set_current_dir(&self, new_dir: Rc<Node>) {
        *self.current_dir.borrow_mut() = new_dir;
    }

    /// Change a directory to one of its children.
    /// with the name: `dir_name`
    fn dir_to_child(current_dir: &mut Rc<Node>, dir_name: &str) -> Result<(), InvalidFolder> {
        let children = current_dir.children().unwrap().borrow();
        let dir = children.iter().find(|dir| { 
            &dir.name().unwrap() == dir_name
        });

        if let Some(dir) = dir {
            let dir = Rc::clone(&dir);

            // the reference to self.current_dir's children means that current_dir is
            // borrowed. Therefore children must be dropped before borrowing current_dir mutably
            // to assign to it.
            drop(children);

            *current_dir = dir;
            return Ok(());
        }

        Err(InvalidFolder)
    }

    /// Change `dir` to its parent
    fn dir_to_parent(dir: &mut Rc<Node>) {
        let parent = Weak::upgrade(&dir.parent().unwrap().borrow()).unwrap();
        *dir = parent;
    }

    /// Replaces the directory stored in the dir parameter with the root directory 
    fn dir_to_root(&self, dir: &mut Rc<Node>) {
        *dir = Rc::clone(&self.root.borrow());
    }

    /// Get a node from the tree from a `NodePath`.
    pub fn node_from_path(&self, dir: &NodePath) -> Result<Rc<Node>, InvalidFolder> {
        let mut buffer_dir = Rc::clone(&self.current_dir.borrow());

        for path_segment in dir.iter() {
            match path_segment {
                NodePathSegment::Root => self.dir_to_root(&mut buffer_dir),
                NodePathSegment::Dir(folder_name) => {
                    Self::dir_to_child(&mut buffer_dir, folder_name)?;
                },
                NodePathSegment::Parent => Self::dir_to_parent(&mut buffer_dir),
                _ => return Err(InvalidFolder)
            }
        }

        Ok(buffer_dir)
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidFolder;

#[derive(Debug)]
/// Represents a node in the file tree, could be the root, a folder, or a file.
pub enum Node {
    /// Root is not accessable by the user, but it only contains children.
    Root {
        children: RefCell<Vec<Rc<Node>>>,
    },
    /// A folder with a parent and children containing more nodes.
    Folder {
        /// Folder name
        name: RefCell<String>,

        /// size in kilobytes of the folder (sum of the sizes of its children)
        size: RefCell<usize>,
        
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,

        /// Depth represents the depth into the heirarchy where the root has a depth of 0
        depth: RefCell<usize>,
    },
    /// A file has no children
    File {
        /// File name
        name: RefCell<String>,

        /// Size of the file (in kilobytes)
        size: RefCell<usize>,

        parent: RefCell<Weak<Node>>,

        /// Depth represents the depth into the heirarchy where the root has a depth of 0
        depth: RefCell<usize>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(parent) = self.parent() {
            let parent = parent.borrow().upgrade().unwrap();
            return write!(f, "{}/{}", parent, self.name().unwrap_or(String::new()));
        }
        write!(f, "")
    }
}

impl Node {
    /// Create a new root node
    pub fn new_root() -> Self {
        Self::Root {
            children: RefCell::new(Vec::new()),
        }
    }

    /// Create a new folder with the name, `name`
    pub fn new_folder(name: &str) -> Self {
        Self::Folder {
            name: RefCell::new(name.to_string()),
            size: RefCell::new(0),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
            depth: RefCell::new(0),
        }
    }

    /// Create a new file of size: `size` with name: `name`
    pub fn new_file(name: &str, size: usize) -> Self {
        Self::File {
            name: RefCell::new(name.to_string()),
            size: RefCell::new(size),
            parent: RefCell::new(Weak::new()),
            depth: RefCell::new(0),
        }
    }

    /// Get a reference to the node's parent, if it has one
    pub fn parent(&self) -> Option<&RefCell<Weak<Node>>> {
        match &*self {
            Node::Folder { parent, .. } => Some(parent),
            Node::File { parent, .. } => Some(parent),
            Node::Root { .. } => None,
        }
    }

    /// Get a reference to the node's children, if it has them
    pub fn children(&self) -> Option<&RefCell<Vec<Rc<Node>>>> {
        match &*self {
            Node::Folder { children, .. } => Some(children),
            Node::Root { children, .. } => Some(children),
            Node::File { .. } => None,
        }
    }

    /// Get the node's size
    pub fn size(&self) -> Option<usize> {
        match self {
            Node::Folder { size, .. } => Some(*size.borrow()),
            Node::File { size, .. } => Some(*size.borrow()),
            Node::Root { .. } => None,
        }
    }

    /// Get the node's name
    pub fn name(&self) -> Option<String> {
        match self {
            Node::Folder { name, .. } => Some(name.borrow().clone()),
            Node::File { name, .. } => Some(name.borrow().clone()),
            Node::Root { .. } => None,
        }
    }
    
    /// Get the depth of the node
    pub fn depth(&self) -> usize {
        match self {
            Node::Folder { depth, .. } => depth.borrow().clone(),
            Node::File { depth, .. } => depth.borrow().clone(),
            Node::Root { .. } => 0,
        }
    }

    /// Get a reference to the node's depth.
    pub fn depth_ref(&self) -> Option<&RefCell<usize>> {
        match self {
            Node::Folder { depth, .. } => Some(depth),
            Node::File { depth, .. } => Some(depth),
            Node::Root { .. } => None,
        }
    }

    /// Add the node: `child` to this node.
    pub fn add(self: Rc<Self>, child: Rc<Self>) -> Result<(), NodeTypeError> {
        let add_child_to_children =
            |children: &RefCell<Vec<Rc<Self>>>,
            file_size: Option<&RefCell<usize>>| -> Result<(), NodeTypeError> {
                *child.parent().ok_or(NodeTypeError)?.borrow_mut() = Rc::downgrade(&self);
                *child.depth_ref().ok_or(NodeTypeError)?.borrow_mut() = self.depth() + 1;
                children.borrow_mut().push(Rc::clone(&child));

                if let Some(size) = file_size {
                    *size.borrow_mut() += child.size().unwrap();
                }

                Ok(())
            };

        match &*self {
            Node::Root { children, .. } => add_child_to_children(children, None)?,
            Node::Folder { children, size, .. } => add_child_to_children(children, Some(size))?,
            Node::File { .. } => return Err(NodeTypeError),
        }

        Ok(())
    }

    /// Remove a node by name from this node.
    pub fn remove(self: Rc<Self>, node_name: &str) -> Result<(), String> {
        let get_index = || {
            for (i, node) in self.children().unwrap().borrow().iter().enumerate() {
                if &node.name().unwrap() == node_name {
                    return Ok(i);
                }
            }
            return Err(format!["Could not locate item: {}", node_name]);
        };

        let index = get_index()?;
        self.children().unwrap().borrow_mut().swap_remove(index);
        Ok(())
    }
}

#[derive(Debug)]
pub struct NodeTypeError;

/// Build a hardcoded file tree
pub fn build_tree(username: &str) -> Context {
    let root = Rc::new(Node::new_root());
    let home = Rc::new(Node::new_folder("home"));
    let user = Rc::new(Node::new_folder(username));
    
    let ctx = Context::new(Rc::clone(&root), Rc::clone(&user));

    root.add(Rc::clone(&home)).unwrap();
    home.add(Rc::clone(&user)).unwrap();

    let documents = Rc::new(Node::new_folder("documents"));

    Rc::clone(&documents).add(Rc::new(Node::new_file("cv.pdf", 1))).unwrap();
    Rc::clone(&documents).add(Rc::new(Node::new_file("data.dat", 1))).unwrap();

    let downloads = Rc::new(Node::new_folder("downloads"));

    let music = Rc::new(Node::new_folder("music"));
    
    Rc::clone(&music).add(Rc::new(Node::new_file("1.mp3", 1))).unwrap();
    Rc::clone(&music).add(Rc::new(Node::new_file("2.mp3", 1))).unwrap();
    Rc::clone(&music).add(Rc::new(Node::new_file("3.mp3", 1))).unwrap();
    Rc::clone(&music).add(Rc::new(Node::new_file("4.mp3", 1))).unwrap();
    Rc::clone(&music).add(Rc::new(Node::new_file("5.mp3", 1))).unwrap();
    Rc::clone(&music).add(Rc::new(Node::new_file("6.mp3", 1))).unwrap();
    Rc::clone(&music).add(Rc::new(Node::new_file("7.mp3", 1))).unwrap();
    Rc::clone(&music).add(Rc::new(Node::new_file("8.mp3", 1))).unwrap();
    Rc::clone(&music).add(Rc::new(Node::new_file("9.mp3", 1))).unwrap();
    Rc::clone(&music).add(Rc::new(Node::new_file("10.mp3", 1))).unwrap();
    
    let photos = Rc::new(Node::new_folder("photos"));

    Rc::clone(&user).add(Rc::clone(&documents)).unwrap();
    Rc::clone(&user).add(Rc::clone(&downloads)).unwrap();
    Rc::clone(&user).add(Rc::clone(&music)).unwrap();
    Rc::clone(&user).add(Rc::clone(&photos)).unwrap();

    let japan2026 = Rc::new(Node::new_folder("japan2026"));

    photos.add(Rc::clone(&japan2026)).unwrap();

    ctx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folder_file_size() {
        let folder = Rc::new(Node::new_folder("folder"));
        assert_eq!(folder.size().unwrap(), 0);

        let file = Rc::new(Node::new_file("file", 2));
        let other_file = Rc::new(Node::new_file("file", 3));

        Rc::clone(&folder).add(file).unwrap();
        Rc::clone(&folder).add(other_file).unwrap();

        assert_eq!(folder.size().unwrap(), 5);
    }

    #[test]
    fn navigate_valid() {
        let ctx = build_tree("test_user");
        let result = ctx.node_from_path(&vec![NodePathSegment::Dir("documents".to_string())]);
        assert!(result.is_ok());
    }
    
    #[test]
    fn navigate_invalid() {
        let ctx = build_tree("test_user");
        let result = ctx.node_from_path(&vec![NodePathSegment::Dir("abcdefg".to_string())]);
        assert!(result.is_err());
    }
}

