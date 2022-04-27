//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut count = 0_i32;
        let mut ret = 0_i32;
        for i in s.bytes(){
            if i == b'R'{ count -= 1;}
            else {count += 1;}
            ret += (count == 0) as i32;
        }
        ret
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
