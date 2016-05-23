//! This module will hold the generic structs to wrap the arbitrary types

use std::collections::HashMap;

struct Key {
    val: String,
    created: String
}

struct Dictionary {
    store: HashMap<String, String>

}

#[test]
fn box_check() {
    let x = Box::new(6);
    assert_eq!(6,*x);
}

#[test]
fn store_test() {
    let x = Box::new(Key{val:"Matt".to_string(), created:"Today".to_string()});
    assert_eq!("Matt",x.val);
}
