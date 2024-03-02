use std::error::Error;
use std::io;
use std::io::Write;

pub fn ask_user(question: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", question);
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

#[allow(dead_code)]
pub fn ask_user_for_i32(question: &str) -> i32 {
    loop {
        match ask_user(question) {
            Ok(rep) => match rep.trim().parse::<i32>() {
                Ok(n) => break n,
                Err(_e) => println!(
                    "Your value [{}] was not a number.  Please enter a number.",
                    rep
                ),
            },
            Err(err) => {
                println!(
                    "An unexpected error occurred, exiting.  {}",
                    err.to_string()
                );
                panic!("{}", err.to_string());
            }
        }
    }
}

#[allow(dead_code)]
pub fn ask_user_for_u32(question: &str) -> u32 {
    loop {
        match ask_user(question) {
            Ok(rep) => match rep.trim().parse::<u32>() {
                Ok(n) => break n,
                Err(_e) => println!(
                    "Your value [{}] was not a number.  Please enter a number.",
                    rep
                ),
            },
            Err(err) => {
                println!(
                    "An unexpected error occurred, exiting.  {}",
                    err.to_string()
                );
                panic!("{}", err.to_string());
            }
        }
    }
}

pub fn ask_user_for_u32_with_max(question: &str, max_inclusive: u32) -> u32 {
    loop {
        match ask_user_for_u32(question) {
            n if (n > max_inclusive) => {
                println!("Your value [{n}] exceeds the allowed maximum of [{max_inclusive}]")
            },
            n => break n
        }
    }
}
