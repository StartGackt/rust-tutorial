// จากไฟล์ Associated_functions_structs.rs เราสามารถเพิ่ม constructor ได้ โดยไฟล์ก่อนหน้านั้นจะเน้นเชิงอธิบาย 

// หน้าที่ของ constructor คือ สร้าง Book เล่มใหม่ และคืนค่าออกมา

// ส่วนแรก  กำหนดว่าหนังสือ 1 เล่มต้องมีข้อมูลอะไรบ้าง ยังไม่ได้เป็นหนังสือจริง 
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

//ส่วนที่สอง impl Book = บอกว่าต่อไปนี้คือความสามารถของ Book

impl Book {
    // ส่วนที่ 3 : fn new  เป็น constructor
    //เป็น Associated Function (ไม่มี self)  ไม่มี self คือ 
    //ใช้สำหรับสร้าง Book เล่มใหม่
    fn new(title: String, author: String, pages: u32, available: bool) -> Book {
        // ส่วนที่สี่ Book { title, author, pages, available } = บอกว่าต่อไปนี้คือการสร้าง Book เล่มใหม่
        Book { title, 
            author, 
            pages, 
            available : available,
        }
    }
}

fn main() {
    let new_book = Book::new(
        String::from("Thai"),
        String::from("OTA DEV"),
        100,
        true,
    );
    println!("Book title: {}", new_book.title);
    println!("Book author: {}", new_book.author);
    println!("Book pages: {}", new_book.pages);
    println!("Book available: {}", new_book.available);
}