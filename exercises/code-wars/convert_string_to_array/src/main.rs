fn string_to_array(s: &str) -> Vec<String> {
    // let r: Vec<String> = s
    //     .split(' ')
    //     .collect::<Vec<&str>>()
    //     .into_iter()
    //     .map(|s| s.to_string())
    //     .collect();

    // r

    s.split_whitespace().map(String::from).collect()
}

fn main() {
    println!("{:?}", string_to_array("Bora testar essa parada"));
}
