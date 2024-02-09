// use pesquisa_binaria::pesquisa_binaria;

fn somar_array_recursividade(arr: Vec<i32>) -> i32 {
    let n: usize = arr.len() as usize;

    if n <= 0 {
        return 0;
    }

    return somar_array_recursividade(arr[..n-1].to_vec()) + arr.clone()[n - 1];
}

fn main() {
    // let arr: Vec<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec();
    // println!("{:?}", pesquisa_binaria(arr, 11));
    let arr: Vec<i32> = [4, 5, 6, 7, 8, 10].to_vec();
    println!("{}", somar_array_recursividade(arr));
}
