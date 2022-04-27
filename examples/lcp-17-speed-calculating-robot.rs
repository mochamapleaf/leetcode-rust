//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let (a,b) = s.bytes().fold( (1,0), | (x, y), ch | {
            match ch{
                b'A' => { (2*x+y, y) },
                _ => { (x, 2*y+x) }
            }
        });
        (a+b) as i32
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
