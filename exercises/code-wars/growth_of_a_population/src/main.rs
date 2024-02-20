fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let percentage_converted: f64 = percent / 100.0;

    let mut current_p: i32 = p0;
    let mut years: i32 = 0;

    while current_p < p {
        let account_per_year: i32 =
            (current_p as f64 + current_p as f64 * percentage_converted + aug as f64) as i32;

        current_p = account_per_year;
        years += 1;
    }

    years
}
fn main() {
    println!("{:?}", nb_year(1500000, 2.5, 10000, 2000000));
}
