pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: [i32; 2] = [0, 0];
    let mut was_target: bool = false;

    for (index, num) in nums.iter().enumerate() {
        if was_target {
            break;
        }

        for (index_to_sum, num_to_sum) in nums.iter().enumerate() {
            if index == index_to_sum {
                continue;
            }

            if (num + num_to_sum) == target {
                result[0] = index as i32;
                result[1] = index_to_sum as i32;
                was_target = true;

                break;
            }
        }
    }

    result.to_vec()
}

fn main() {
    println!("{:?}", two_sum([2, 7, 11, 15].to_vec(), 9));
}
