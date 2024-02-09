fn find_odd(arr: &[i32]) -> i32 {
    let mut response: i32 = 0;
    for n in arr.iter() {
        if arr.iter().filter(|&x| x == n).count() % 2 != 0 {
            response = *n
        }
    }

    response
}

fn main() {
    println!("{}", find_odd(&[0, 1, 0, 1, 0]));
}
