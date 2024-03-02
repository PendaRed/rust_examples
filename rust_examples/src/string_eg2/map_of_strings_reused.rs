use std::collections::HashMap;
use std::rc::Rc;

pub struct MyCache {
   pub some_lookup: HashMap<String, Rc<String>>
}

pub struct MyReuse {
    pub name: Rc<String>,
    age: u8,
}

impl MyReuse {
    pub fn new(name: &Rc<String>, age: u8) -> MyReuse {
        MyReuse { name:name.clone(), age }
    }
}