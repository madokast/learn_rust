fn main() {
    let mut s = String::from("hello");

    let ss = &s;

    println!("{s}"); // hello
    println!("{ss}"); // hello

    s.insert(0, 'k');

    let ss = &s; // 必须再定义一下。否则 ss 作用范围和 s.insert 构成的可变引用重叠

    println!("{s}"); // khello
    println!("{ss}"); // khello
}

