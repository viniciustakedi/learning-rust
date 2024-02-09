fn feast(beast: &str, dish: &str) -> bool {
    let beast_name_arr: Vec<String> = beast.split_whitespace().map(String::from).collect();
    let beast_first_name_first_char: &char = &beast_name_arr[0].chars().collect::<Vec<char>>()[0];
    let beast_last_name_last_char: &char = &beast_name_arr[beast_name_arr.len() - 1]
        .chars()
        .collect::<Vec<char>>()[beast_name_arr[beast_name_arr.len() - 1].len() - 1];

    let dish_arr: Vec<String> = dish.split_whitespace().map(String::from).collect();
    let dish_first_char: &char = &dish_arr[0].chars().collect::<Vec<char>>()[0];
    let dish_last_char: &char = &dish_arr[dish_arr.len() - 1].chars().collect::<Vec<char>>()
        [dish_arr[dish_arr.len() - 1].len() - 1];

    beast_first_name_first_char == dish_first_char && beast_last_name_last_char == dish_last_char

    // beast.as_bytes().first() == dish.as_bytes().first() && 
    // beast.as_bytes().last() == dish.as_bytes().last()
}

fn main() {
    println!("{}", feast("great blue heron", "garlic naan"));
}
