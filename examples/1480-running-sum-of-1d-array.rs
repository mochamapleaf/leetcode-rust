//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

fn main() {}

#[test]
fn test_solution() { main(); }
