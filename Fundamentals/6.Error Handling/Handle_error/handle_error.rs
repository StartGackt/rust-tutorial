// ตัวอย่างการ Handle error: กดรหัสผ่านผิด
// ถ้ากดผิด โปรแกรมจะไม่หยุด
// แต่จะแจ้งเตือน และยังทำงานต่อได้

fn check_password(input: &str) -> Result<(), String> {
    let correct_password = "1234";

    if input == correct_password {
        // รหัสถูก
        Ok(())
    } else {
        // รหัสผิด
        Err(String::from("Password is incorrect"))
    }
}

fn main() {
    let user_input = "1111"; // สมมติผู้ใช้กรอกรหัสผิด

    let result = check_password(user_input);

    // ใช้ match จัดการผลลัพธ์
    match result {
        Ok(()) => {
            println!("Login success");
        }
        Err(error) => {
            println!("Login failed: {}", error);
        }
    }

    // โปรแกรมยังทำงานต่อได้
    println!("Program continues...");
}
