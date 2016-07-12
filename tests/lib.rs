extern crate spork;

use spork::Spork;


#[test]
fn it_works() {
    assert!(1 > 0);
}

#[test]
fn test_hash_map() {
    let mut map = Spork::new();
    map.insert("Daniel", "798-1364");
    map.insert("Daniel", "164-6743");
    assert_eq!(map.get("Daniel").unwrap(),"164-6743");
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_hash_map_should_fail() {
    let mut map = Spork::new();
    map.insert("Daniel", "798-1364");
    map.insert("Daniel", "164-6743");
    assert_eq!(map.get("Daniel").unwrap(),"798-1364");
}
