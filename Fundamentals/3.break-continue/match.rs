//* ตัวอย่างการใช้ enum
// enum คือ การกำหนดชนิดข้อมูลที่สามารถเลือกได้
enum Status {
    Success,
    NotFound,
    Error,
}


fn main() {
    //* match เป็นคำสั่งที่ใช้ในการเลือกค่าจากตัวแปร
    let name = "Chicken";
     
    //* match จะต้องมีทุก case ที่อาจจะเกิดขึ้นได้ match ใน Rust ต้อง ครอบคลุมทุกกรณี
    match name {
        "Hello Chicken" => println!("Hello Chicken"),
        "Chicken" => println!("Chicken"),
        "Hello Cat" => println!("Hello Cat"),
        _ => println!("Hello Other"),
    }


    // ใช้เมื่อ “มีหลายเงื่อนไข” และอยากให้โค้ดอ่านง่าย 
   //* ตัวอย่างการใช้ if else
    let name = "Chicken";
    if name == "Chicken" {
        println!("Chicken");
    } else if name == "Dog" {
        println!("Dog");
    } else if name == "Cat" {
        println!("Cat");   
    }

    // * ตัวอย่างการใช้ match
    let name = "Chicken";
    match name {
        "Chicken" => println!("Chicken"),
        "Dog" => println!("Dog"),
        "Cat" => println!("Cat"),
        _ => println!("Other"),
    }

    // enum คือ การกำหนดชนิดข้อมูลที่สามารถเลือกได้
    
    let statuses = [
    Status::Success,
    Status::NotFound,
    Status::Error,
];

for status in statuses {
    match status {
        Status::Success => println!("Success"),
        Status::NotFound => println!("404"),
        Status::Error => println!("Something went wrong"),
    }
}

    

}