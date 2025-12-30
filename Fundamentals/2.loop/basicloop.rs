
fn main() {

    //* พื้นฐานลูป  if เป็นคำสั่ง (Statement)  อาจดูรกหากต้องทำซ้ำๆ **/ 
    // let proceed = false;
    // if proceed {
    //     println!("Proceeding");
    // } else {
    //     println!("Not proceeding");
    // }

    // let age = 11;
    // if age < 10 {
    //     println!("Child");
    // } else if age > 15 {
    //     println!("Teenager");
    // } else if age > 10 {
    //     println!("Adult");
    // } else {
    //     println!("Short");
    // }
    

    // if เป็นนิพจน์ (Expression) โค้ดสะอาดกว่า แยก Logic การเลือกค่าออกจากการแสดงผล
// let age = 11;
//   let result = if age <10{
//        "Child"
//   }else if age > 15{
//       "Teenager"
//   }else if age > 10{
//       "Adult"     
//   } else{
//       "BABY"
//   };
//   println!("The result is: {}", result);

//*  ใช้คำสั่ง if let  ซึ่งเป็นเอกลักษณ์ของภาษา Rust เอาไว้จัดการกับข้อมูลประเภท "อาจจะมีค่า หรือ ไม่มีค่าก็ได้" (เรียกว่า Option)
let maybe_number = Some(42); // or None in some cases
//* แปลว่า: "ถ้า" แกะกล่อง maybe_number ออกมาแล้วเจอ "Some(เลข)"
//* ให้เอาเลขนั้นมาใส่ในตัวแปรชื่อ "num" แล้วทำในวงเล็บปีกกา
if let Some(num) = maybe_number {
    println!("The number is {}", num); // ปริ้นท์เลขที่แกะได้ (42)
} else {
    //* แต่ถ้าแกะแล้วเป็น None (กล่องเปล่า) ให้ทำตรงนี้แทน
    println!("No number provided.");
}
}