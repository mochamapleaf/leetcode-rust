//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let mut pro: i32 = 1;
        let mut tally: i32 = 0;
        while n != 0{
            pro *= n%10;
            tally += n%10;
            n /= 10;
        }
        pro - tally
    }
}

//test
#[test]
fn test_solution(){
}
