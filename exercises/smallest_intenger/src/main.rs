fn find_smallest_int(arr: &[i32]) -> i32 {
    // let mut response: i32 = 0;
    // for (index, &n) in arr.iter().enumerate() {
    //     if index == 0 { 
    //         response = n;
    //         continue;
    //     }

    //     if response > n {
    //         response = n;
    //     }
    // }

    // response
    *arr.iter().min().unwrap()
}

fn main() {
    println!("{}", find_smallest_int(&[34, -345, -1, 100]));
}
