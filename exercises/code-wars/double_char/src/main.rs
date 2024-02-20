fn double_char(s: &str) -> String {
    let mut response: String = String::new();

    for s_char in s.chars() {
        // response = response + &s_char.to_string() + &s_char.to_string();
        response.push(s_char);
        response.push(s_char);
    }

    response.to_string()
}

fn main() {
    println!("{}", double_char("String"));
}
