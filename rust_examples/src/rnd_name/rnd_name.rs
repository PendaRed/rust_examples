//use std::io;
use rand::{Rng, prelude::ThreadRng};
//use rand::distributions::{Distribution, Uniform};

struct NameGenerator<'a> {
    rnd: ThreadRng,
    prefixes: Vec<&'a str>,
    mids: Vec<&'a str>,
    postfixes: Vec<&'a str>,
}

impl <'a>NameGenerator<'_> {
    fn new() ->  Self  {
        let r = rand::thread_rng();
        let pre = vec!["Jo", "Al", "El", "Ka"];
        let mid = vec!["Jo", "Al", "El", "Ka"];
        let post = vec!["Jo", "Al", "El", "Ka"];
        NameGenerator { rnd:r, prefixes:pre, mids:mid, postfixes:post}
    }
}

pub fn rnd() -> u32 {
    let r = rand::thread_rng();
    let n = rand::thread_rng().gen_range(1..10);
    println!("{}", n);
    n
}