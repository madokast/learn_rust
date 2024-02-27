fn main() {
    let mut s:String = "hello".to_string();
    
    let m:&mut String = &mut s;

    // let im:&String = &s; // cannot borrow `s` as immutable because it is also borrowed as mutable

    m.push_str(", world");

    println!("{s}");

}

