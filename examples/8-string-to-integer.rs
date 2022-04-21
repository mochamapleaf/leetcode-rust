//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut is_neg = false;
        let mut s_iter = s.bytes();
        let mut temp_num: i32 = 0;
        //trim all leading whitespace characters, until it sees '+' or '-' or a number
        while let Some(x) = s_iter.next() {
            if x == b' ' {
                continue;
            }
            //skip if whitespace
            else if x == b'-' {
                is_neg = true;
                break;
            } else if x == b'+' {
                break;
            }
            //remember the symbol, and exit
            else if x.is_ascii_digit() {
                //no sign meaning the sign is '+' and omitted
                temp_num = (x - b'0') as i32; //store the number
                break;
            } else {
                return 0;
            }
        }
        if is_neg {
            temp_num = -temp_num;
        } //if the number is negative, we need to use '-' every time
          //if we operate on the positive number, and apply the '-' at the end, then we can't handle i32::MAX, since we can't tell which one is -2^32+1 (-i32::MAX), and which one is -2^32
        while let Some(x) = s_iter.next() {
            if x.is_ascii_digit() {
                temp_num = temp_num.saturating_mul(10);
                if is_neg {
                    temp_num = temp_num.saturating_sub((x - b'0') as i32);
                } else {
                    temp_num = temp_num.saturating_add((x - b'0') as i32);
                }
            } else {
                break;
            }
        }
        temp_num
    }
}

fn main() {
    assert_eq!(Solution::my_atoi("42".to_string()), 42_i32);
    assert_eq!(Solution::my_atoi("   -42".to_string()), -42_i32);
    assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(
        Solution::my_atoi("-2147483647".to_string()),
        -2147483647_i32
    );
    assert_eq!(Solution::my_atoi("+-12".to_string()), 0_i32);
    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0_i32);
}

#[test]
fn test_solution() { main(); }
