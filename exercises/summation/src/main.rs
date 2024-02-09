fn summation(n: i32) -> i32 {
    (1..=n).sum()   
}

fn main() {
    println!("{:?}", summation(2));
}
