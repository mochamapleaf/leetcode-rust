//modules

//struct define
struct Solution;
//code
impl Solution{
  pub fn largest_perimeter(mut nums: Vec<i32>) -> i32{
    nums.sort();
    nums
      .windows(3)
      .rev()
      .find_map(|sides| 
          if sides[2] < sides[1] + sides[0] { 
          Some(sides.iter().sum()) 
          } else {None})
      .unwrap_or(0)
  }
  pub fn largest_perimeter_window(mut nums: Vec<i32>) -> i32 {
    //This approach is not actually faster. Each time the window is slided, there will be an extra subtraction
    nums.sort();
    let mut per = nums[nums.len()-1] + nums[nums.len()-2] + nums[nums.len()-3];
    while nums[nums.len()-1]*2 >= per {
      if nums.len() < 4 {return 0;}
      per -= nums.pop().unwrap();
      per += nums[nums.len()-3];
    }
    per
  }
  pub fn largest_perimeter_hand(mut nums: Vec<i32>) -> i32{
    //hand implemented version, not as elegant as the iterator one
    nums.sort();
    for i in (2..nums.len()).rev(){
      if nums[i] < nums[i-1] + nums[i-2]{ return nums[i] + nums[i-1] + nums[i-2]; }
    }
    0
  }
}

//test
#[test]
fn test_solution(){
  assert_eq!(Solution::largest_perimeter_hand(vec![2,1,2]),5);
  assert_eq!(Solution::largest_perimeter_hand(vec![1,2,1]),0);
}
