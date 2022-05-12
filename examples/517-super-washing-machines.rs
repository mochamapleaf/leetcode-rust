//modules

//struct define
struct Solution;
//code
impl Solution{
    pub fn find_min_moves(machines: Vec<i32>) -> i32{
        debug_assert!(machines.len() >= 1);
        let total_count: i32 = machines.iter().sum();
        if total_count % (machines.len() as i32) != 0{ return -1; }
        let target = total_count/machines.len() as i32;
        let mut left_sum = 0;
        let mut max_val = (machines[0] - target).abs();
        if machines.len() < 3{ return max_val; }
        for i in 0..machines.len()-2{
            left_sum += machines[i];
            let left_val = left_sum - (i+1) as i32 * target;
            let right_val = total_count - left_sum - machines[i+1]
                - (machines.len() - i - 2) as i32 * target;
            if left_val < 0 && right_val < 0{
                max_val =
                    std::cmp::max(max_val, -left_val - right_val);
            }else{
                max_val =
                    std::cmp::max(max_val,
                    std::cmp::max(left_val.abs(), right_val.abs()))
            }
        }
        max_val
    }
}

fn main() {
    assert_eq!( Solution::find_min_moves(vec![1,0,5]), 3 );
    assert_eq!( Solution::find_min_moves(vec![0,3,0]), 2 );
    assert_eq!( Solution::find_min_moves(vec![0,2]), 1 );
    assert_eq!( Solution::find_min_moves(vec![1]), 0 );
}

#[test]
fn test_solution() {
    main();
}
