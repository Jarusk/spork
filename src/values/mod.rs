//! This module will hold the generic structs to wrap the arbitrary types

struct Store {
    name: String
}

#[test]
fn box_check() {
    let x = Box::new(6);
    assert_eq!(6,*x);
}

#[test]
fn store_test() {
    let x = Box::new(Store {name: "Matt".to_string()});
    assert_eq!("Matt",x.name);
}
