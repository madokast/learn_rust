#[derive(Debug)]
struct V2 {a:f64, b:f64}

impl V2 {
    fn new(x:f64, y:f64) -> Self {
        V2 {a:x, b:y}
    }
}

fn main() {
    let v = V2::new(10., 3.14);
    println!("{:?}", v);
}
