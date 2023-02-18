
mod engines;
use engines::fibonacci;


fn main() {
    
    // let fib_cnt = fibonacci::user_input_with_parse();
    
    // println!("Printing {} fibonacci numbers...", fib_cnt);

    fibonacci::execute_fibonacci();
    
}
