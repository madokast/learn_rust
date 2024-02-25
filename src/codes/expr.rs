fn main() {
    let a = {
        let b = 123;
        println!("b = {b}"); // 123
        b + 1
    };

    println!("a = {a}"); // 124

    let hr = explicit_return();
    let hnr = implicit_return();
    println!("r {hr} {hnr}");
}

fn explicit_return() -> i32 {
    return 123;
}

fn implicit_return() -> i32 {
    123
}
