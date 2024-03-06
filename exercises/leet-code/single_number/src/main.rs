pub fn single_number(nums: Vec<i32>) -> i32 {
    for (index, n) in nums.iter().enumerate() {
        let mut arr: Vec<i32> = nums.clone();
        arr.swap_remove(index);

        if !arr.contains(n) {
            return *n;
        }
    }

    nums[0]
}

fn main() {
    println!("{:?}", single_number([2, 2, 1].to_vec()));
}
