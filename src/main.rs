use std::mem::size_of;

#[derive(Debug)]
enum Value {
    I32(i32),
    Str(String),
    OK,
}

fn main() {
    let i = Value::I32(10);
    let s = Value::Str("hello".to_string());
    let o = Value::OK;

    println!("i = {i:?}");
    println!("s = {s:?}");
    println!("o = {o:?}");

    println!("size = {}", size_of::<Value>());
    println!("size = {}", size_of::<i32>());
}

