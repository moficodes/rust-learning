fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?}", six.unwrap(), none.unwrap_or(0));

    let some_u8_value = 3u8;
    match some_u8_value {
        1 => println!("one"),
        4 => println!("four"),
        _ => println!("{}",some_u8_value),
    }
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
