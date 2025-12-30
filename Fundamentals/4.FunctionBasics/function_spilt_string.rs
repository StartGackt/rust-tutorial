// ฟังก์ชันนี้ใช้สำหรับ "แยกข้อความ" (split string)
// split = การเอาข้อความยาว ๆ มาแบ่งออกเป็นชิ้น ๆ
// โดยใช้ตัวคั่น (delimiter) เช่น , | : หรือช่องว่าง
//
// ตัวอย่าง:
// "Hello,world,rust" เมื่อ split ด้วย ','
// จะได้ ["Hello", "world", "rust"]
fn split_string(s: &str, delimiter: char, field: usize) -> Option<String> {

    // s.split(delimiter)
    // คือการบอกว่า "ถ้าเจอ delimiter ให้แยกข้อความออกเป็นชิ้นใหม่"
    //
    // collect() จะรวบรวมผลลัพธ์ที่ได้
    // จากหลายชิ้น มาเก็บไว้ในรูปแบบ Vec (list)
    let parts: Vec<&str> = s.split(delimiter).collect();

    // parts.get(field)
    // คือการดึงข้อมูลตามตำแหน่ง (index)
    // - ถ้ามีข้อมูล → Some(&str)
    // - ถ้าไม่มีข้อมูล → None
    //
    // map() ใช้แปลงค่าที่อยู่ข้างใน Some
    // จาก &str → String
    // ถ้าเป็น None จะส่ง None กลับไปทันที
    parts.get(field).map(|v| v.to_string())
}

fn main() {
    // ข้อความต้นฉบับ: "Hello,world,rust,programming"
    // ตัวคั่นคือ ','
    // field = 1 หมายถึงเอาคำที่ 2 (เริ่มนับจาก 0)
    let chunk = split_string("Hello,world,rust,programming", ',', 1);

    // แสดงผลลัพธ์
    // ใช้ {:?} เพราะเป็น Option<String>
    println!("Chunk: {:?}", chunk);
}
