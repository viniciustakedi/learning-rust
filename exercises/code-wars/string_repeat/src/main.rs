fn repeat_str(str: &str, count: usize) -> String {
    let mut sr: String = String::new();
    for _ in 0..count {
        sr.push_str(str);
    }

    sr
}

fn main() {
    let s = repeat_str("Hello", 3);
    println!("{:?}", s);
}