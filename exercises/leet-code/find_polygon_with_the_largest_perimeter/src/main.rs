pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
    fn get_max_and_remove (mut vec: Vec<i32>) -> (i32, Vec<i32>) {
        let mut max: i32 = vec[0];
        let mut max_index: usize = 0;

        for (index, value) in vec.iter().enumerate() {
            if value > &max {
                max = *value;
                max_index = index;
            }
        }

        vec.remove(max_index);
        (max, vec)
    }

    while nums.len() > 0 {
        let (max, new_arr) = get_max_and_remove(nums);
        nums = new_arr
        
    }
    todo!()
}

fn main() {
    println!("{:?}", largest_perimeter([5, 5, 5].to_vec()));
}
