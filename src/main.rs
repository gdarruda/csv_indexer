mod index;
mod csv;

use std::fs::File;

fn main() -> std::io::Result<()> {

    let filename = "resources/sample.csv";
    let mut file = File::open(filename)?;
    let mut tree = index::btree::BTree::create(3);

    csv::index_file(&file, &mut tree);

    let key_value = "10";

    match tree.search(key_value) {
        None => {println!("Key not foundL {}", key_value)},
        Some(key) => {
            match csv::read_line(&mut file, key.position) {
                Err(e) => {println!("Error: {}", e)},
                Ok(line) => {println!("Found line: {}", line)}
            }
        }
    };

    Ok(())
}
