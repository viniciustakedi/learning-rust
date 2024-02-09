fn are_you_playing_banjo(name: &str) -> String {
    let first_name_letter: char = name.chars().collect::<Vec<char>>()[0];
    if first_name_letter == 'r' || first_name_letter == 'R' {
        format!("{} plays banjo", name)
    } else {
        format!("{} does not play banjo", name)
    }
}

fn main() {
    println!("{}", are_you_playing_banjo("Takedi"));
}
