#[derive(Debug,Clone)]
pub struct WrappedVal<'a,T> {
    data_type: &'a str,
    val: Box<T>
}

impl<'a, T> WrappedVal<'a, T> {
    pub fn new(data_type: &'a str, val: T) -> WrappedVal<'a,T> {
        WrappedVal {data_type:data_type, val:Box::new(val)}
    }
    pub fn get_val(&self) -> &T {
        &self.val
    }
}
