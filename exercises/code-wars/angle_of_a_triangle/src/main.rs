fn other_angle(a: u32, b: u32) -> u32 {
    180 - a - b
}


fn main() {
    println!("{:?}", other_angle(30, 60));
}
