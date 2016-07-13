extern crate time;

use std::collections::HashMap;
use wrapped_val::WrappedVal;

#[derive(Clone,Debug)]
pub struct StrMap<'a> {
    created: time::Tm,
    map: HashMap<&'a str, WrappedVal<'a, &'a str>>
}

impl<'a> StrMap<'a> {
    pub fn new() -> StrMap<'a> {
       StrMap {created: time::now(), map: HashMap::new()} 
    }

    pub fn insert(&mut self, key: &'a str, val: &'a str) -> Option<WrappedVal<&'a str>> {
        self.map.insert(key, WrappedVal::new("str", val))
    }

    pub fn get(&self, key: &str) -> Option<&'a str> {
        match self.map.get(key) {
            Some(x) => Some(&x.get_val()),
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
