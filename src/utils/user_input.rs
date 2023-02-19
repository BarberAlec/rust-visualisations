use std::{io, str::FromStr};

fn ask_user_input(msg: &str) -> String {
    let mut user_input = String::new();
    println!("{}", msg);
    io::stdin()
        .read_line(&mut user_input)
        .expect("Bad input reading");

    // remove carriage return and newline, cross platform
    if user_input.ends_with("\n") {
        user_input.pop();
        if user_input.ends_with("\r") {
            user_input.pop();
        }
    }
    user_input
}


/// Ask user question `msg` until user responds with i32 appropaite response
pub fn user_input_with_parse<T: std::str::FromStr>(msg: &str) -> T where <T as FromStr>::Err: std::fmt::Display{
    let int_resp: T;

    // Get user input and parse to i32
    loop {
        let user_input = ask_user_input(msg);

        match user_input.parse::<T>() {
            Ok(i) => {
                int_resp = i;
                break;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    int_resp
}