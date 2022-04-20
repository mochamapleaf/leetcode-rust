//modules

//struct define
struct Solution;
//code
impl Solution
    //Loop twice, O(N)
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut my_map = vec![0_i32; 100]; //use a vector to act as HashMap
        for &i in &nums{
            my_map[i as usize - 1] += 1;
        }
        let mut ret = 0_i32;
        for i in k as usize..100{
            ret += my_map[i] * my_map[i - k as usize];
        }
        ret
    }
    //use HashMap
    pub fn count_k_difference_hashmap(nums: Vec<i32>, k: i32) -> i32 {
        let mut my_map = std::collections::HashMap::<i32, i32>::new();
        for &i in &nums{
            my_map.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut ret = 0_i32;
        for &v in my_map.keys(){
            ret += my_map.get(&(v-k)).unwrap_or(&0_i32) * my_map.get(&v).unwrap();
        }
        ret
    }
    //only loop once
    pub fn count_k_difference_once(nums: Vec<i32>, k: i32) -> i32 {
        let mut my_map = std::collections::HashMap::<i32, i32>::new();
        let mut ret = 0_i32;
        for &i in &nums{
            ret += my_map.get(&(i+k)).unwrap_or(&0);
            ret += my_map.get(&(i-k)).unwrap_or(&0);
            my_map.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }
        ret
    }
}
//test
#[test]
fn test_solution(){
}
