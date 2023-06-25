use crate::Node;

pub struct DataNode {
    id: i32,
    parent_id: Option<i32>,
    child_ids: Vec<i32>,
}

impl DataNode {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            parent_id: None,
            child_ids: vec![],
        }
    }
}

impl Node<i32> for DataNode {
    fn id(&self) -> i32 {
        self.id
    }

    fn parent_id(&self) -> Option<i32> {
        self.parent_id
    }

    fn child_ids_vec(&self) -> Vec<i32> {
        self.child_ids.clone()
    }

    fn set_parent_id(&mut self, parent: i32) {
        self.parent_id = Some(parent);
    }

    fn add_child_id(&mut self, child_id: i32) {
        self.child_ids.push(child_id);
    }

    fn remove_child_id(&mut self, child_id: &i32) {
        self.child_ids.retain(|id| id != child_id);
    }
}
