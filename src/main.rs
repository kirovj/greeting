fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    if divisor == 0 {
        println!("\nError! Division by zero is not allowed.");
        // To prevent division by zero, halt execution and return to the caller
        return false;
    } else if dividend % divisor > 0 {
        println!(
            "\n{} % {} has a remainder of {}.",
            dividend,
            divisor,
            (dividend % divisor)
        );
    } else {
        println!("\n{} % {} has no remainder.", dividend, divisor);
    }

    // Create the boolean value and return it to the function caller
    dividend % divisor == 0
}

fn main() {
    is_divisible_by(5, 4);
}
