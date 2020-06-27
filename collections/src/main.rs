use std::collections::HashMap;

fn main() {
    // VECTORS
    // let mut v: Vec<i32> = Vec::new();

    // v.push(1);
    // v.push(2);

    // let mut v2 = vec![1,2,3];

    // let third = v2[2];
    // v2.push(6);
    // println!("third elem {}", third);

    // for i in &mut v2 {
    //     *i += 10;
    // }

    // match v2.get(3) {
    //     Some(elem) => println!("{}", elem),
    //     None => println!("no item found"),
    // }

    // println!("{:?}", v2);

    // STRINGS

    // let data = "initial string";
    // let s = data.to_string();
    // println!("{}", s);
    // let s = String::from(data);
    // println!("{}", s);

    // let s1 = String::from("hello ");
    // let s2 = String::from("world");

    // let s3 = format!("{}{}", s1, s2);
    // println!("{} {}", s3, s2);
    // let s3 = s1 + &s2;
    // println!("{} {}", s3, s2);

    // println!("{}",data);

    // let special = "नमस्ते";

    // for c in special.bytes() {
    //     println!("{}", c);
    // }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // println!("{:?}", scores);
    for (k, v) in &scores {
        println!("{} {}", k, v);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
