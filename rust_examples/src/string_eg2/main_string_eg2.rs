use std::collections::HashMap;
use std::rc::Rc;
use crate::print_type_of::print_type_of::print_type_of;
use crate::string_eg2::map_of_strings_reused::{MyCache, MyReuse};
use crate::string_eg2::slice_strings::SliceStrings;
use crate::string_eg2::struct_strings::StructStrings;


fn main_string_eg2() {
    {
        println!("====== A struct holding a string");
        let v1 = StructStrings::new("Fred".to_string(), 42);

        println!("This clones the strings : {}", &v1);
        dbg!(&v1);
        println!("Type of v1 is {}", std::any::type_name::<StructStrings>());
        print_type_of(&v1.name);
        println!("The name is at {:p} with len {}, cap {}", &v1.name, &v1.name.len(), &v1.name.capacity());
    }
    {
        println!("====== A struct holding an &str");
        let string_with_lifetime = "Fred".to_string();
        let v2 = SliceStrings::new(&string_with_lifetime, 42);
        dbg!(&v2);
        println!("Type of v2 is {}", std::any::type_name::<SliceStrings>());
        print_type_of(&v2.name);
        println!("The struct   name is at {:p} with len {}", &v2.name, &v2.name.len());
        println!("The variable name is at {:p} with len {}", &string_with_lifetime, &string_with_lifetime.len());
        println!(r##"The struct   name string is at {:?},
The variable name string is at {:?}"##, &v2.name.as_ptr(), &string_with_lifetime.as_ptr());
    }
    {
        println!("====== Using an Rc (or Arc, etc) to not copy the string over and over.");
        let string_gibbons = "Gibbons".to_string();
        println!("The original string is at {:?}", &string_gibbons.as_ptr());
        let cache = MyCache{
            some_lookup:HashMap::from([
            ("Jonathan".to_string(), Rc::new(string_gibbons)),
            ("Bill".to_string(), Rc::new("Gates".to_string())),
            ("Steve".to_string(), Rc::new("Jobs".to_string())),
        ])};

        let o1 = MyReuse::new(cache.some_lookup.get("Jonathan").unwrap(), 42);
        let o2 = MyReuse::new(cache.some_lookup.get("Jonathan").unwrap(), 42);
        println!(r##"
The cached  Rc<String> is at {:p}
The o1.name Rc<String> is at {:p},
The o2.name Rc<String> is at {:p},
The variable o1.name string is at {:?}
The variable o2.name string is at {:?}
"##, &cache.some_lookup.get("Jonathan").unwrap(), &o1.name, &o2.name, &o1.name.as_ptr(), &o2.name.as_ptr());
    }
}