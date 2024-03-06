#[derive(Debug)]
struct Person {
    name : String,
    age : u8,
}

fn main() {
    let p = Person {
        name :"mdk".to_string(),
        age: 10,
    };
    println!("{:?}", p);
    println!("{:#?}", p);
    dbg!(&p);
    println!("{}", p.name);
    println!("{p.name}");
}
