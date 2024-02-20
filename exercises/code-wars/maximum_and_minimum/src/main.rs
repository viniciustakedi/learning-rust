fn minimum(arr: &[i32]) -> i32 {
    // if arr.is_empty() {
    //     panic!("Empty Array")
    // }

    // let mut referece: i32 = arr[0];

    // for &n in arr.iter()  {
    //     if n < referece {
    //         referece = n;
    //     }
    // }

    // referece
    *arr.iter().min().unwrap()
}

fn maximum(arr: &[i32]) -> i32 {
    // let mut referece: i32 = arr[0];

    // for &n in arr.iter()  {
    //     if n > referece {
    //         referece = n;
    //     }
    // }

    // referece
    *arr.iter().max().unwrap()
}

fn main() {
    let arr: [i32; 8] = [4,6,2,1,9,63,-134,566];

    println!("{:?}", minimum(&arr));
    println!("{:?}", maximum(&arr));

}
