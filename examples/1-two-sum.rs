//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut my_set = std::collections::HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            if my_set.contains_key(&(target - v)) {
                return vec![my_set[&(target - v)] as i32, i as i32];
            }
            my_set.insert(v, i);
        }
        vec![]
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}

#[test]
fn test_solution() {
    main();
}
