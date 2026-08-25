fn main() {
    let temp = 16;
    let mut temp_mutable = 16;
    println!("Outer immutable before: {temp}");
    println!("Outer mutable before: {temp_mutable}");

    {
        let temp = 32;
        temp_mutable = 32;
        println!("Inner immutable: {temp}");
        println!("Inner mutable: {temp_mutable}");
    }

    println!("Outer immutable after: {temp}");
    println!("Outer mutable after: {temp_mutable}");
}
