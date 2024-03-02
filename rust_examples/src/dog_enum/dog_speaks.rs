pub trait Speaker {
    fn exclaim(&self) -> String {
        String::from("!") // default implementation
    }
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
}

// I will use the default function for exclaim
impl Speaker for Dog {
    fn speak(&self) -> String {
        // If you miss off the ; and its the last line then its a return statement
        String::from("Woof")
    }
}

pub mod chat {
    use crate::dog_enum::dog_speaks::Speaker;
    pub fn conversation(sp1: &impl Speaker, sp2: &impl Speaker) {
        println!("Speaker 1 says {}", sp1.speak());
        println!("Speaker 2 exclaims {}", sp2.exclaim());
    }
}
