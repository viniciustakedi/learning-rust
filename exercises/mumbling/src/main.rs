fn accum(s:&str)->String {
    let arr: Vec<char> = s.chars().collect();
    let mut response: String = String::new();

    for (index, char) in arr.iter().enumerate() {
        let char_repeated: String = char.to_string().to_uppercase() + &char.to_string().repeat(index).to_lowercase();

        if index + 1 == arr.len() {
            response = response + &char_repeated;
            continue;
        }

        response = response + &char_repeated + "-";
    }

    response
}

fn main() {
    println!("{}", accum("abcrgt"));
}
