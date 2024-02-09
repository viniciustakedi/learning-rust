fn split_and_sort(numbers: &str) -> Vec<f32> {
    let numbers_arr_str = numbers.split_whitespace().collect::<Vec<&str>>();
    let mut numbers_arr: Vec<f32> = Vec::new();

    for number in numbers_arr_str.clone() {
        
        numbers_arr.push(number.parse().unwrap())
    }

    numbers_arr.to_vec()
}

fn main() {
    let split_sort_str: Vec<f32> = split_and_sort("1.0 2.0 9.3");
    println!("{:?}", split_sort_str);
}
