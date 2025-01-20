mod index;

use crate::index::btree::BTree;
use crate::index::key::Key;

fn main() {
    let order =3;
    let mut tree = BTree::create(order);

    // tree.root.keys = vec![
    //     Key { value: 'G'.to_string() },
    //     Key { value: 'M'.to_string() },
    //     Key { value: 'P'.to_string() },
    //     Key { value: 'X'.to_string() },
    // ];

    // tree.root.leaf = false;

    // tree.root.children = vec![
    //     {
    //         let mut child = Node::empty(order, true);
    //         child.keys = vec![
    //             Key { value: 'A' },
    //             Key { value: 'C' },
    //             Key { value: 'D' },
    //             Key { value: 'E' },
    //         ];
    //         child
    //     },
    //     {
    //         let mut child = Node::empty(order, true);
    //         child.keys = vec![Key { value: 'J' }, Key { value: 'K' }];
    //         child
    //     },
    //     {
    //         let mut child = Node::empty(order, true);
    //         child.keys = vec![Key { value: 'N' }, Key { value: 'O' }];
    //         child
    //     },
    //     {
    //         let mut child = Node::empty(order, true);
    //         child.keys = vec![
    //             Key { value: 'R' },
    //             Key { value: 'S' },
    //             Key { value: 'T' },
    //             Key { value: 'U' },
    //             Key { value: 'V' },
    //         ];
    //         child
    //     },
    //     {
    //         let mut child = Node::empty(order, true);
    //         child.keys = vec![Key { value: 'Y' }, Key { value: 'Z' }];
    //         child
    //     },
    // ];

    tree.insert(Key::create("B"));
    tree.insert(Key::create("Q"));
    tree.insert(Key::create("L"));
    tree.insert(Key::create("F"));
    tree.insert(Key::create("a"));
    tree.insert(Key::create("b"));
    tree.insert(Key::create("c"));
    tree.insert(Key::create("d"));

}
