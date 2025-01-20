use crate::index::key::Key;
use crate::index::node::Node;
use uuid::Uuid;

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

    fn search_tree(node: &Node, value: String) -> Option<&Key> {
        for (i, key) in node.keys.iter().enumerate() {
            if key.value == value {
                return Some(key);
            } else if key.value > value {
                if node.leaf {
                    return None;
                } else {
                    return BTree::search_tree(&node.children[i], value);
                }
            }
        }

        if node.leaf {
            None
        } else {
            BTree::search_tree(&node.children[node.keys.len()], value)
        }
    }

    pub fn search(&self, value: &str) -> Option<&Key> {
        BTree::search_tree(&self.root, value.to_string())
    }
}

mod tests {
    use super::*;

    fn _valid_tree(node: &Node, limit: Option<&String>) -> bool {
        for i in 1..node.keys.len() {
            if node.keys[i - 1].value >= node.keys[i].value {
                return false;
            }

            match limit {
                None => {}
                Some(value) => {
                    if &node.keys[i].value > value {
                        return false;
                    }
                }
            }
        }

        if node.leaf {
            true
        } else {
            match node
                .children
                .iter()
                .enumerate()
                .map(|(i, node)| {
                    if i < node.keys.len() {
                        _valid_tree(&node.children[i], Some(&node.keys[i].value))
                    } else {
                        _valid_tree(&node.children[i], None)
                    }
                })
                .reduce(|acc, e| acc & e)
            {
                None => false,
                Some(result) => result,
            }
        }
    }

    #[test]
    fn create() {
        let order = 3;
        let tree = BTree::create(order);

        assert_eq!(tree.order, order);
        assert_eq!(tree.root.leaf, true);
    }

    #[test]
    fn insert() {
        let order = 3;
        let mut tree = BTree::create(order);

        tree.root.keys = vec![
            Key::create("G"),
            Key::create("M"),
            Key::create("P"),
            Key::create("X"),
        ];

        tree.root.leaf = false;

        tree.root.children = vec![
            {
                let mut child = Node::empty(order, true);
                child.keys = vec![
                    Key::create("A"),
                    Key::create("C"),
                    Key::create("D"),
                    Key::create("E"),
                ];
                child
            },
            {
                let mut child = Node::empty(order, true);
                child.keys = vec![Key::create("J"), Key::create("K")];
                child
            },
            {
                let mut child = Node::empty(order, true);
                child.keys = vec![Key::create("N"), Key::create("O")];
                child
            },
            {
                let mut child = Node::empty(order, true);
                child.keys = vec![
                    Key::create("R"),
                    Key::create("S"),
                    Key::create("T"),
                    Key::create("U"),
                    Key::create("V"),
                ];
                child
            },
            {
                let mut child = Node::empty(order, true);
                child.keys = vec![Key::create("Y"), Key::create("Z")];
                child
            },
        ];

        tree.insert(Key::create("B"));
        tree.insert(Key::create("Q"));
        tree.insert(Key::create("L"));
        tree.insert(Key::create("F"));

        assert!(_valid_tree(&tree.root, None));
    }

    #[test]
    fn search() {
        let order = 3;
        let mut tree = BTree::create(order);

        let uuids: Vec<String> = (0..100).map(|_| Uuid::new_v4().to_string()).collect();

        for uuid in &uuids {
            tree.insert(Key::create(uuid));

            assert!(match tree.search(uuid) {
                None => {false},
                Some(key) => {key.value == *uuid}
            });
        }

        for uuid in uuids {
            assert!(match tree.search(&uuid) {
                None => {false},
                Some(key) => {key.value == uuid}
            });
        }

        let none_found: Option<bool> = (0..100).map(|_| {
            match tree.search(&Uuid::new_v4().to_string()) {
                None => {true},
                Some(_) => {false}
            }    
        }).reduce(|acc,e| acc & e);

        assert!(match none_found {
            None => {false},
            Some(result) => result
        });

    }   
}
