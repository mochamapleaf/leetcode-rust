//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(nums.len());
        for &i in &nums {
            ret.push(nums[i as usize]);
        }
        ret
    }
    //Official solution provides a way that does not use any extra space
    //since 0<= nums[i] < nums.length, nums[i] < 1000
    //We can store the new value by using number hack
    //For each number ######, the first 3 '#' is the new value, and the last 3 '#' is the old value
    //This way, we could store both the original info and the new info in the same position
    pub fn build_array_mem_opt(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            nums[i] += 1000 * (nums[nums[i] as usize] % 1000);
        }
        nums.iter_mut().for_each(|x| *x /= 1000);
        nums
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
