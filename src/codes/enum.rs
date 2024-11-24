use std::mem::size_of;

#[derive(Debug)]
enum Number {
    I32(i32),
    F32(f32),
}

fn main() {
    let n1 = Number::F32(3.14);
    let n2 = Number::I32(10);

    println!("{:?}, {:?}", n1, n2); // F32(3.14), I32(10)
    println!("{}", size_of::<Number>()); // 8
}