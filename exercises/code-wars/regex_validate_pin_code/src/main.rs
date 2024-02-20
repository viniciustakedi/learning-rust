use regex::Regex;

fn validate_pin(pin: &str) -> bool {
    if pin.len() == 4 || pin.len() == 6 {
        return Regex::new(r"^[0-9]*$").unwrap().is_match(pin);
    }

    false
}

fn main() {
    println!("{:?}", validate_pin("1234"));
}
