use ch05::{Color, Point, Rectangle, User};

fn main() {
    let mut user1 = User::default();
    user1.active = true;
    user1.username = "choi";
    user1.email = "choi@tdyh.com.cn";
    user1.sign_in_count = 1;
    println!("{:#?}", user1);

    let user2: User = User {
        username: "lora",
        email: "lora@tdyh.com.cn",
        ..user1
    };
    println!("{:#?}", user2);

    let rect1 = Rectangle {
        width: 5,
        height: 5,
    };
    let rect2 = Rectangle::default();

    let flag = &rect2.can_hold(&rect1);
    println!("{}",flag);
    let flag = &rect1.can_hold(&rect2);
    println!("{}",flag);
}
