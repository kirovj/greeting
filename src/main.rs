fn main() {
    let x = {
        let y = 1; // first statement
        let z = 2; // second statement
        y + z // this is the *tail* - what the whole block will evaluate to
    };
    println!("{}", x);
}