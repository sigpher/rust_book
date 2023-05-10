fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference `{:?}` gets {:?}",
        user_pref1, giveaway1
    );

    println!("----------------------------");
    let user_pref1 = None;
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference `{:?}` gets {:?}",
        user_pref1, giveaway1
    );
}

#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // if let Some(user_pref1) = user_preference {
        //     user_pref1
        // } else {
        //     self.most_stocked()
        // }
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0_u8;
        let mut num_blue = 0_u8;

        for color in &self.shirts {
            if let ShirtColor::Red = color {
                num_red += 1;
            } else {
                num_blue += 1;
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


