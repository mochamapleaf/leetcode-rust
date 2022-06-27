//modules

//struct define
struct Solution;

impl Solution{
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>>{
        fn backtrack(nums: &Vec<i32>, cur: usize, ret: &mut Vec<Vec<i32>>, ret_buf: &mut Vec<i32>){
            ret.push(ret_buf.to_owned());
            //base case
            if cur >= nums.len(){
                return;
            }
            for i in (cur as usize)..nums.len(){
                if i != 0 && nums[i] == nums[i-1]{ //这里利用nums之前已经排序好的特性，可以省掉一个HashMap进行查重
                    continue;
                }
                ret_buf.push(nums[i]);
                backtrack(nums, i + 1, ret, ret_buf);
                ret_buf.pop();
            }
        }
        let mut ret: Vec<Vec<i32>> = Vec::new();
        let mut ret_buf = Vec::with_capacity(nums.len());
        nums.sort();
        backtrack(&nums, 0, &mut ret, &mut ret_buf);
        ret
    }
}

fn main() {
    println!("{:?}",Solution::subsets_with_dup(vec![4,4,4,1,4]));
}

#[test]
fn test_solution() {
    main();
}
