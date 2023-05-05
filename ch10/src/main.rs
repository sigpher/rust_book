use ch10::{get_largest, Point};

fn main() {
    let vec_char = vec!['x', 'a', 'c', 'h', 'z'];
    let largest_char = get_largest(&vec_char);
    println!("{}", largest_char);

    let vec_number = vec![34, 50, 25, 100, 65];
    let largest_number = get_largest(&vec_number);
    println!("{largest_number}");

    let point1: Point<f32> = Point { x: 20.0, y: 10.0 };
    let point2: Point<f32> = Point { x: -5.0, y: 10.0 };

    println!("{:?}", point1);
    println!("{:?}", point2);
    println!("{}", &point1.distance_from_origin());
    println!("{}", &point2.distance_from_origin());
}
