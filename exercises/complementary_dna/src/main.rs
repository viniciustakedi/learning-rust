fn dna_strand(dna: &str) -> String {
    let arr: Vec<char> = dna.chars().collect::<Vec<char>>();
    let mut s: String = String::new();

    for sa in arr.iter() {
        match sa {
            'A' => s = s + "T",
            'T' => s = s + "A",
            'C' => s = s + "G",
            'G' => s = s + "C",
            _ => s = s + ""
        }
    }

    s
}

fn main() {
    println!("{}", dna_strand("ATTGC"));
}
