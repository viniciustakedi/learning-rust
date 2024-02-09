fn abbrev_name(name: &str) -> String {
    let arr_name_and_lastname: Vec<&str> = name.split_whitespace().collect::<Vec<&str>>();
    let first_letter_name = &arr_name_and_lastname[0][0..1];
    let first_letter_lastname = &arr_name_and_lastname[1][0..1];

    format!(
        "{}.{}",
        first_letter_name.to_uppercase(),
        first_letter_lastname.to_uppercase()
    )
}

fn main() {
    println!("{}", abbrev_name("Vinicius Takedi"));
}
