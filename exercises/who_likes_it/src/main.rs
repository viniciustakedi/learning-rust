fn likes(names: &[&str]) -> String {
    if names.len() == 0 {
        return "no one likes this".to_string();
    }

    let mut message: String = String::new();

    for (index, name) in names.into_iter().enumerate() {
            
        println!("{:?} {}", index, name);
    }

    message
}

fn main() {
    println!("{}", likes(&["John", "Jane"]));
}
