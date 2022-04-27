//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn max_depth_original(s: String) -> i32 {
        let mut ret = 0_i32;
        let mut counter = 0_i32;
        for i in s.bytes(){
            if i == b'('{
                counter += 1;
            }else if i == b')'{
                ret = std::cmp::max(ret, counter);
                //the str is guranteed to be VPS, otherwise, check value of counter here to avoid underflow
                //assert!(counter > 0);
                counter -= 1;
            }
        }
        ret
    }
    //Iter approach, could save a bit memory, I think it's because the generated code is shorter
    //P.S. Leetcode doesn't use optimized compile
    pub fn max_depth_iter(s: String) -> i32 {
        s.bytes().fold( (0,0), |(ret, counter), ch| {
            match ch{
                b'(' => (std::cmp::max(ret, counter+1), counter + 1),
                b')' => (ret, counter-1),
                _ => (ret, counter)
            }
        }).0
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
