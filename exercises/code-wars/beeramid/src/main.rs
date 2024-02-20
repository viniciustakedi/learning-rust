fn beeramid(bonus: i32, price: f32) -> usize {
    if bonus < price as i32 {
        return 0.try_into().unwrap();
    }

    let cans_amount: i32 = bonus / price as i32;

    let mut level: i32 = 1;
    let mut current_level_cans_amount: i32 = 0;

    while current_level_cans_amount <= cans_amount {
        let amount_account: i32 = level.pow(2);
        current_level_cans_amount += amount_account;

        if current_level_cans_amount > cans_amount {
            level -= 1;
            break;
        }

        level += 1;
    }

    level.try_into().unwrap()
}

fn main() {
    println!("{:?}", beeramid(15, 3.0));
}
