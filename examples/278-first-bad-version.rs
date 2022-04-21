struct Solution;
//code

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut start = 0;
        let mut end = n as usize;
        while end > start {
            let mid = start + ((end - start) >> 1);
            if self.isBadVersion(mid as i32) {
                end = mid;
            } else {
                start = mid + 1;
            }
        }
        start as i32
    }
}

fn main() {}

#[test]
fn test_solution() { main(); }
