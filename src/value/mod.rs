//! This module will hold the generic structs to wrap the arbitrary types being stored

/// A genreal structure to hide the actual content being stored in the KV store
/// At some point, may forgo this. However, right now, provides a good way of forcing
/// stored objects to be on the heap.
pub struct Value<T: Clone> {
    val: Box<T>
}

impl <T:Clone> Value<T> {

    /// Returns a new instance of the 'Value' struct.
    /// Holds the data pointed to by the 'Key'
    pub fn new(store: T) -> Value<T> {
        return Value{val: Box::new(store)};
    }

    /// Returns a Box to the value stored
    pub fn get_val(&self) -> Box<T> {
        return self.val.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_str() {
        let x = Value::new("Hello World!");
    }

    #[test]
    fn get_str() {
        let x = Value::new("Hello World!");
        let y = x.get_val();
        let z = x.get_val();
        assert_eq!(*y,"Hello World!");
        assert_eq!(*z,"Hello World!");
        assert_eq!(*y,*z);
    }

    #[test]
    fn store_String() {
        let x = Value::new("Hello World!".to_string());
    }

    #[test]
    fn get_String() {
        let x = Value::new("Hello World!".to_string());
        let y = x.get_val();
        let z = x.get_val();
        assert_eq!(*y,"Hello World!".to_string());
        assert_eq!(*z,"Hello World!".to_string());
        assert_eq!(*y,*z);
    }

    #[test]
    fn store_int() {
        let x = Value::new(356);
    }

    #[test]
    fn store_float() {
        let x = Value::new(123.456);
    }
}
