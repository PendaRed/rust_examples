use std::fmt;

/// This struct will clone the name on every instance.  Strings live on heap with capacity and length.
/// But say the names are already stored in a directory, and we want to re-use the same instance, then
/// this would be slower and more memory intensive.
#[derive(Debug)]
pub struct StructStrings {
    pub name: String,
    pub age: u8,
}

// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for StructStrings {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} is {} years old.", self.name, self.age)
    }
}

impl StructStrings {
    pub fn new(name: String, age: u8) -> Self {
        StructStrings { name, age }
    }
}
