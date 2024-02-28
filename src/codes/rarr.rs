fn main() {
    let mut v = vec![1,2,3];

    let r = &mut v[0..1];

    r[0] = 100;

    let a = r[0];

    println!("{a}");

    let s = &[1,2,3][..];
}


