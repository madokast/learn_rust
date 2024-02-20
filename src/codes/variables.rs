fn main() {
    let x = 10;
    println!("x = {x}");
    // x = 3; // cannot assign twice to immutable variable `x`

    const PI :f64 = 3.14;
    println!("PI = {PI}");

    // A non-constant value was used in a constant expression. 'constant' means 'a compile-time value'.
    // const A :i32= x; 
    // println!("A = {A}")
}
