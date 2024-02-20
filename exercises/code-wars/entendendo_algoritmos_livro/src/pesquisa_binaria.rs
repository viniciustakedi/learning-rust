pub fn pesquisa_binaria(mut arr: Vec<i32>, n: i32) -> bool {
    let mut existe_na_arr: bool = false;
    let mut numero_de_tentativas: i32 = 0;

    while !arr.is_empty() {
        let index: usize = arr.len() / 2;

        if arr[index] == n {
            existe_na_arr = true;
            numero_de_tentativas += 1;
            break;
        } else if arr[index] > n {
            arr = arr[0..index].to_vec();
        } else {
            arr = arr[index+1..].to_vec();
        }

        numero_de_tentativas += 1
    }

    println!("Número de tentativas: {}", numero_de_tentativas);
    existe_na_arr
}