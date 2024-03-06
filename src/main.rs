struct V2 {a:f64, b:f64}

impl V2 {
    fn length(&mut self) -> f64 {
        (self.a * self.a + self.b * self.b).sqrt()
    }
}

fn main() {
    let mut v = V2 {a:3.,b:4.};
    println!("length is {:.3}", v.length()); // length is 5.000
}
