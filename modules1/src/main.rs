use modules1::nesting; // refering to the pub lib.rs function

fn main() {
    nesting(); // no need to have modules1:: on the font because of the above use
}

