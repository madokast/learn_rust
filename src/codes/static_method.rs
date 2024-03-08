#[derive(Debug)]
struct V2 {a:f64, b:f64}

impl V2 {
    fn new(x:f64, y:f64) -> Self {
        V2 {a:x, b:y}
    }
    fn length(&self) -> f64 {
        (self.a * self.a + self.b *self.b).sqrt()
    }
}

fn main() {
    let v = V2::new(5., 12.);
    println!("{:?} length is {}", v, v.length());
}
