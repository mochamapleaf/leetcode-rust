//modules

//struct define
struct Solution;
//code
/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    //These two solutions have the same idea, just using different control flows
    unsafe fn guessNumber_while_if(n: i32) -> i32 {
        let mut start = 1_i32;
        let mut end = n;
        while end > start{
            let mid = start + ((end-start + 1)>>1);
            if guess(mid) >=0{ //evaluating >= will be faster, the compiler can directly look at the signbit
                start = mid;
            }else{
                end = mid-1;
            }
        }
        start
    }
    unsafe fn guessNumber_loop_match(n: i32) -> i32 {
        let mut start = 1_i32;
        let mut end = n;
        loop{
            let mid = start + ((end-start)>>1);
            match guess(mid){
                -1 =>{
                    end = mid-1;
                },
                1 =>{
                    start = mid+1;
                },
                _ =>{
                    //0
                    return mid;
                }
            }
        }
    }
}

//test
#[test]
fn test_solution(){
}
