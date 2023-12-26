use std::collections::HashMap; //I would use a hash map!
pub struct QueryString<'buf>{
    data: HashMap<&'buf str, Value<'buf>>,
}


pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString {
    pub fn get(&self, key: &str) -> Option<&Value>{
        self.data.get(key)
    }
}
