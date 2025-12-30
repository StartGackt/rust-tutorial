// panic_borrowing_Immutable
// ใช้ Borrowing แบบอ่านอย่างเดียว (&T)
// ถ้าข้อมูลผิดเงื่อนไข จะ panic! เพื่อหยุดโปรแกรม

fn read_book(book: &str) {
    // ตรวจสอบข้อมูลก่อนใช้งาน
    if book.is_empty() {
        // error ร้ายแรง → หยุดโปรแกรมทันที
        panic!("Book is empty, cannot read");
    }

    // อ่านข้อมูลได้ตามปกติ
    println!("Reading book: {}", book);
}

fn main() {
    let book = "Rust Basic";

    // ยืมมาอ่าน (Immutable Borrowing)
    read_book(book);
}
