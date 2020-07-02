// Find n-th element of Fibonacci sequence
use std::io;

fn main() {
    println!("Hi! Please enter a number of Fibonacci element:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("error: {}", error);
            0
        }
    };

    let fib_number: u128 = fibonacci(n);
    println!("result: {}", &fib_number)
}

fn fibonacci(n: u32) -> u128 {
    let mut current_element: u128 = 0;
    let mut next_element: u128 = 1;
    let mut buffer: u128;
    for _ in 0..n {
        buffer = next_element;
        next_element = buffer + current_element;
        current_element = buffer;
    }
    current_element
}
