fn main() {
    const FREEZING_POINT_C: f32 = 0.0; // 32 bit floating-point type
    let celsius: f32 = -16.0; // 32 bit floating-point type
    let farenheit: f32 = celsius * 9.0 / 5.0 + 32.0;
    let mut count: u32 = 0; // unsigned 32 bit integer; 0 - 4,294,967,295
    let is_below_freezing: bool = celsius < FREEZING_POINT_C;
    let units: [char; 2] = ['C', 'F']; // fixed-size array containing two 'char' unit symbols
    let conversion: (f32, f32) = (celsius, farenheit); // tuple contatining two 'f32'

    let status_label: &str = "Below freezing"; // a view of text stored somewhere else. It does not own that text and cannot grow it.
    // String -> type to create
    // :: -> select an operation belonging to that type
    // from -> create a String from another value
    // () -> the &str input
    let title: String = String::from("Temperature converter"); // a value that owns its text storage. It can grow or change when used through a mutable variable.
    let line: String = String::from("---------------------");

    count += 1;

    println!("{line}");
    println!("{title}");
    println!("{line}");
    println!(
        "Tuple conversion: {}°{} -> {}°{}",
        conversion.0, units[0], conversion.1, units[1]
    );
    println!("{}: {}", status_label, is_below_freezing);
    println!("Conversion count: {}", count);
}
