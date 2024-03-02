
fn process_digit(x:i32) {

}

fn process_onetwo(x:i32) {
    
}
fn process_wierd(x:i32) {
    
}
fn process_other(x:i32) {
    
}

pub fn match_eg1(maybe_digit : Option<i32>) {

    let message = match maybe_digit {
        Some(x) if x > 10 => process_digit(x),  // Notice the pattern guard - the if
        Some(x @ 1) | Some(x @ 2) => process_onetwo(x), // Alias to x and or expression
        Some(z @ 3) | Some(z) if z%2==0 => process_wierd(z), // guard could execute many times.
        Some(x) => process_other(x),
        None => panic!(),
    };
}