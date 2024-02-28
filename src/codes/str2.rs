fn main() {
    let mut s = "hello".to_string();

    let rs = f(&s);

    s.push_str("aa");

    println!("{rs}");
}

fn f(s:&String) -> &str {
    &s[0..2]
    // "others"
}

