//modules

//struct define
struct Solution;
//code
impl Solution {
    //This solution is O(k), where k=log(n) (k=32 in the worse case) and requires the modification of n, which is ok in rust, since it already has its ownership
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut ret: i32 = 0;
        while n != 0 {
            ret += (n & 0b1) as i32;
            n >>= 1;
        }
        ret
    }
    //A brute force solution is to loop through all 32 positions of the memory, and tally up the number of ones.
    //Here's a slightly better version requiring less loop cycle for large numbers
    pub fn hammingWeight_opt(mut n: u32) -> i32 {
        let mut ret: i32 = 0;
        while n != 0 {
            // n & (n-1) changes the least significant 1 to 0. For instance (110) & (110 - 1) = (100), (100110) & (100110 - 1) = (100100)
            n = n & (n - 1);
            ret += 1;
        }
        ret
    }
    //Important Notes
    //In fact, the real implementation of counting ones for a memory space is more complicated, and much more efficient
    //Here's an essay dedicated to this - https://arxiv.org/pdf/1611.07612.pdf
    //The optimized algorithm consider instruction-level parallelism, and many other things to speed up the performance of the operation
    //Rust implement this as intrinsic function
    //As a programmer, we should just focus on using more of these optimized functions to improve efficiency. Understanding the whole range of algorithms that does it should be the work of Computer Scientists
    pub fn hammingWeight_rust(n: u32) -> i32 {
        n.count_ones() as i32
    }
}

fn main() {}

#[test]
fn test_solution() { main(); }
