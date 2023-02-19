use std::{fmt::Error, io};



fn ask_user_input() -> String {
    let mut user_input = String::new();
    println!("How many fibonacci numbers would you like to print?");
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

fn user_input_with_parse() -> i32 {
    let fib_count: i32;

    // Get user input and parse to i32
    loop {
        let user_input = ask_user_input();

        match user_input.parse::<i32>() {
            Ok(i) => {
                fib_count = i;
                break;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    fib_count
}

pub fn get_fibonacci(cnt: i32) -> Result<Vec<i32>, Error> {
    if cnt < 0 || cnt > 47 {
        // cant have negative counts
        return Err(Error);
    }
    if cnt > 47 {
        // cannot have a count which results in an i32 overflow
        return Err(Error);
    }
    let mut fib_vec: Vec<i32> = Vec::new();
    if cnt == 0 {
        return Ok(fib_vec);
    }
    fib_vec.push(0);
    if cnt == 1 {
        return Ok(fib_vec);
    }
    fib_vec.push(1);
    if cnt == 2 {
        return Ok(fib_vec);
    }
    fib_vec.push(1);

    for i in 3..cnt as usize {
        fib_vec.push(fib_vec[i - 1] + fib_vec[i - 2]);
    }

    Ok(fib_vec)
}

#[allow(dead_code)]
pub fn execute_fibonacci() {
    let fib_cnt = user_input_with_parse();
    let fib_nums = get_fibonacci(fib_cnt).expect(&format!(
        "Fibinanci function failed with count input {}",
        fib_cnt
    ));

    println!("{:?}", fib_nums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_get_0() {
        assert_eq!(get_fibonacci(0), Ok(vec![]));
        assert_eq!(get_fibonacci(1), Ok(vec![0]));
    }

    #[test]
    fn fibonacci_get_1() {
        assert_eq!(get_fibonacci(1), Ok(vec![0]));
    }
    
    #[test]
    fn fibonacci_get_5() {
        assert_eq!(get_fibonacci(5), Ok(vec![0, 1, 1, 2, 3]));
    }

    #[test]
    fn fibonacci_get_negative() {
        assert_eq!(get_fibonacci(-1), Err(Error));
    }


}