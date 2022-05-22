//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
        //more optimized version
        //n.count ones() == 1 && (n & 0x55555555) != 0
        //more optimized version
        //(n > 0) && (n & 0x2aaaaaaa == 0) && (n & -n) == n
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
