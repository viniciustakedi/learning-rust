fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut multiples: Vec<u32> = Vec::new();

    for i in 1..=n {
        multiples.push(x * i);
    }

    multiples
}

fn main() {
    let multiples = count_by(100, 3);
    println!("{:?}", multiples);
}
