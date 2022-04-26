//modules

//code
use rand::Rng;
struct Solution {
    list: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    //My original solution
    fn new_original(nums: Vec<i32>) -> Self {
        Solution{
            list: nums,
            rng: rand::thread_rng(),
        }
    }
    fn pick_original(&mut self, target: i32) -> i32 {
        let mut search_res = Vec::new();
        for i in 0..self.list.len(){
            if self.list[i] == target{
                search_res.push(i);
            }
        }
        if search_res.len() == 1{ return search_res[0] as i32;}
        search_res[self.rng.gen::<usize>() % (search_res.len())] as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */

fn main() {}

#[test]
fn test_solution() { main(); }
