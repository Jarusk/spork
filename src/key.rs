//! This module defines the keys for our KV pair

extern crate time;

/// A struct to store the key and it's metadata
pub struct Key {
    val: &'static str,
    stored_type: &'static str,
    created: time::Tm
}

impl Key {

    /// Creates a new instance of 'Key' given the supplied label of the key
    pub fn new(val: &'static str, stored_type: &'static str) -> Key {
        return Key{
                    val: val,
                    stored_type: stored_type,
                    created: time::now()
                };
    }

    /// Returns the indexed label of the given key
    pub fn get_label(&self) -> &'static str {
        return self.val.clone();
    }

    /// Returns the time datetime of creation of the key
    pub fn get_created(&self) -> time::Tm {
        return self.created.clone();
    }

    /// Returns the type of the value the key points to
    pub fn get_type(&self) -> &'static str {
        return self.stored_type.clone();
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate time;

    #[test]
    fn create_key() {
        let x = Box::new(Key::new("user.test","u64"));
        assert_eq!(x.stored_type, "u64");
    }

    #[test]
    fn get_key_creation() {
        let x = Box::new(Key::new("user.test","u64"));
        let result = time::now() - x.get_created();
        assert!(result.num_milliseconds() < 100);
    }

    #[test]
    fn get_key_stored_type() {
        let x = Box::new(Key::new("user.test", "i32"));
        assert_eq!(x.get_type(), "i32");
    }

    #[test]
    fn get_label() {
        let x = Box::new(Key::new("user.test", "i32"));
        assert_eq!(x.get_label(), "user.test");
    }
}
