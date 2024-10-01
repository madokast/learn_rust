fn main() {
    println!("Hello, world!");

    let a:i32 = "42".parse().expect("err");

    println!("a = {:?}", a);
}
