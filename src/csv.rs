use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};
use std::error;
use crate::index::btree::BTree;
use crate::index::key::Key;


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
        Err(e) => {Err(Box::new(e))}
        Ok(line) => Ok(line)
    }
}
