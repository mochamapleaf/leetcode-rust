//modules

//struct define
struct Solution;
//code
impl Solution {
    //Only need to modify the places that are digits
    pub fn replace_digits(s: String) -> String {
        let mut my_s = s.into_bytes();
        for i in 0..my_s.len()>>1{
            my_s[i*2+1] += my_s[i*2] - b'0';
        }
        unsafe{ String::from_utf8_unchecked(my_s) }
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
