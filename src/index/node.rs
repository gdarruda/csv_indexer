use crate::index::key::Key;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Node {
    pub keys: Vec<Key>,
    pub children: Vec<String>,
    pub leaf: bool,
    pub filename: String,
}

impl Node {
    fn filename(path: &str) -> String {
        format!("{}/{}.json", path, Uuid::new_v4().to_string())
    }

    pub fn load(filename: &str) -> Node {
        serde_json::from_slice(&fs::read(filename).unwrap()).unwrap()
    }

    fn find_position(&self, key: &Key) -> usize {
        let mut idx = 0;

        for (i, iter_key) in self.keys.iter().enumerate() {
            idx = i;
            if iter_key.value > key.value {
                break;
            }
        }

        if idx + 1 == self.keys.len() {
            if key.value > self.keys[idx].value {
                idx += 1;
            }
        }

        idx
    }

    pub fn save(&self) {
        let mut file = File::create(&self.filename).unwrap();
        file.write_all(serde_json::to_string(self).unwrap().as_bytes())
            .unwrap();
    }

    fn add_key(&mut self, idx: usize, key: Key) {
        if self.keys.len() == 0 {
            self.keys.push(key);
        } else {
            self.keys.insert(idx, key);
        }
        self.save();
    }

    pub fn empty(order: usize, leaf: bool, path: &str) -> Node {
        Node {
            keys: Vec::with_capacity(2 * order - 1),
            children: Vec::with_capacity(2 * order),
            leaf,
            filename: Node::filename(path),
        }
    }

    pub fn is_full(&self, order: usize) -> bool {
        self.keys.len() == 2 * order - 1
    }

    pub fn split(&mut self, pivot: usize, order: usize, path: &str) {
        let left = &mut Node::load(&self.children[pivot]);
        let key = left.keys[order - 1].clone();

        let right = Node {
            keys: left.keys[order..left.keys.len()].to_owned(),
            children: match left.leaf {
                true => Vec::with_capacity(2 * order),
                false => left.children[order..left.children.len()].to_owned(),
            },
            leaf: left.leaf,
            filename: Node::filename(path),
        };

        right.save();

        left.keys.resize(order - 1, Key::create("", (0, 0)));

        if !left.leaf {
            left.children.resize(order, String::from(""));
        }

        left.save();

        self.keys.insert(pivot, key);
        self.children.insert(pivot + 1, right.filename);

        self.save()
    }

    pub fn insert(&mut self, key: Key, order: usize, path: &str) {
        if self.leaf {
            self.add_key(self.find_position(&key), key.clone());
        } else {
            let mut idx = self.find_position(&key);

            if Node::load(&self.children[idx]).is_full(order) {
                self.split(idx, order, path);
                idx = self.find_position(&key);
            }

            Node::load(&self.children[idx]).insert(key, order, path);
        }
    }
}

mod tests {
    use super::*;
    const _PLACEHOLDER: (u64, u64) = (0, 0);

    fn _create_key(value: &str) -> Key {
        Key::create(value, _PLACEHOLDER)
    }

    #[test]
    fn add_key() {
        let path = "node_test_add_key";
        fs::create_dir(path).unwrap();

        let mut node = Node::empty(3, true, path);

        let first_key = _create_key("A");
        let second_key = _create_key("B");
        let last_key = _create_key("C");

        node.add_key(0, _create_key("A"));
        assert_eq!(node.keys[0].value, first_key.value);

        node.add_key(1, _create_key("C"));
        assert_eq!(node.keys[1].value, last_key.value);

        node.add_key(1, _create_key("B"));
        assert_eq!(node.keys[1].value, second_key.value);
        assert_eq!(node.keys[2].value, last_key.value);

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn find_position() {
        let path = "node_test_find_position";
        fs::create_dir(path).unwrap();
        let mut node = Node::empty(3, true, path);

        vec!["B", "D", "F"].iter().enumerate().for_each(|(i, s)| {
            node.add_key(i, _create_key(s));
        });

        assert_eq!(node.find_position(&_create_key("A")), 0);
        assert_eq!(node.find_position(&_create_key("C")), 1);
        assert_eq!(node.find_position(&_create_key("E")), 2);
        assert_eq!(node.find_position(&_create_key("G")), 3);

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn empty() {
        let path = "node_test_empty";
        fs::create_dir(path).unwrap();
        let order = 3;
        let node = Node::empty(3, true, path);
        

        assert_eq!(node.keys.capacity(), 2 * order - 1);
        assert_eq!(node.children.capacity(), 2 * order);

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn is_full() {
        let order = 2;
        let path = "node_test_is_full";
        fs::create_dir(path).unwrap();
        let mut node = Node::empty(2, true, path);

        vec!["A", "B"].iter().enumerate().for_each(|(i, s)| {
            node.add_key(i, _create_key(s));
        });

        assert!(!node.is_full(order));

        node.add_key(2, _create_key("C"));

        assert!(node.is_full(order));

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn split() {
        let order = 3;
        let path = "node_test_split";
        fs::create_dir(path).unwrap();
        let mut node = Node::empty(order, true, path);

        vec!["A", "B", "C", "D", "E"]
            .iter()
            .enumerate()
            .for_each(|(i, s)| {
                node.add_key(i, _create_key(s));
            });

        let mut father = Node::empty(order, false, &path);
        father.children.push(node.filename);
        father.split(0, order, path);

        assert_eq!(father.keys.len(), 1);
        assert_eq!(Node::load(&father.children[0]).keys.len(), 2);
        assert_eq!(Node::load(&father.children[1]).keys.len(), 2);

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn insert() {
        let order = 3;
        let path = "node_test_insert";
        fs::create_dir(path).unwrap();
        let mut node = Node::empty(order, true, path);

        vec!["A", "Z", "C", "J", "E"].iter().for_each(|s| {
            node.insert(_create_key(s), order, path);
        });

        vec!["A", "C", "E", "J", "Z"]
            .iter()
            .enumerate()
            .for_each(|(i, s)| assert_eq!(node.keys[i].value, s.to_string()));

        fs::remove_dir_all(path).unwrap();
    }
}
