// struct คือโครงสร้างข้อมูล (Data Structure)
// ใช้สำหรับรวมข้อมูลหลาย ๆ ค่า ที่เกี่ยวข้องกัน
// ให้อยู่ภายใต้ชื่อเดียวกัน
// เปรียบเหมือน "แบบฟอร์ม" หรือ "ข้อมูล 1 ชุด"

// กำหนด struct ชื่อ User
// ใช้เก็บข้อมูลของผู้ใช้ 1 คน
struct User {
    name: String,   // ชื่อผู้ใช้
    age: u8,        // อายุ
    email: String,  // อีเมล
}

fn main() {
    // สร้างตัวแปร user จาก struct User
    // เปรียบเหมือนการกรอกข้อมูลลงในแบบฟอร์ม User
    let user = User {
        name: String::from("John"),
        age: 20,
        email: String::from("john@example.com"),
    };

    // เข้าถึงข้อมูลใน struct ด้วยเครื่องหมาย .
    println!("User name: {}", user.name);
    println!("User age: {}", user.age);
    println!("User email: {}", user.email);
}
