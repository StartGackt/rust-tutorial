use std::io;

fn main() {
    loop {
        println!("\n=== Rust Tutorial Menu ===");
        println!("1. Variables Demo");
        println!("2. Functions Demo");
        println!("3. Loops Demo");
        println!("4. Exit");
        println!("Choose option (1-4): ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "1" => variables_demo(),
            "2" => functions_demo(),
            "3" => loops_demo(),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option!"),
        }
    }
}

fn variables_demo() {
    println!("\n--- Variables Demo ---");
    let message = "Weight Converter";
    let weight = 150.0;
    let kilos = weight / 2.2;

    println!("{}", message);
    println!("Weight: {:.1} lbs = {:.1} kg", weight, kilos);
}

fn functions_demo() {
    println!("\n--- Functions Demo ---");
    let result = add_numbers(10.0, 5.0);
    println!("10 + 5 = {}", result);
}

fn add_numbers(a: f64, b: f64) -> f64 {
    a + b
}

fn loops_demo() {
    println!("\n--- Loops Demo ---");
    let mut i = 0;
    while i < 10 {
        println!("i = {}", i);
        i += 1;
    }
}
    