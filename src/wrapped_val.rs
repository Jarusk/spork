#[derive(Debug,Clone)]
pub struct WrappedVal<'a, T: PartialEq + Clone> {
    data_type: &'a str,
    val: T
}

impl<'a, T: PartialEq + Clone> WrappedVal<'a, T> {
    pub fn new(data_type: &'a str, val: T) -> WrappedVal<'a,T> {
        WrappedVal {data_type:data_type, val:val}
    }
    pub fn get_val(&self) -> &T {
        &self.val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_WrappedvVal() {
        let tmp_type = "str";
        let stored_val = "ABC123";
        let tmp_struct = WrappedVal::new(tmp_type, stored_val);
        assert_eq!(tmp_struct.data_type, tmp_type);
    }

    #[test]
    fn get_val() {
        let tmp_struct = WrappedVal::new("int",14);
        assert_eq!(*tmp_struct.get_val(), 14);
    }
}
