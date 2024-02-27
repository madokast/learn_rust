fn main() {
    let mut s:String = "hello".to_string();
    refer(&s);
    println!("main {s}"); // s is valid after calling refer

    mut_refer(&mut s);
    println!("main2 {s}");
}

fn refer(s:&String) {
    println!("refer {s}");
}

fn mut_refer(s:&mut String) {
    s.push_str(", world");
    println!("mut refer {s}");
}