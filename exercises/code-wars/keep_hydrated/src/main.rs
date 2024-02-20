fn litres(time: f64) -> i32 {
    let litres_per_hour: f64 = 0.5;
    let time_rounded = time.floor() as i32;

    (time_rounded as f64 * litres_per_hour) as i32
}

fn main() {
    println!("{:?}", litres(11.8));
}
