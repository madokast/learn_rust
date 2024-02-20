fn main() {
    let age:f32 = 21.3;
    println!("age = {age}"); // 21.3

    {
        let age = age as i32;
        println!("age = {age}"); // 21
    }

    println!("age = {age}"); // 21.3
}
