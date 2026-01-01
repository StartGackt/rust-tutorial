// struct คือ ข้อตกลง ว่าเราจะเก็บข้อมูลอะไรบ้าง (เหมือนหัวข้อในใบงาน)
// instance structs คือ ใบที่กรอกข้อมูลเสร็จแล้ว
// 
// 1. กำหนด struct ชื่อ Book
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

fn main() {
    // 2. สร้างตัวแปร (Instance)
    let book = Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
        pages: 180,
        available: true,
    };

    // 3. เข้าถึงข้อมูล
    println!("Title: {}", book.title);
    println!("Available: {}", book.available);
}