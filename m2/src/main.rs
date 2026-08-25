fn main() {
    const FREEZING_POINT_C: i32 = 0; // signed 32 bit integer; -2,147,483,648 - 2,147,483,647
    const FREEZING_POINT_AS_FLOAT_C: f32 = FREEZING_POINT_C as f32; // type conversion from i32 to f32
    let celsius: f32 = -16.0; // 32 bit floating-point type
    let mut conversions: u32 = 0; // unsigned 32 bit integer; 0 - 4,294,967,295
    println!("Freezing point (C): {FREEZING_POINT_C}");
    println!("Freezing point as float (C): {FREEZING_POINT_AS_FLOAT_C}");
    println!("Celsius input: {celsius}");

    let farenheit: f32 = celsius * 9.0 / 5.0 + 32.0;
    let farenheit_as_integer: i32 = farenheit as i32; // type conversion from f32 to i32; lossy conversion
    let is_below_freezing: bool = celsius < FREEZING_POINT_AS_FLOAT_C;
    conversions += 1;

    println!("Farenheit output: {farenheit}");
    println!("Farenheit output as integer: {farenheit_as_integer}");
    println!("Below freezing: {is_below_freezing}");
    println!("Conversions: {conversions}");

    let input_unit: char = 'C';
    let output_unit: char = 'F';
    println!("Conversion: {celsius}°{input_unit} -> {farenheit}°{output_unit}");
}
