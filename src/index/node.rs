use crate::index::key::Key;

#[derive(Clone)]
pub struct Node {
    pub keys: Vec<Key>,
    pub children: Vec<Node>,
    pub leaf: bool,
}

impl Node {
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

    fn add_key(&mut self, idx: usize, key: Key) {
        if self.keys.len() == 0 {
            self.keys.push(key);
        } else {
            self.keys.insert(idx, key);
        }
    }

    pub fn empty(order: usize, leaf: bool) -> Node {
        Node {
            keys: Vec::with_capacity(2 * order - 1),
            children: Vec::with_capacity(2 * order),
            leaf,
        }
    }

    pub fn is_full(&self, order: usize) -> bool {
        self.keys.len() == 2 * order - 1
    }

    pub fn split(&mut self, pivot: usize, order: usize) {
        let left = &mut self.children[pivot];
        let key = left.keys[order - 1].clone();

        let right = Node {
            keys: left.keys[order..left.keys.len()].to_owned(),
            children: match left.leaf {
                true => Vec::with_capacity(2 * order),
                false => left.children[order..left.children.len()].to_owned(),
            },
            leaf: left.leaf,
        };

        left.keys.resize(
            order - 1,
            Key {
                value: '_'.to_string(),
            },
        );

        if !left.leaf {
            left.children.resize(order, Node::empty(order, self.leaf));
        }

        self.keys.insert(pivot, key);
        self.children.insert(pivot + 1, right);
    }

    pub fn insert(&mut self, key: Key, order: usize) {
        if self.leaf {
            self.add_key(self.find_position(&key), key.clone());
        } else {
            let mut idx = self.find_position(&key);

            if self.children[idx].is_full(order) {
                self.split(idx, order);
                idx = self.find_position(&key);
            }

            self.children[idx].insert(key, order);
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn add_key() {
        let mut node = Node::empty(3, true);

        let first_key = Key::create("A");
        let second_key = Key::create("B");
        let last_key = Key::create("C");

        node.add_key(0, Key::create("A"));
        assert_eq!(node.keys[0].value, first_key.value);

        node.add_key(1, Key::create("C"));
        assert_eq!(node.keys[1].value, last_key.value);

        node.add_key(1, Key::create("B"));
        assert_eq!(node.keys[1].value, second_key.value);
        assert_eq!(node.keys[2].value, last_key.value);
    }

    #[test]
    fn find_position() {
        let mut node = Node::empty(3, true);

        vec!["B", "D", "F"].iter().enumerate().for_each(|(i, s)| {
            node.add_key(i, Key::create(s));
        });

        assert_eq!(node.find_position(&Key::create("A")), 0);
        assert_eq!(node.find_position(&Key::create("C")), 1);
        assert_eq!(node.find_position(&Key::create("E")), 2);
        assert_eq!(node.find_position(&Key::create("G")), 3);
    }

    #[test]
    fn empty() {
        let order = 3;
        let node = Node::empty(3, true);

        assert_eq!(node.keys.capacity(), 2 * order - 1);
        assert_eq!(node.children.capacity(), 2 * order);
    }

    #[test]
    fn is_full() {
        let order = 2;
        let mut node = Node::empty(2, true);

        vec!["A", "B"].iter().enumerate().for_each(|(i, s)| {
            node.add_key(i, Key::create(s));
        });

        assert!(!node.is_full(order));

        node.add_key(2, Key::create("C"));

        assert!(node.is_full(order));
    }

    #[test]
    fn split() {
        let order = 3;
        let mut node = Node::empty(order, true);

        vec!["A", "B", "C", "D", "E"]
            .iter()
            .enumerate()
            .for_each(|(i, s)| {
                node.add_key(i, Key::create(s));
            });

        let mut father = Node::empty(order, false);
        father.children.push(node);
        father.split(0, order);

        assert_eq!(father.keys.len(), 1);
        assert_eq!(father.children[0].keys.len(), 2);
        assert_eq!(father.children[1].keys.len(), 2);
    }

    #[test]
    fn insert() {
        let order = 3;
        let mut node = Node::empty(order, true);

        vec!["A", "Z", "C", "J", "E"].iter().for_each(|s| {
            node.insert(Key::create(s), order);
        });

        vec!["A", "C", "E", "J", "Z"]
            .iter()
            .enumerate()
            .for_each(|(i, s)| assert_eq!(node.keys[i].value, s.to_string()));
    }
}
