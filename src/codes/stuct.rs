struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = "mdk".to_string();
    let age : u8 = 10;
    let p = Person {
        name, age,
    };

let p1 = Person {
    name:"mdk".to_string(),
    age:15,
};
let p2 = Person {
    name:String::new(),
    age:0,
    ..p1
};

let p1_name = p1.name;
println!("{p1_name}")

}
