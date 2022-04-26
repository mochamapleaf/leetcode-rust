//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        fn process(coins: &Vec<i32>, coin_i: usize, amount: i32) -> i32 {
            //base case
            if coin_i == 0 {
                if amount % coins[0] == 0 {
                    return amount / coins[0];
                } else {
                    return i32::MAX;
                }
            }
            let mut min = i32::MAX;
            for i in 0..amount / coins[coin_i] {
                min = std::cmp::min(
                    min,
                    process(coins, coin_i - 1, amount - i * coins[coin_i]).saturating_add(i),
                );
            }
            min
        }
        match process(&coins, coins.len() - 1, amount) {
            i32::MAX => -1,
            x => x,
        }
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
