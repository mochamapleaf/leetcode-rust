//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut my_set = std::collections::HashMap::new();
        for (i,v) in nums.iter().enumerate(){
            if my_set.contains_key(&(target-v)){
                return vec![my_set[&(target-v)] as i32, i as i32];
            }
            my_set.insert(v,i);
        }
        vec![]
    }
}

//test
#[test]
fn test_solution(){
}
