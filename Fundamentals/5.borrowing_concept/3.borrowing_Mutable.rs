// เพื่อน: ขอยืมหนังสือไปเขียนแก้หน่อยนะ
// เรา:   ได้ แต่ช่วงนี้ห้ามใครมายุ่ง

fn main() {
    // book คือหนังสือของเรา (เป็นเจ้าของข้อมูล)
    let mut book = String::from("Rust Programming");

    // borrowed_book คือการยืมหนังสือมา "แก้ไขได้"
    // ใช้ &mut book หมายถึง Borrowing Mutable
    let borrowed_book = &mut book;

    // แก้ไขข้อมูลได้ (ขีดเขียนได้)
    borrowed_book.push_str(" - Advanced");

    // แสดงผลหลังจากแก้ไข
    println!("Borrowed book: {}", borrowed_book);

    // ระหว่างที่ borrowed_book ถูกยืมแบบ mutable
    // ห้ามมีการยืมแบบอื่น (ทั้ง &book หรือ &mut book ซ้อนกัน)

    // เมื่อออกจาก scope นี้
    // borrowed_book จะถูก "คืน" อัตโนมัติ
}
