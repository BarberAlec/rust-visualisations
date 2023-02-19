use std::fmt::Error;

pub fn get_fibonacci(cnt: i32) -> Result<Vec<i32>, Error> {
    if cnt < 0 {
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