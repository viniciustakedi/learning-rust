fn min_max(lst: &[i32]) -> (i32, i32) {
    (*lst.iter().min().unwrap(), *lst.iter().max().unwrap())
}

fn main() {
    println!("{:?}", min_max(&[1, 2, 3, 4, 5]));
}
