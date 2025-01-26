use crate::index::key::Key;
use crate::index::node::Node;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct BTree {
    root: Node,
    order: usize,
    path: String,
}

impl BTree {
    fn filename(&self) -> String {
        format!("{}/{}.json", self.path, Uuid::new_v4().to_string())
    }

    fn save(&self) {
        let mut file = File::create(self.filename()).unwrap();
        file.write_all(serde_json::to_string(self).unwrap().as_bytes())
            .unwrap();
    }

    pub fn create(order: usize, path: &str) -> BTree {

        fs::create_dir(path).unwrap();

        let btree = BTree {
            root: Node::empty(order, true),
            path: path.to_string(),
            order,
        };

        btree.save();
        btree
    }

    pub fn insert(&mut self, key: Key) {
        if self.root.is_full(self.order) {
            let mut new_root = Node::empty(self.order, false);
            new_root.children.push(self.root.clone().filename);
            new_root.split(0, self.order);
            self.root = new_root;
        }

        self.root.insert(key, self.order);
    }

    fn search_tree(node: &Node, value: String) -> Option<Key> {
        for (i, key) in node.keys.iter().enumerate() {
            if key.value == value {
                return Some(key.clone());
            } else if key.value > value {
                if node.leaf {
                    return None;
                } else {
                    return BTree::search_tree(&Node::load(&node.children[i]), value);
                }
            }
        }

        if node.leaf {
            None
        } else {
            BTree::search_tree(&Node::load(&node.children[node.keys.len()]), value)
        }
    }

    pub fn search(&self, value: &str) -> Option<Key> {
        BTree::search_tree(&self.root, value.to_string())
    }
}

mod tests {
    use super::*;
    const _PLACEHOLDER: (u64, u64) = (0, 0);

    fn _create_key(value: &str) -> Key {
        Key::create(value, _PLACEHOLDER)
    }

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
                .map(|(i, node_filename)| {
                    let node = Node::load(node_filename);
                    if i < node.keys.len() {
                        _valid_tree(&Node::load(&node.children[i]), Some(&node.keys[i].value))
                    } else {
                        _valid_tree(&Node::load(&node.children[i]), None)
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
        let tree = BTree::create(order, "files_test_insert");

        assert_eq!(tree.order, order);
        assert_eq!(tree.root.leaf, true);
    }

    #[test]
    fn insert() {
        let order = 3;
        let mut tree = BTree::create(order, "files_test_insert");

        tree.root.keys = vec![
            _create_key("G"),
            _create_key("M"),
            _create_key("P"),
            _create_key("X"),
        ];

        tree.root.leaf = false;
        tree.root.save();

        tree.root.children = vec![
            {
                let mut child = Node::empty(order, true);
                child.keys = vec![
                    _create_key("A"),
                    _create_key("C"),
                    _create_key("D"),
                    _create_key("E"),
                ];
                child.save();
                child.filename
            },
            {
                let mut child = Node::empty(order, true);
                child.keys = vec![_create_key("J"), _create_key("K")];
                child.save();
                child.filename
            },
            {
                let mut child = Node::empty(order, true);
                child.keys = vec![_create_key("N"), _create_key("O")];
                child.save();
                child.filename
            },
            {
                let mut child = Node::empty(order, true);
                child.keys = vec![
                    _create_key("R"),
                    _create_key("S"),
                    _create_key("T"),
                    _create_key("U"),
                    _create_key("V"),
                ];
                child.save();
                child.filename
            },
            {
                let mut child = Node::empty(order, true);
                child.keys = vec![_create_key("Y"), _create_key("Z")];
                child.save();
                child.filename
            },
        ];

        tree.insert(_create_key("B"));
        tree.insert(_create_key("Q"));
        tree.insert(_create_key("L"));
        tree.insert(_create_key("F"));

        assert!(_valid_tree(&tree.root, None));
    }

    #[test]
    fn search() {
        use uuid::Uuid;
        let order = 3;
        let mut tree = BTree::create(order, "files_test_search");

        let uuids: Vec<String> = (0..100).map(|_| Uuid::new_v4().to_string()).collect();

        for uuid in &uuids {
            tree.insert(_create_key(uuid));

            assert!(match tree.search(uuid) {
                None => {
                    false
                }
                Some(key) => {
                    key.value == *uuid
                }
            });
        }

        for uuid in uuids {
            assert!(match tree.search(&uuid) {
                None => {
                    false
                }
                Some(key) => {
                    key.value == uuid
                }
            });
        }

        let none_found: Option<bool> = (0..100)
            .map(|_| match tree.search(&Uuid::new_v4().to_string()) {
                None => true,
                Some(_) => false,
            })
            .reduce(|acc, e| acc & e);

        assert!(match none_found {
            None => {
                false
            }
            Some(result) => result,
        });
    }
}
