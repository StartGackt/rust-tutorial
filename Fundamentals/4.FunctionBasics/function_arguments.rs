// function arguments คือ ค่าที่ส่งเข้ามาให้ฟังก์ชันทำงาน
// ในที่นี้ arguments คือ `numbers`
// ซึ่งเป็นข้อมูลที่ถูกส่งมาจากฟังก์ชัน main
fn sum(numbers: &[i32]) -> i32 {

    // numbers คือ parameter (ตัวแปรรับค่า)
    // &[i32] หมายถึง รับข้อมูลหลายตัวแบบอ้างอิง (ไม่ก็อปปี้ข้อมูล)
    let mut result = 0;

    // วนลูปผ่าน argument ที่รับเข้ามา
    for number in numbers {
        result += number;
    }

    // คืนค่าผลรวมออกไป
    result
}

fn main() {
    // สร้างข้อมูลต้นทาง
    let numbers = [1, 2, 3, 4, 5];

    // เรียกใช้ฟังก์ชัน sum
    // &numbers คือ argument ที่ส่งเข้าไปในฟังก์ชัน
    let result = sum(&numbers);

    // แสดงผลลัพธ์ที่ได้จากฟังก์ชัน
    println!("Result: {}", result);
}
