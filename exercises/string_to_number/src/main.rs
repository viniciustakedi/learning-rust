fn string_to_number(s: &str) -> i32 {
  s.parse::<i32>().unwrap()
}

fn main() {
    println!("{}", string_to_number("1234"));
}
