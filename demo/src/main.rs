use std::cell::RefCell;

fn main() {
    let circle = Circle::new(10);
    println!("{:?}", circle);
    let mut circle2_radius = circle.radius.borrow_mut();
    println!("{:?}", circle2_radius);
    *circle2_radius = 100;
    println!("{:?}", circle2_radius);
}

#[derive(Debug)]
pub struct Circle {
    radius: RefCell<u32>,
}

impl Circle {
    pub fn new(radius: u32) -> Circle {
        Circle {
            radius: RefCell::new(radius),
        }
    }
    pub fn area(&self) -> f64 {
        let radius = self.radius.borrow();
        (*radius * *radius) as f64 * 3.14
    }
}
