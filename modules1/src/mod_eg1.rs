// Because this file is called mod_eg1.rs it is automatically the implementation
// of mod_eg1 and in lib.rs we said mod mod_eg1; which meant the crate knew about it.
mod sub_eg2;

pub fn bar() {
    mod_eg2::nested::foo();
}

mod mod_eg2 {
    pub mod nested {
        use crate::mod_eg1::sub_eg2;

        pub fn foo() {
            sub_eg2::do_stuff();
        }
    }
}