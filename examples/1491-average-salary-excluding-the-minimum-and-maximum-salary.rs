//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut max_amount: i32 = salary[0];
        let mut min_amount: i32 = salary[0];
        let mut sum: i32 = salary[0];
        for i in salary.iter().skip(1) {
            max_amount = std::cmp::max(*i, max_amount);
            min_amount = std::cmp::min(*i, min_amount);
            sum += *i;
        }
        (sum - max_amount - min_amount) as f64 / (salary.len() - 2) as f64
    }
    pub fn average_iter(salary: Vec<i32>) -> f64 {
        (salary.iter().sum::<i32>() - salary.iter().max().unwrap() - salary.iter().min().unwrap())
            as f64
            / (salary.len() - 2) as f64
    }
}

fn main() {
    assert_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500.0);
    assert_eq!(Solution::average(vec![1000, 2000, 3000]), 2000.0);
}

#[test]
fn test_solution() {
    main();
}
