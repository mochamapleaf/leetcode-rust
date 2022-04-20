//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0_usize;
        let mut end = nums.len()-1;
        while start as i32 <= end as i32{ //This covers the edge case that target is smaller/greater than the min/max of the vector, in which case mid will overflow
	//The conversion above acts as a cast, which let the compiler interpretate start and end as i32 values rather than usize
        //This also solves the case where len = 1
            let mid = start + ((end - start) >> 1);
            if nums[mid] < target{
                start = mid+1;
            }else if nums[mid] > target{
                end = mid - 1;
            }else{
                return mid as i32;
            }
        }
        -1
    }
    //From rust std library
    pub fn search_std(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target){
            Ok(x) => x as i32,
            _ => -1 
        }
    }
}

//test
#[test]
fn test_solution(){
}
