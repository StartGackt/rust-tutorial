fn main() {
    println!("Functions lesson");
    let result = add_numbers(5, 3);
    println!("5 + 3 = {}", result);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}