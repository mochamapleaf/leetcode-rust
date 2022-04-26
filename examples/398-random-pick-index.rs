use rand::Rng;
struct Solution_original {
    list: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}
impl Solution_original {
    //My original solution
    fn new(nums: Vec<i32>) -> Self {
        Solution {
            list: nums,
            rng: rand::thread_rng(),
        }
    }
    fn pick(&mut self, target: i32) -> i32 {
        let mut search_res = Vec::new();
        for i in 0..self.list.len() {
            if self.list[i] == target {
                search_res.push(i);
            }
        }
        if search_res.len() == 1 {
            return search_res[0] as i32;
        }
        search_res[self.rng.gen::<usize>() % (search_res.len())] as i32
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
