//! This module defines the keys for our KV pair

extern crate time;

use std::hash::{Hash, Hasher};

/// A struct to store the key and it's metadata
pub struct Key<'a> {
    name: &'a str,
    created: time::Tm,
    updated: time::Tm,
}

impl<'a> Key<'a> {
    /// Creates a new instance of 'Key' given the supplied label of the key
    pub fn new(name: &'a str) -> Key<'a> {
        Key {
            name: name,
            created: time::now(),
            updated: time::now(),
        }
    }

    /// Returns the indexed label of the given key
    pub fn get_label(&self) -> &'a str {
        self.name
    }

    /// Returns the time datetime of creation of the key
    pub fn get_created(&self) -> &time::Tm {
        &self.created
    }

    /// Returns the datetime of last key/value update
    pub fn get_updated(&self) -> &time::Tm {
        &self.updated 
    }

    /// Updates the update_tm of the key to relfect a change in 
    /// the Value it links to
    pub fn update_tm(&mut self) {
        self.updated = time::now();
    }
}

impl<'a> Hash for Key<'a> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write(self.name.as_bytes());
        hasher.finish();
    }
}

impl<'a> PartialEq for Key<'a> {
  fn eq(&self, other: &Key) -> bool {
      self.name == other.name
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate time;

    #[test]
    fn create_key() {
        let x = Key::new("user.test");
        assert_eq!(x.name, "user.test");
    }

    #[test]
    fn get_key_creation() {
        let x = Key::new("user.test");
        let result = time::now() - *x.get_created();
        assert!(result.num_milliseconds() < 100);
    }

    #[test]
    fn get_label() {
        let x = Key::new("user.test");
        assert_eq!(x.get_label(), "user.test");
    }
}
