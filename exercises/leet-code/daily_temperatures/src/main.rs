pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut response: Vec<i32> = Vec::new();
    let mut was_second_for_was_pushed_a_number: bool = false;

    for (index, t) in temperatures.iter().enumerate() {
        let future_temperatures: &[i32] = &temperatures.clone()[index+1..];

        for (index_ft, ft) in future_temperatures.iter().enumerate() {
            if ft > t {
                response.push(index_ft as i32);
                was_second_for_was_pushed_a_number = true;
            }
        }

        if !was_second_for_was_pushed_a_number {
            response.push(0);
            was_second_for_was_pushed_a_number = false;
        }
    }

    response
}

fn main() {
    println!("{:?}", daily_temperatures([30,40,50,60].to_vec()));
}
