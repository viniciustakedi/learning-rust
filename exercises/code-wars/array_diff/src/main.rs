fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    // let mut diff_numbers: Vec<T> = Vec::new();

    // for na in a.into_iter() {
    //     let exists_on_b = b.contains(&na);
    //     if !exists_on_b {
    //         diff_numbers.push(na)
    //     }
    // }

    // diff_numbers

    a.into_iter().filter(|f| !b.contains(f)).collect()
}

fn main() {
    println!("{:?}", array_diff(vec![1, 2, 2, 2, 3], vec![2]));
}
