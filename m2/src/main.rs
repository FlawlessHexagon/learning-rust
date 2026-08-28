fn print_header() {
    // `String` -> type to create
    // `::` -> select an operation belonging to that type
    // `from` -> create a String from another value
    // `()` -> the &str input
    let title: String = String::from("Temperature converter"); // a value that owns its text storage. It can grow or change when used through a mutable variable.
    let line: String = String::from("---------------------");

    println!("{line}");
    println!("{title}");
    println!("{line}");
}

fn print_status(label: &str, status: bool) {
    // a function without `-> Type` implicitly returns `()`
    println!("{label}: {status}");
}

fn celsius_input() -> f32 {
    println!("Enter a Celsius value:");
    let mut raw_input: String = String::new(); // create an empty `String`
    // `std` is standard Rust library; `io` is its input/output module
    // `stdin()` accesses standard input
    // `read_line()` waits until the user enters text and presses Enter
    // `&mut` permits `read_line` to modify `raw_input`; borrowing is taught in M4
    // `.expect()` stops the program if reading fails
    std::io::stdin()
        .read_line(&mut raw_input)
        .expect("Failed to read input");
    // `.trim()` removes surrounding whitespace before parsing
    let celsius: f32 = raw_input.trim().parse().expect("Celsius must be a number"); // `celsius: f32` determines Type to convert in `.parse()`
    return celsius;
}

fn c_to_f(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0 // final-expression; the missing semicolon makes this the block's value
}

fn main() {
    print_header();

    const FREEZING_POINT_C: f32 = 0.0; // 32 bit floating-point type
    let mut count: u32 = 0; // unsigned 32 bit integer; 0 - 4,294,967,295
    let units: [char; 2] = ['C', 'F']; // fixed-size array containing two 'char' unit symbols
    let conversion: (f32, f32) = {
        let celsius: f32 = celsius_input();
        let farenheit: f32 = c_to_f(celsius); // accessible only within scope as a single variable
        (celsius, farenheit)
    }; // tuple contatining two 'f32'
    let is_below_freezing: bool = conversion.0 < FREEZING_POINT_C;
    let status_label: &str = "Below freezing"; // a view of text stored somewhere else. It does not own that text and cannot grow it.

    count += 1;

    println!(
        "Tuple conversion: {}°{} -> {}°{}",
        conversion.0, units[0], conversion.1, units[1]
    );
    print_status(status_label, is_below_freezing);
    println!("Conversion count: {}", count);
}
