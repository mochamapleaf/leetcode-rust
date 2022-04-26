//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max = 0_usize;
        for str in &sentences {
            max = std::cmp::max(max, str.bytes().filter(|x| *x == b' ').count());
        }
        max as i32 + 1
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
