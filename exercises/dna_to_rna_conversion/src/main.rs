fn dna_to_rna(dna: &str) -> String {
    // let dna_chars = dna.chars().collect::<Vec<char>>();
    // let mut response: Vec<char> = Vec::new();

    // for i in 0..dna_chars.len() {
    //     if dna_chars[i] == 'T' {
    //         response.push('U');
    //         continue;
    //     }

    //     response.push(dna_chars[i]);
    // }

    // response.into_iter().collect::<String>()

    dna.replace("T", "U")
}

fn main() {
    println!("{}", dna_to_rna("TTTT"));
}
