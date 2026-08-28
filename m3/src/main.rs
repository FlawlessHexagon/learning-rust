fn main() {
    let secret_number: i32 = 1023;
    let guess: i32 = 6;

    // in Rust, `if` can produce a value
    // each selected branch’s final expression becomes the value of the entire `if`
    // each branch produces `&str`
    let feedback: &str = if guess < secret_number {
        // the `&str` expects returning values of the `if` branches to be `&str`
        "Too low!"
    } else if guess > secret_number {
        "Too high!"
    } else {
        "Correct!"
    }; // the semicolon ends the let statement

    println!("{feedback}");
}
