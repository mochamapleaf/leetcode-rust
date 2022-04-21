//modules
//struct define
struct Solution;
//code
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut ret = 0_i32;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                ret += nums[i..j].iter().fold(0_i32, |xor, &x| xor ^ x);
            }
        }
        ret
    }
}

fn main(){

}

#[test]
fn test_solution(){
    main();
}
