// เพื่อน: ขอยืมหนังสือไปดูนะ เดี๋ยวคืน
// เรา:   ได้ ดูได้ แต่อย่าขีดเขียน

fn main() {
    // book คือหนังสือของเรา (เป็นเจ้าของข้อมูล)
    let book = "The Great Gatsby";

    // borrowed_book คือการยืมหนังสือมาอ่าน
    // ใช้ &book หมายถึง Borrowing Immutable
    // อ่านได้อย่างเดียว แก้ไขไม่ได้
    let borrowed_book = &book;

    // ใช้ข้อมูลที่ยืมมาได้ตามปกติ (อ่าน)
    println!("Borrowed book: {}", borrowed_book);

    // เมื่อออกจากฟังก์ชัน main
    // borrowed_book จะหมดอายุการยืม (คืนอัตโนมัติ)
}
