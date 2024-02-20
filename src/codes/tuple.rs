fn main() {
    let ts:(i32, f64, char) = (14, 3.14, 'a');

    let t0:i32 = ts.0;
    let t1:f64 = ts.1;
    let t2:char = ts.2;
    println!("ts = {t0}, {t1}, {t2}"); // ts = 14, 3.14, a
    

    let (i, f, c) = ts;
    println!("ts = {i}, {f}, {c}"); // ts = 14, 3.14, a
}
