fn main() {
    let s: &str = "hello";
    println!("{s}");

    let string = s.to_string();
    let s :&str= &string[0..3];
    println!("{s}");
    let s :&str= &string[0..2];
    println!("{s}");
}

