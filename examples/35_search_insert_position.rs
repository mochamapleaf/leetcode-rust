//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn search_insert_my_solution(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0_usize;
        let mut end = nums.len() - 1;
        //handle 2 edge cases where target isn't in vector
        if target < nums[0] {
            return 0;
        }
        if target > *nums.last().unwrap() {
            return nums.len() as i32;
        }
        //split line: val < target |split| val > target
        //which to choose: [a,b] if a < target < b => b (a |split| b)
        while end as i32 > start as i32 {
            let mid = start + ((end - start) >> 1);
            if nums[mid] > target {
                end = mid;
            } else if nums[mid] < target {
                start = mid + 1;
            } else {
                //target found
                return mid as i32;
            }
        }
        start as i32
    }
    pub fn search_insert_std_source(nums: Vec<i32>, target: i32) -> i32 {
        //This is almost identical to how std does slice.binary_search
        let mut start = 0;
        let mut end = nums.len(); //IMPORTANT - The fact that here isn't len()-1 makes it possible to not consider the edge cases
        while end as i32 > start as i32 {
            let mid = start + ((end - start) >> 1); //We never get the value of nums[nums.len()], so setting end = nums.len() won't be problematic
                                                    //std checks < first, but it doesn't matter
            if nums[mid] < target {
                start = mid + 1;
                //because we initialized end to nums.len(), if the target is larger than the largest element, then start will end up at start == end == nums.len(), which is what we want.
            } else if nums[mid] > target {
                end = mid;
            } else {
                //target found
                return mid as i32;
            }
        }
        start as i32
    }
    //Using std, turns out to be saving a little memory
    pub fn search_insert_std(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(x) => x as i32,
            Err(x) => x as i32,
        }
    }
}

fn main() {}

#[test]
fn test_solution() { main(); }
