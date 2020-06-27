enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

#[derive(Debug)]
enum USState {
    Alabama, 
    Alaska,
}

impl Message {
    fn call(&self) {

    }
}



fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let s = USState::Alabama;
    let c = Coin::Quarter(s);
    println!("{}", value_in_cents(c));


    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
    println!("{}", sum);
}
