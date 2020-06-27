// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, r :&Rectangle) -> bool {
        self.width > r.width && self.height > r.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let sq = Rectangle::square(10);

    let rect1 = Rectangle {
        width: 30,
        height: 20,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };


    let rect3 = Rectangle {
        width: 40,
        height: 10,
    };

    println!("Can rect1 hold sq? {}", rect1.can_hold(&sq));
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // println!("rect1 is has an area of {} square pixels", rect1.area());
    // let user1 = build_user(String::from("a@b.com"), String::from("someone"));
    // let user2 = User {
    //     email: String::from("email@mail.com"),
    //     ..user1
    // };
    // println!("{}",user2.username);

}

// fn build_user(email: String, username: String) -> User {
//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
