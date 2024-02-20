use std::collections::HashMap;

fn rgb(r: i32, g: i32, b: i32) -> String {
    let rgb_vec: [i32; 3] = [r, g, b];

    let hex_letters: HashMap<i32, &str> = HashMap::from([
        (10, "A"),
        (11, "B"),
        (12, "C"),
        (13, "D"),
        (14, "E"),
        (15, "F"),
    ]);

    let mut hexcolor: String = String::new();

    for color_value in rgb_vec.iter() {
        if *color_value < 0 {
            let zero_string = 0.to_string();
            hexcolor = format!("{hexcolor}{zero_string}{zero_string}");
            continue;
        }

        if *color_value > 255 {
            hexcolor = format!("{hexcolor}FF");
            continue;
        }

        let calc_first = color_value / 16;
        let calc_second = color_value - calc_first * 16;

        match hex_letters.get(&calc_first) {
            Some(value) => hexcolor = format!("{hexcolor}{value}"),
            None => hexcolor = format!("{hexcolor}{calc_first}"),
        }

        match hex_letters.get(&calc_second) {
            Some(value) => hexcolor = format!("{hexcolor}{value}"),
            None => hexcolor = format!("{hexcolor}{calc_second}"),
        }
    }

    hexcolor
}

fn main() {
    println!("{}", rgb(192, 255, 90));
}
