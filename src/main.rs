#[derive(Clone)]
struct Key {
    value: char,
}

#[derive(Clone)]
struct Node {
    keys: Vec<Key>,
    children: Vec<Node>,
    leaf: bool,
}

impl Node {
    fn empty(order: usize, leaf: bool) -> Node {
        Node {
            keys: Vec::with_capacity(2 * order - 1),
            children: Vec::with_capacity(2 * order),
            leaf,
        }
    }

    fn split_child(&mut self, pivot: usize, order: usize) {
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

        left.keys.resize(order - 1, Key { value: '_' });

        if !left.leaf {
            left.children.resize(order, Node::empty(order, self.leaf));
        }

        self.keys.insert(pivot, key);
        self.children.insert(pivot + 1, right);
    }

    fn add_key(&mut self, idx: usize, key: Key) {
        if self.keys.len() == 0 {
            self.keys.push(key);
        } else {
            self.keys.insert(idx, key);
        }
    }

    fn is_full(&self, order: usize) -> bool {
        self.keys.len() == 2 * order - 1
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

    fn insert(&mut self, key: Key, order: usize) {
        if self.leaf {
            self.add_key(self.find_position(&key), key.clone());
        } else {
            let mut idx = self.find_position(&key);

            if self.children[idx].is_full(order) {
                self.split_child(idx, order);
                idx = self.find_position(&key);
            }

            self.children[idx].insert(key, order);
        }
    }
}

#[derive(Clone)]
struct BTree {
    root: Node,
    order: usize,
}

impl BTree {
    fn create(order: usize) -> BTree {
        BTree {
            root: Node::empty(order, true),
            order,
        }
    }

    fn insert(&mut self, key: Key) {
        if self.root.is_full(self.order) {
            let mut new_root = Node::empty(self.order, false);
            new_root.children.push(self.root.clone());
            new_root.split_child(0, self.order);
            self.root = new_root;
        }

        self.root.insert(key, self.order);
    }
}
fn main() {
    let order = 3;
    let mut tree = BTree::create(order);

    tree.root.keys = vec![
        Key { value: 'G' },
        Key { value: 'M' },
        Key { value: 'P' },
        Key { value: 'X' },
    ];

    tree.root.leaf = false;

    tree.root.children = vec![
        {
            let mut child = Node::empty(order, true);
            child.keys = vec![
                Key { value: 'A' },
                Key { value: 'C' },
                Key { value: 'D' },
                Key { value: 'E' },
            ];
            child
        },
        {
            let mut child = Node::empty(order, true);
            child.keys = vec![Key { value: 'J' }, Key { value: 'K' }];
            child
        },
        {
            let mut child = Node::empty(order, true);
            child.keys = vec![Key { value: 'N' }, Key { value: 'O' }];
            child
        },
        {
            let mut child = Node::empty(order, true);
            child.keys = vec![
                Key { value: 'R' },
                Key { value: 'S' },
                Key { value: 'T' },
                Key { value: 'U' },
                Key { value: 'V' },
            ];
            child
        },
        {
            let mut child = Node::empty(order, true);
            child.keys = vec![Key { value: 'Y' }, Key { value: 'Z' }];
            child
        },
    ];

    tree.insert(Key { value: 'B' });
    tree.insert(Key { value: 'Q' });
    tree.insert(Key { value: 'L' });
    tree.insert(Key { value: 'F' });
    tree.insert(Key { value: 'a' });
    tree.insert(Key { value: 'b' });
    tree.insert(Key { value: 'c' });
    tree.insert(Key { value: 'd' });
}
