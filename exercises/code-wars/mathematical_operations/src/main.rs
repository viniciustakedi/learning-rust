fn basic_op(operator: char, value1: i32, value2: i32) -> i32 {
    let result: i32;

    match operator {
        '+' => result = value1 + value2,
        '-' => result = value1 - value2,
        '/' => result = value1 / value2,
        '*' => result = value1 * value2,
        _ => result = 0
    }

    result
}

fn main() {
    println!("{:?}", basic_op('-', 15, 18));
}
