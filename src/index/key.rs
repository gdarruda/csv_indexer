#[derive(Clone)]
pub struct Key {
    pub value: String,
}

impl Key {
    pub fn create(key: &str) -> Key {
        Key{value: key.to_string()}
    }
}

mod tests {
    use super::*;

    #[test]
    fn create() {
        let key = Key::create("Sample");
        assert_eq!(key.value, "Sample");
    }
}
