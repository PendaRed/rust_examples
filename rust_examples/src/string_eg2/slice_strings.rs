
#[derive(Debug)]
pub struct SliceStrings <'a>{
    pub name: &'a str,
    pub age: u8,
}

impl <'a>SliceStrings<'a> {
    // Lifetime elision rules apply here, ie no 'a needed as it works it out
    pub fn new(name: &str, age: u8) -> SliceStrings {
        SliceStrings { name, age }
    }
}