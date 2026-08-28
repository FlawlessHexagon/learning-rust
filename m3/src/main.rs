fn main() {
    let secret_number: u32 = 1023;

    // use `while` when repetition depends on a changing Boolean condition.
    // use `for` when processing known values such as a range.
    for preparation_step in 1..4 {
        println!("Preparing game: step {preparation_step}");
    }

    let mut count: u32 = 0;
    let attempts: u32 = loop {
        // a loop can return a value at `break`
        count += 1;
        println!("Attempt: {count}");

        println!("Guess:");
        let mut raw_input: String = String::new();
        std::io::stdin()
            .read_line(&mut raw_input)
            .expect("Failed to read input");
        let guess: u32 = raw_input
            .trim()
            .parse()
            .expect("Guess must be a natural number");

        if guess == 0 {
            println!("Zero is not allowed. Try again.");
            continue;
        }

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

        if guess == secret_number {
            break count;
        }
    };

    println!("Solved in {attempts} attempts!");
}
