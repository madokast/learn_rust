fn main() {
    let s = String::from("hello");

    let ss = &s;
    println!("{ss}");

    let moved = s; // cannot move out of `s` because it is borrowed

    println!("{ss}");
    println!("{moved}");
}

