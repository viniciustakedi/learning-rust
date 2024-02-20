fn string_to_char(value: &str, revert: bool) -> Vec<char> {
    match revert {
        true => value.chars().rev().collect(),
        false => value.chars().collect(),
    }
}

fn solution(word: &str, ending: &str) -> bool {
    let word_arr: Vec<char> = word.chars().rev().collect();
    let ending_arr: Vec<char> = ending.chars().rev().collect();
    let mut is_valid = true;

    for (index, char_value) in ending_arr.iter().enumerate() {
        if word_arr[index] != *char_value {
            is_valid = false;
            break;
        }
    }

    is_valid
}

fn main() {
    println!("{:?}", solution("abc", "d"));
}
