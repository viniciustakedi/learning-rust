fn main() {
    // Read a text from file
    let text = std::fs::read_to_string("./text.txt").unwrap();
    println!("Text: {}", text);
}
