//! This module defines the keys for our KV pair

pub struct Key {
    val: &'static str,
    stored_type: &'static str,
    created: &'static str
}

impl Key {
    pub fn new(val: &'static str, stored_type: &'static str) -> Key {
        return Key{
                    val: val,
                    stored_type: stored_type,
                    created: "Now"
                };
    }

    pub fn get_created(&self) -> &'static str {
        return self.created;
    }

    pub fn get_type(&self) -> &'static str {
        return self.stored_type;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_key() {
        let x = Box::new(Key::new("user.test","u64"));
        assert_eq!(x.stored_type, "u64");
    }

    #[test]
    fn get_key_creation() {
        let x = Box::new(Key::new("user.test","u64"));
        assert_eq!(x.get_created(), "Now");
    }

    #[test]
    fn get_key_stored_type() {
        let x = Box::new(Key::new("user.test", "i32"));
        assert_eq!(x.get_type(), "i32");
    }
}
