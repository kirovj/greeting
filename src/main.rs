fn main() {
    let formal = true;
    let greeting = if formal {
        // "if" keyword used here as an expression
        "Good day to you." // Returns the string "Good day to you."
    } else {
        "Hey!" // Returns the string "Hey!"
    };
    println!("{}", greeting) // Prints "Good day to you."
}
