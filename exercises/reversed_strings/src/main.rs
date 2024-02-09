fn solution(phrase: &str) -> String {
    // let phrase_chars: Vec<char> = phrase.chars().collect();
    // let mut response: Vec<char> = Vec::new();

    // for i in 0..phrase_chars.len() {
    //     let index: usize = (phrase_chars.len() - 1) - i;
    //     response.push(phrase_chars[index]);
    // }

    // response.into_iter().collect()

    phrase.chars().rev().collect::<String>()
}

fn main() {
    println!("{}", solution("ola"));
}
