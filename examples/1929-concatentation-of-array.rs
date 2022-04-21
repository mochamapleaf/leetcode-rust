//modules

//struct define
struct Solution;
//code
impl Solution {
    //using std
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        nums.extend_from_within(..);
        nums
    }
    //implement with unsafe, extend_from_within uses a different approach, to suit more cases
    pub fn get_concatenation_unsafe(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(nums.len() * 2);
        unsafe {
            std::ptr::copy(nums.as_ptr(), ret.as_mut_ptr(), nums.len());
            std::ptr::copy(
                nums.as_ptr(),
                ret.as_mut_ptr().offset(nums.len() as isize),
                nums.len(),
            );
            ret.set_len(nums.len() * 2);
        }
        ret
    }
}

fn main(){

}

#[test]
fn test_solution(){
    main();
}
