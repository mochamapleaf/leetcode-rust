//modules

//struct define
struct Solution;
//code
impl Solution {
    //The answer equals to the max value from the string
    pub fn min_partitions(n: String) -> i32 {
        n.bytes().map(|i| (i-b'0') as i32).max().unwrap()
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
