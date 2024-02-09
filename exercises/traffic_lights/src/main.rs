fn update_light(current: &str) -> String {
    let new_value: &str;

    match current {
        "green" => new_value = "yellow",
        "yellow" => new_value = "red",
        "red" => new_value = "green",
        _ => new_value = "nothing"
    }

    new_value.to_string()
}

fn main() {
    println!("{:?}", update_light("red"));
}
