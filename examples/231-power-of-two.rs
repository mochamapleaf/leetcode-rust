//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n.count_ones() == 1 && n.is_positive() //Special case: i32::MIN, also has a single 1 in binary representation
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
