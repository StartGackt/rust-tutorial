// Using panic! to stop a program
//
// panic! คือคำสั่งที่ใช้ "หยุดโปรแกรมทันที"
// ใช้เมื่อเกิดข้อผิดพลาดร้ายแรง และไม่ควรให้โปรแกรมทำงานต่อ

fn panic_basic() {

    println!("Program start");

    // ถ้าเงื่อนไขนี้เกิดขึ้น โปรแกรมจะหยุดทันที
    panic!("Something went wrong!");

    // โค้ดด้านล่างนี้จะไม่ถูกรัน
    println!("Program end");
}


// ตัวอย่าง: ใช้ Borrowing ร่วมกับ panic!
//
// แนวคิด:
// - Borrowing = ยืมข้อมูลมาใช้ชั่วคราว (ไม่เอา ownership)
// - panic! = หยุดโปรแกรมทันที เมื่อเกิดข้อผิดพลาดร้ายแรง

fn read_book(book: &str) {
    // Borrowing Immutable: ยืมมาอ่านอย่างเดียว
    if book.is_empty() {
        // ถ้าหนังสือว่าง ถือว่าเป็น error ร้ายแรง
        panic!("Book is empty, cannot read");
    }

    println!("Reading book: {}", book);
}

fn write_book(book: &mut String) {
    // Borrowing Mutable: ยืมมาแก้ไขได้
    if book.is_empty() {
        panic!("Book is empty, cannot write");
    }

    book.push_str(" (edited)");
    println!("Writing book: {}", book);
}

fn main() {
    // ตัวอย่าง Borrowing Immutable
    let book1 = "Rust Basic";
    read_book(book1); // ยืมมาอ่าน

    // ตัวอย่าง Borrowing Mutable
    let mut book2 = String::from("Rust Advanced");
    write_book(&mut book2); // ยืมมาเขียน

    println!("Final book2: {}", book2);
}
