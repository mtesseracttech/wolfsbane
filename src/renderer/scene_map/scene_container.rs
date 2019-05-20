use core::fmt;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug)]
pub struct SceneContainer {
    members: HashMap<u64, Rc<RefCell<SceneNode>>>,
    cur_id: u64,
}

impl SceneContainer {
    pub fn new() -> Rc<RefCell<SceneContainer>> {
        Rc::new(RefCell::new(SceneContainer {
            members: HashMap::new(),
            cur_id: 0,
        }))
    }

    pub fn get_node(&self, node_id: u64) -> Option<Rc<RefCell<SceneNode>>> {
        Some(self.members.get(&node_id)?.clone())
    }

    pub fn add_node(&mut self, node: Rc<RefCell<SceneNode>>) -> u64 {
        let node_id = self.cur_id;
        node.as_ref().borrow_mut().set_id(node_id);
        self.members.insert(self.cur_id, node.to_owned());
        self.cur_id += 1;
        node_id
    }

    pub fn remove_node(&mut self, node_id: u64) {
        self.members.remove(&node_id);
    }

    pub fn has_node(&self, node_id: u64) -> bool {
        self.members.contains_key(&node_id)
    }

    pub fn node_count(&self) -> usize {
        self.members.len()
    }
}
