//modules

//struct define
struct Solution;
//code
impl Solution {
    //Using dp, from Zuo Cheng Yun Algorithm course, 0ms, 2.2MB
    pub fn longest_valid_parentheses_dp(s: String) -> i32 {
        let mut dp = vec![0; s.len()];
        let mut sb = s.into_bytes();
        let mut max_val = 0;
        for i in 1..sb.len(){
            //if is '(', return 0
            if sb[i] == b'(' { continue; }
            let mut temp = dp[i-1];
            if i == temp{
                //Single ')', like in ())
                continue;
            }
            if sb[i-temp-1] == b'('{
                temp += 2;
                if i >= temp{ //the last matching '(' might not have a value before it ()
                    temp += dp[i-temp];
                }
                dp[i] = temp;
                max_val = std::cmp::max(max_val, temp);
            }
        }
        max_val as i32
    }
}

fn main() {}

#[test]
fn test_solution() { main(); }
