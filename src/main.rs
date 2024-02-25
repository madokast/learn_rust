fn main() {
let s: String = String::from("hello");
// let s2: String = s; // move occurs because `s` has type `String`, which does not implement the `Copy` trait. value moved here
use_s(s);
println!("{s}"); // value borrowed here after move
// println!("{s2}");

   let a = 1;
   let a2 = a;

   println!("{a}");
   println!("{a2}");
}

fn use_s(s:String) {
    println!("{s}");
}


