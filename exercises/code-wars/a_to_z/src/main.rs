fn gimme_the_letters(sp: &str) -> String {
    let alphabet: &str = "abcdefghijklmnopqrstuvwxyz";
    let letters: Vec<&str> = sp.split("-").collect();
    let mut result: &str;

    for alphabetChar in alphabet.chars() {
        if letters.get(0) == alphabetChar.get(0) {
            
        }
        // println!("{:?}", l);
    }

    "t".to_string()
}

fn main() {
    println!("{:?}", gimme_the_letters("a-d"));
}
