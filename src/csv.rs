use crate::index::btree::BTree;
use crate::index::key::Key;
use std::error;
use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};

fn get_key(posic: usize, buf: &String) -> Option<&str> {
    for (i, col) in buf.split(",").enumerate() {
        if i == posic {
            return Some(col);
        }
    }
    None
}

pub fn index_file(file: &File, tree: &mut BTree) {
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    let mut offset: u64 = 0;

    loop {
        buf.clear();

        let size: u64 = reader
            .read_line(&mut buf)
            .expect("reading from cursor shouldn't fail")
            .try_into()
            .unwrap();

        if size == 0 {
            break;
        }

        let key_value = match get_key(0, &buf) {
            None => {
                offset += size;
                continue;
            }
            Some(value) => value,
        };

        tree.insert(Key::create(key_value, (offset, size)));
        offset += size;
    }
}

pub fn read_line(file: &mut File, position: (u64, u64)) -> Result<String, Box<dyn error::Error>> {
    let (start, offset) = position;
    file.seek(SeekFrom::Start(start))?;

    let mut read_buf = vec![0; offset.try_into().unwrap()];
    file.read_exact(&mut read_buf)?;

    match String::from_utf8(read_buf) {
        Err(e) => Err(Box::new(e)),
        Ok(line) => Ok(line),
    }
}

mod tests {

    use crate::csv;
    use crate::index::btree::BTree;
    use std::fs::File;
    use std::fs;

    #[test]
    fn get_key() {
        let buf = String::from("10,20,30");

        match csv::get_key(1, &buf) {
            None => {
                panic!("Key not found");
            }
            Some(key) => {
                assert_eq!(key, "20");
            }
        };

        match csv::get_key(3, &buf) {
            None => {}
            Some(_) => {
                panic!("Key shouldn't be found");
            }
        };
    }

    #[test]
    fn index_file() {

        let filename = "resources/sample.csv";
        let tree_path  = "csv_test_index_file";

        match File::open(filename) {
            Err(_) => {panic!("Can't open {}!", filename);},
            Ok(file) => {
                let mut tree = BTree::create(3, tree_path);
                
                csv::index_file(&file, &mut tree);

                match tree.search("10") {
                    None => {panic!("10 not found!")},
                    Some(key) => { assert_eq!(key.position, (0, 9))}
                };

                match tree.search("11") {
                    None => {},
                    Some(_) => {panic!("Key shouldn't be found");}
                };

                match tree.search("20") {
                    None => {panic!("20 not found!")},
                    Some(key) => { assert_eq!(key.position, (27, 6))}
                };
            }
        }

        fs::remove_dir_all(tree_path).unwrap();
    }

    #[test]
    fn read_line() {
        let filename = "resources/sample.csv";
        let tree_path  = "csv_test_load_line";

        match File::open(filename) {
            Err(_) => {panic!("Can't open {}!", filename);},
            Ok(mut file) => {

                let mut tree = BTree::create(3, tree_path);
                csv::index_file(& file, &mut tree);

                match tree.search("10") {
                    None => {panic!("10 not found!")},
                    Some(key) => {
                        match csv::read_line(&mut file, key.position) {
                            Err(_) => {panic!("Can't read line 10");},
                            Ok(line) => {assert_eq!(line, "10,20,30\n")}
                    }}
                };
            }
        }

        fs::remove_dir_all(tree_path).unwrap();
    }
}
