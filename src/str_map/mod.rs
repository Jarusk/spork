extern crate time;

use std::collections::HashMap;

#[derive(Clone,Debug)]
pub struct StrMap<'a> {
    created: time::Tm,
    map: HashMap<&'a str, Box<&'a str>>
}

impl<'a> StrMap<'a> {
    pub fn new() -> StrMap<'a> {
       StrMap {created: time::now(), map: HashMap::new()} 
    }

    pub fn insert(&mut self, key: &'a str, val: &'a str) -> Option<Box<&'a str>> {
        self.map.insert(key,Box::new(val))
    }

    pub fn get(&self, key: &str) -> Option<&'a str> {
        match self.map.get(key) {
            Some(x) => Some(&x),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
