use super::scene_container::SceneContainer;
use crate::renderer::Transform;
use core::borrow::BorrowMut;
use core::fmt;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::string::ToString;

pub struct SceneNode {
    transform: Transform<f32>,
    container: Rc<RefCell<SceneContainer>>,
    parent: Option<u64>,
    children: Vec<u64>,
    id: u64,
    name: String,
}

impl SceneNode {
    pub fn create_new(
        container: Rc<RefCell<SceneContainer>>,
        name: Option<&str>,
        parent: Option<u64>,
    ) -> u64 {
        let mut container_deref = (*container).borrow_mut();
        let new_node_id = container_deref.add_node(Rc::new(RefCell::new(SceneNode {
            transform: Transform::default(),
            container: container.clone(),
            parent,
            children: Vec::new(),
            id: 0,
            name: if name.is_some() {
                name.unwrap().to_string()
            } else {
                "unnamed".to_string()
            },
        })));

        if parent.is_some() {
            let parent_id = parent.unwrap();
            let parent_node = container_deref.get_node(parent_id).unwrap();
            let mut parent_node = (*parent_node).borrow_mut();
            parent_node.add_child_id(new_node_id);
        }

        new_node_id
    }

    pub(in crate::renderer::scene_map) fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn get_transform(&mut self) -> &mut Transform<f32> {
        self.transform.borrow_mut()
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn add_child_id(&mut self, node_id: u64) {
        if (*self.container).borrow_mut().has_node(node_id) {
            match self.children.iter().find(|node| **node == node_id) {
                None => self.children.push(node_id),
                Some(_) => (),
            }
        }
    }

    pub fn add_child(&mut self, child_node: Rc<RefCell<SceneNode>>) {
        self.add_child_id((*child_node).borrow().get_id());
    }

    pub fn remove_child(&mut self, node_id: u64) {
        let position = self.children.iter().position(|node| *node == node_id);
        match position {
            None => (),
            Some(index) => {
                self.children.remove(index);
            }
        }
    }

    pub fn get_child_count(&self) -> usize {
        self.children.len()
    }

    //Gets a child from a given index, if the child was not available, it returns None
    pub fn get_child(&self, index: usize) -> Option<Rc<RefCell<SceneNode>>> {
        (*self.container)
            .borrow_mut()
            .get_node(self.children[index])
    }

    //Returns a list of child id's guaraneed to be available for usage from the data structure
    pub fn get_children_ids(&mut self) -> Vec<u64> {
        self.children
            .iter()
            .take_while(|c| (*self.container).borrow().has_node(**c))
            .cloned()
            .collect()
    }

    //Returns a list of children guaranteed to not be deleted from the data structure
    pub fn get_children(&self) -> Vec<Rc<RefCell<SceneNode>>> {
        self.children
            .iter()
            .take_while(|c| (*self.container).borrow().has_node(**c))
            .flat_map(|c| (*self.container).borrow_mut().get_node(*c))
            .collect()
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<SceneNode>>> {
        match self.parent {
            None => None,
            Some(parent_id) => (*self.container).borrow_mut().get_node(parent_id),
        }
    }

    pub fn set_parent_id(&mut self, parent_id: u64) {
        if (*self.container).borrow_mut().has_node(parent_id) {
            self.parent = Some(parent_id);
        }
    }

    pub fn get_parent_id(&self) -> Option<u64> {
        self.parent
    }
}

impl fmt::Debug for SceneNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {:?}, name: {:?}, parent: {:?}, children: {:?}",
            self.id, self.name, self.parent, self.children
        )
    }
}
