fn main() {
     //* ลูปจาก 1 ถึง 10
    for i in 1..=10{
        if i % 2 == 0 {
            //* continue จะข้ามการทำงานของคำสั่งต่อไปนั้น
            continue;
        }
        println!("i = {}", i);
        if i == 5 {
            //* break จะออกจากลูป i เท่ากับ 5 จะออกจากลูป   // i = 1, i = 3, i = 5
            break;
        }
    }
}

   
