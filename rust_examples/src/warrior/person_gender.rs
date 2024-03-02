
#[derive(Debug)]
pub enum Gender {
    Male,
    Femail
}

#[derive(Debug)]
pub struct Person {
    pub gender: Gender,
    pub age: u32,
}

impl Person {
    pub fn new(gender: Gender, age:u32) -> Person{
        Person{gender, age}
    }

    pub fn chat(&self) -> String {
        String::from("Hi")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Gender::*;

    #[test]
    fn chat_test() {
        let p1 = Person::new(Male, 55);
        assert_eq!(p1.chat(), "Hi");
    }
}
