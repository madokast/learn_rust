#[derive(Debug)]
struct V2 {a:f64, b:f64}

impl V2 {
    fn new(x:f64, y:f64) -> V2 {
        V2 {a:x, b:y}
    }
    fn length(self:&V2) -> f64 {
        (self.a * self.a + self.b *self.b).sqrt()
    }
}

use std::fmt;

impl fmt::Display for V2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

fn main() {
    let v = V2::new(5., 12.);
    println!("{} is {:?} length is {}", v, v, v.length());
}
