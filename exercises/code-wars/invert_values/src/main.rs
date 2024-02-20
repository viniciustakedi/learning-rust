fn invert(values: &[i32]) -> Vec<i32> {
    // let mut response:Vec<i32> = Vec::<i32>::new();

    // for v in values.iter() {
    //     response.push(v * -1);
    // }

    // response
    values.iter().map(|x| -x).collect()
}

fn main() {
    println!("{:?}", invert(&[1, 2, 3, 4, 5]));
}
