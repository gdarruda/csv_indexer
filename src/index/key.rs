#[derive(Clone)]
pub struct Key {
    pub value: String,
    pub position: (u64, u64),
}

impl Key {
    pub fn create(value: &str, position: (u64, u64)) -> Key {
        Key {
            value: value.to_string(),
            position,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn create() {
        let key = Key::create("Sample", (10, 20));
        assert_eq!(key.value, "Sample");
        assert_eq!(key.position, (10, 20));
    }
}
