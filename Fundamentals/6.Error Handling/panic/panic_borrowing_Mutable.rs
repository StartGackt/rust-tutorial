// panic_borrowing_Mutable
// ใช้ Borrowing แบบแก้ไขได้ (&mut T)
// ถ้าข้อมูลผิดเงื่อนไข จะ panic! เพื่อหยุดโปรแกรม

fn write_book(book: &mut String) {
    // ตรวจสอบก่อนแก้ไข
    if book.is_empty() {
        // error ร้ายแรง → หยุดโปรแกรม
        panic!("Book is empty, cannot write");
    }

    // แก้ไขข้อมูลได้
    book.push_str(" (edited)");
    println!("Writing book: {}", book);
}

fn main() {
    let mut book = String::from("Rust Advanced");

    // ยืมมาเขียน (Mutable Borrowing)
    write_book(&mut book);

    println!("Final book: {}", book);
}
