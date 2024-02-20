pub fn is_power_of_two(mut n: i32) -> bool {
    let mut is_power = false;

    while n > 0 {
        let division: f64 = (n as f64) / 2.0;

        if division == 1.0 || n == 1 {
            is_power = true
        }

        if division % 2.0 != 0.0 {
            break;
        }

        n = division as i32;
    }

    is_power
}

fn main() {
    println!("{:?}", is_power_of_two(16777217));
}
