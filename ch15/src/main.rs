use std::rc::Rc;

use ch15::{
    hello,
    CustomSmartPointer,
    // List::{self, Cons, Nil},
    MyBox,
    Rectangle,
};

fn main() {
    // let mybox = MyBox::new(5);
    // assert!(5 == *mybox);
    // let rect = Rectangle::new(10u8, 20);
    // println!("{:#?}", *rect);

    // let m = MyBox::new(String::from("Rust"));
    // hello(&(*m)[..]);
    // //等价于以下
    // hello(&(*m));
    // //等价于以下
    // hello(&m);

    // println!("----------------------");
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };
    // drop(c);
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // drop(d);

    // println!("----------------------");

    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    use List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    drop(b);
    println!("count after dropping b = {}", Rc::strong_count(&a));
    println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
