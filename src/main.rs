mod utils;

fn main() {
    let n: utils::number::Number = utils::number::Number {value:123};
    println!("{:?}", n); // Number { value: 123 }
    utils::number::func::hello(); // hello
}
