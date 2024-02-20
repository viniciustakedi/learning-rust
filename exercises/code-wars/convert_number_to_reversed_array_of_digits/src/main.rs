fn digitize(n: u64) -> Vec<u8> {
    let n_arr: Vec<char> = n.to_string().chars().collect();

    n_arr
        .clone()
        .iter_mut()
        .map(|c: &mut char| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect::<Vec<u8>>()
}

fn main() {
    println!("{:?}", digitize(0));
}
