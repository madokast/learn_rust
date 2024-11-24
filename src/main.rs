fn add_one(num:&mut Option<i32>) {
    match num {
        Some(val) => *val += 1,
        _ => ()
    };
}

fn add_one2(num:&mut Option<i32>) {
    if let Some(val) = num {
        *val += 1;
    }
}

fn main() {
    let mut num = Some(1);
    println!("{:?}", num); // Some 1
    add_one(&mut num);
    println!("{:?}", num); // Some 2
    add_one2(&mut num);
    println!("{:?}", num); // Some 3
}