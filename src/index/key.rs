#[derive(Clone)]
pub struct Key {
    pub value: String,
}

impl Key {
    pub fn create(key: &str) -> Key {
        Key{value: key.to_string()}
    }
}