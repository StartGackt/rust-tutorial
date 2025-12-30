fn main() {
    let message = "Variables lesson";
    let weight = 150.0;
    let kilos = weight / 2.2;
    
    println!("{}", message);
    println!("Weight: {:.1} lbs = {:.1} kg", weight, kilos);
}