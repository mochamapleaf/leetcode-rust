//modules

//struct define
struct Solution;
//code
impl Solution {
    //My solution, using pattern matching
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut ret = 0_i32;
        for op in &operations {
            match &**op {
                //&String => String => str => &str
                "++X" | "X++" => {
                    ret += 1;
                }
                _ => {
                    //should be "--X" | "X--", but we could exploit the fact that operations[i] is one of the four
                    ret -= 1;
                }
            }
        }
        ret
    }
    //Optimized solution, only look for the symbol in the second place
    pub fn final_value_after_operations_opt(operations: Vec<String>) -> i32 {
        let mut ret = 0_i32;
        for op in &operations {
            let tmp = op.as_bytes();
            ret += (tmp[1] == b'+') as i32 * 2;
        }
        ret - operations.len() as i32
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
