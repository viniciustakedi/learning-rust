fn gimme_the_letters(sp: &str) -> String {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();

    let binding: String = sp.to_string();
    let letters: Vec<&str> = binding.split("-").collect::<Vec<&str>>();

    let mut res: String = String::new();
    let mut add_into_array: bool = false;

    if letters[0] == letters[1] {
        return letters[0].to_string()
    }

    for l in alphabet.into_iter() {
        if l.to_string() == letters[0].to_lowercase() {
            res = res + &l.to_string();
            add_into_array = true;
            continue;
        }

        if l.to_string() == letters[1].to_lowercase() {
            res = res + &l.to_string();
            add_into_array = false;
            continue;
        }

        if add_into_array {
            res = res + &l.to_string();
        }
    }

    let first_char = letters[0].chars().nth(0);

    match first_char {
        Some(c) => {
            if c.is_uppercase() {
                res = res.to_uppercase();
            }
        }
        None => {}
    }

    res
}

fn main() {
    println!("{}", gimme_the_letters("Q-Z"));
}
