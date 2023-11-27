use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;

fn main() {
    println!("Get the N-th fibonacci number");
    println!("Enter N:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u64 = match input.trim().parse() {
        Ok(num) => num, // Parse the user input into an unsigned 64-bit integer
        Err(_) => {
            println!("Invalid input");
            return; // Exit the program if parsing fails
        }
    };

    let result = nth_fibonacci(n); // Call the nth_fibonacci function with the user input as the argument

    fn nth_fibonacci(n: u64) -> BigUint {
        let mut memo = vec![BigUint::zero(); (n + 1) as usize]; // Create a vector to store the fibonacci numbers
        memo[0] = BigUint::zero(); // Set the first fibonacci number to 0
        memo[1] = BigUint::one(); // Set the second fibonacci number to 1

        for i in 2..=n {
            memo[i as usize] = &memo[(i - 1) as usize] + &memo[(i - 2) as usize];
            // Calculate the fibonacci number at index i
        }

        memo[n as usize].clone() // Return the nth fibonacci number
    }

    println!("{}th fibonacci number is {}", n, result); // Print the result of the fibonacci calculation
}
