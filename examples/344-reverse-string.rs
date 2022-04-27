//modules

//struct define
struct Solution;
impl Solution {
    //This is the approach that std uses
    //Just swap first with last, then second with second last ...
    //Std implements some tricks to speed up
    pub fn reverse_string(s: &mut Vec<char>) {
        let length = s.len();
        for i in 0..length>>1 { s.swap(i, length-i-1); }
    }
    pub fn reverse_string_std(s: &mut Vec<char>){
        s.reverse();
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
