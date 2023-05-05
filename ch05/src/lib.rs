#[derive(Debug, Default)]
pub struct User<'a> {
    pub active: bool,
    pub username: &'a str,
    pub email: &'a str,
    pub sign_in_count: u64,
}

#[derive(Debug, Default)]
pub struct Color(pub u32, pub u32, pub u32);

#[derive(Debug, Default)]
pub struct Point(pub i32, pub i32, pub i32);

impl Color {
    pub fn mix(self, other: &Color) -> Color {
        Color(
            (self.0 + other.0) / 2,
            (self.1 + other.1) / 2,
            (self.2 + other.2) / 2,
        )
    }
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
        }
    }
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
