// ประกาศ Named Struct
// ใช้เก็บข้อมูลหนังสือ โดยมีชื่อ field ชัดเจน
struct Book {
    title: String,      // ชื่อหนังสือ
    author: String,     // ผู้แต่ง
    pages: u32,         // จำนวนหน้า
    available: bool,    // สถานะว่ายืมได้หรือไม่
}

// ประกาศ Tuple Struct
// ไม่มีชื่อ field ใช้ index แทน
struct Book2(String, String, u32, bool);

fn main() {
    // สร้างตัวแปรสำหรับข้อมูลหนังสือ
    let title = String::from("The Great Gatsby");
    let author = String::from("F. Scott Fitzgerald");
    let pages = 180;
    let available = true;

    // สร้าง instance ของ Named Struct
    // ใช้ field init shorthand (ชื่อ field = ชื่อตัวแปร)
    let book = Book {
        title,
        author,
        pages,
        available,
    };

    // สร้าง instance ของ Tuple Struct
    let my_book = Book2(
        String::from("1984"),         // .0 = title
        String::from("George Orwell"),// .1 = author
        328,                          // .2 = pages
        false,                        // .3 = available
    );

    // เข้าถึงค่าจาก Tuple Struct ด้วย index
    println!("Book title: {}", my_book.0);

    // เข้าถึงค่าจาก Named Struct ด้วยชื่อ field
    println!("Author: {}", book.author);
}
