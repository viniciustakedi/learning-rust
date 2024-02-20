fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let enought_cap: i32 = (cap - on - wait) * -1;

    match enought_cap > 0 {
        true => enought_cap,
        false => 0
    }
}

fn main() {
    println!("{:?}", enough(10, 5, 5));
}
