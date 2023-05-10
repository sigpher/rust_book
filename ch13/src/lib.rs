#[derive(Debug, Clone, Copy)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0_u8;
        let mut num_blue = 0_u8;
        for color in &self.shirts {
            // match color {
            //     ShirtColor::Red => num_red += 1,
            //     ShirtColor::Blue => num_blue += 1,
            // }
            if let ShirtColor::Red = color {
                num_red += 1
            } else {
                num_blue += 1
            }
        }
        if num_red > num_blue {
            return ShirtColor::Red;
        } else {
            return ShirtColor::Blue;
        }
    }
}
