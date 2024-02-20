// fn grow(nums: Vec<i32>) -> i32 {
//     let mut result: i32 = 0;

//     for num in nums {
//         if result == 0 {
//             result = num;
//             continue;
//         }

//         result = num * result;
//     }

//     result
// }

fn grow(nums: Vec<i32>) -> i32 {
    nums.iter().fold(1, |acc, x| acc * x)
}

fn main() {
    println!("{:?}", grow(vec![1, 2, 3, 4]));
}
