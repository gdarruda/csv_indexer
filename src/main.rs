mod index;
mod csv;
use std::time::SystemTime;
use uuid::Uuid;

use std::fs::File;


fn main() -> std::io::Result<()> {

    let filename = "/home/gdarruda/Projects/sandbox/clients.csv";
    let mut file = File::open(filename)?;
    let mut tree = index::btree::BTree::create(1000, "/home/gdarruda/btree_files");
    csv::index_file(&file, &mut tree);

    let tree = index::btree::BTree::load("/home/gdarruda/btree_files");

    for uuid in (0..1000).map(|_| Uuid::new_v4().to_string()) {

        let now = SystemTime::now();

        match tree.search(&uuid) {
            None => {},
            Some(key) => {
                match csv::read_line(&mut file, key.position) {
                    Err(e) => {println!("Error: {}", e)},
                    Ok(line) => {println!("Found line: {}", line)}
                }
            }
        };

        match now.elapsed() {
            Ok(elapsed) => {
                println!("{}", elapsed.as_micros());
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }

    }

    Ok(())
}
