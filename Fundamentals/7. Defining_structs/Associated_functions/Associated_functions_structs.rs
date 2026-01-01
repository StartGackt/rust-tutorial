// Associated functions คือ ฟังก์ชันที่เกี่ยวข้องกับ struct
// เปรียบเหมือนการประกาศฟังก์ชันที่เกี่ยวข้องกับ class ในภาษา OOP

// กำหนด struct ชื่อ Book 
// Method (แบบปกติ มี self) 
   // การยืมหนังสือ  ถือหนังสือ" (Instance) มาก่อน ถึงจะยืมเล่มนั้นได้
// Associated Function (แบบไม่มี self)
   // อ่านป้ายได้เลย ไม่ต้องถือหนังสือ


   struct Book {
    title: String,
    pages: u32,
}

impl Book {
    // Associated Function (ไม่มี self)
    // อ่านกฎได้เลย ไม่ต้องมีหนังสือ
    fn max_pages_policy() -> u32 {
        500
    }

    // Method (มี self)
    // ต้องมีหนังสือเล่มจริง ถึงจะทำได้
    fn tear_page(&mut self) {
        self.pages -= 1;
    }
}

fn main() {
    // เรียก Associated Function
    let limit = Book::max_pages_policy();
    println!("Max pages: {}", limit);

    // สร้างหนังสือ
    let mut my_book = Book {
        title: String::from("Rust 101"),
        pages: 100,
    };

    // เรียก Method
    my_book.tear_page();
    println!("Pages left: {}", my_book.pages);
}

