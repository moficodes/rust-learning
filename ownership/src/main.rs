fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &mut s;
    let r3 = &s;

    let x = 5;
    make_copy(x);
    println!("{}", x);

    let str = String::from("Mofizur Rahman");
    println!("{}",first_word(&str));
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn change(some_string: &mut String)  {
    some_string.push_str(", world")
}

fn take_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn make_copy(some_int: i32) {
    println!("{}", some_int);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i]
        }
    }

    &s
}