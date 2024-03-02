mod animal;
mod display;
mod dog_enum;
mod match_eg;
mod rnd_name;
mod warrior;
mod number_guess;
mod stdin;
mod file_utils;
mod async_eg;
mod string_eg1;
mod minigrep;
mod print_type_of;
mod string_eg2;
mod jpg_rand;

pub fn main() {
    do_stuff();
}

use crate::animal::animal_enum::{Animal, Reproduction};
use crate::display::point2d::example_output;
use crate::dog_enum::dog_speaks::chat::conversation;
use crate::dog_enum::dog_speaks::Dog;
use crate::match_eg::match_eg1;
use crate::string_eg1::jpg_string::JpgString;
use crate::warrior::person_gender::{Gender, Person};


pub fn do_stuff() {
    let s = JpgString::new(String::from("abcdefg"));
    s.replace("a", "b");

    match_eg1::match_eg1(Some(1));

    let p1 = Person::new(Gender::Male, 55);
    println!("{:?} aged {} says {}",p1.gender, p1.age, p1.chat());

    let a1 = Animal::new("Human".to_string(),
                         2,
                         false,
                         Reproduction::Born);
    println!("{}", a1.desc_repro());


    example_output();

    let mut cnt =3;
    let v = loop { // loop forever
        println!("cnt={}", cnt);
        if cnt<1 {break cnt}; // break and return a value
        cnt=cnt-1;
    };
    println!("v={}", v);

    let a = [1,2,3];
    for el in a {
        println!("el={}", el);
    }

    for rnge in (1..4).rev() { // a range from 1 to 3, reversed
        println!("rnge={}", rnge);
    }

    let foo = {
        let c = true;
        let bar = if c { 3 * 60 } else { 4 * 60 };
        bar / 30 // No ; so this is the return value.
    };
    println!("foo={}", foo);

    //    println!("Hello, world! {}", rnd());
    let d1 = Dog {
        name: String::from("Fido"),
    };
    let d2 = Dog {
        name: String::from("Juno"),
    };
    conversation(&d1, &d2);
}
