use crate::index::node::Node;
use crate::index::key::Key;

#[derive(Clone)]
pub struct BTree {
    root: Node,
    order: usize,
}

impl BTree {
    pub fn create(order: usize) -> BTree {
        BTree {
            root: Node::empty(order, true),
            order,
        }
    }

    pub fn insert(&mut self, key: Key) {
        if self.root.is_full(self.order) {
            let mut new_root = Node::empty(self.order, false);
            new_root.children.push(self.root.clone());
            new_root.split(0, self.order);
            self.root = new_root;
        }

        self.root.insert(key, self.order);
    }
}