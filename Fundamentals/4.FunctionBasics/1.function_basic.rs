//* ตัวอย่างการใช้ฟังก์ชันที่คืนค่า
//* ฟังก์ชันที่คืนค่า คือ ฟังก์ชันที่ส่งค่าออกไปให้ผู้เรียกใช้งาน
//* ฟังก์ชันต้องระบุชนิดข้อมูลของค่าที่จะคืน (-> i32)
//* ใน Rust ไม่จำเป็นต้องใช้คำสั่ง return
//* ค่าที่อยู่บรรทัดสุดท้าย (ไม่มี ;) จะถูกส่งกลับโดยอัตโนมัติ

fn process_data(data: &[i32]) -> i32 {
    let mut sum = 0;

    // วนลูปผ่านข้อมูล
    for value in data {
        sum += value;
    }

    // แสดงผลลัพธ์
    println!("Sum: {}", sum);

    if sum % 2 == 0 {
        println!("Sum is even");
    } else {
        println!("Sum is odd");
    }

      // คืนค่าได้ แต่ไม่จำเป็นต้องมี return ก็ได้
    sum
}

fn main() {
    // ส่งข้อมูลไปยังฟังก์ชัน
    let result = process_data(&[1, 2, 3, 4, 5]);
    // แสดงผลลัพธ์
    println!("Returned value: {}", result);
}

// ผลลัพธ์:
// Sum: 15
// Sum is odd
// Returned value: 15
