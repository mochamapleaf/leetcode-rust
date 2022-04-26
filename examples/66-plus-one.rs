//modules

//struct define
struct Solution;
//code
impl Solution {
    //my solution
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            if digits[i] != 9 {
                digits[i] += 1;
                return digits;
            } else {
                digits[i] = 0;
            }
        }
        digits.insert(0, 1); //Handle the case that requires one more element, like [9], [9,9] ...
        digits
    }
    //Official solution: only 9 will cause a carry, thus we have 3 cases
    //a. no 9 at the end, simply add 1
    //b. a series of 9 at the end => add 1 to the last element that != 9
    //c. All 9 => return new Vec
    //I feel my solution is better...
    //Maybe check the acm of each code later
    pub fn plus_one_official(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            if digits[i] != 9 {
                digits[i] += 1;
                for k in (i + 1..digits.len()) {
                    digits[k] = 0;
                }
                return digits;
            }
        }
        let mut ret = Vec::with_capacity(digits.len() + 1);
        ret.push(1);
        ret.resize(digits.len() + 1, 0);
        ret
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
