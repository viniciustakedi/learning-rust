use std::collections::HashMap;

fn main() {
    let mut grocery: HashMap<String, f64> = HashMap::from([
        ("ovos".to_string(), 2.99),
        ("leite".to_string(), 3.99),
        ("pao".to_string(), 1.99),
    ]);

    // Dento do insert já tem a hash function para calcular 
    // o hash e inserir o valor na posição correta
    grocery.insert(
        "manga".to_string(),
        2.87
    );

    println!("{}", grocery["pao"]);
}
