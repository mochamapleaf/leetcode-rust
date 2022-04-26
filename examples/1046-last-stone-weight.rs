struct Solution;
//code
impl Solution {
    //Recursive Version, 2ms, 2.2MB
    pub fn last_stone_weight_recursive(mut stones: Vec<i32>) -> i32 {
        fn process(mut stones: Vec<i32>) -> i32 {
            //base case: two same stones or 1 stone
            if stones.len() <= 1 {
                return stones.pop().unwrap_or(0);
            }
            stones.sort();
            let last_i = stones.len() - 1;
            stones[last_i] -= stones[last_i - 1];
            stones.swap(last_i, last_i - 1);
            stones.pop();
            if stones[last_i - 1] == 0 {
                stones.pop();
            }
            process(stones)
        }
        process(stones)
    }
    //Using heap, desired solution, 0ms, 2.1MB
    pub fn last_stone_weight_heap(stones: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(stones);
        while heap.len() > 1 {
            let diff = heap.pop().unwrap() - heap.pop().unwrap();
            if diff != 0 {
                heap.push(diff);
            }
        }
        heap.pop().unwrap_or(0)
    }
}

fn main() {
    assert_eq!(Solution::last_stone_weight_heap(vec![2, 7, 4, 1, 8, 1]), 1);
    assert_eq!(Solution::last_stone_weight_heap(vec![1]), 1);
}

#[test]
fn test_solution() {
    main();
}
