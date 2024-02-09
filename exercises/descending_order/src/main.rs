fn descending_order(x: u64) -> u64 {
    let mut arr: Vec<char>  = x.to_string().chars().collect();
    arr.sort_by(|a, b| b.cmp(a));

    let arr_to_string: String = arr.into_iter().collect();
    arr_to_string.parse::<u64>().unwrap()
}

fn main() {
    println!("{:?}", descending_order(2245615));
}
