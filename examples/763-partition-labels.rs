//modules

//struct define
struct Solution;
//code
impl Solution {
    //tags: greedy algorithms, HashMap

    //Brute force, 4ms, 1.9MB
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut ret_vec = Vec::new();
        let my_s = s.into_bytes();
        let mut start = 0;
        while start < my_s.len() {
            let mut right_limit = my_s.len() - 1;
            while my_s[right_limit] != my_s[start] {
                right_limit -= 1;
            }
            let mut i = start + 1;
            while i < right_limit {
                // find rightmost character that's the same
                for k in (right_limit + 1..my_s.len()).rev() {
                    if my_s[k] == my_s[i] {
                        right_limit = k;
                        break;
                    }
                }
                i += 1;
            }
            ret_vec.push((right_limit - start) as i32 + 1);
            start = right_limit + 1;
        }
        ret_vec
    }
    //Optimize using Dict (Doesn't need hashmap, since we have only 26 characters)
    //0ms, 2MB
    pub fn partition_labels_opt(s: String) -> Vec<i32> {
        let s = s.into_bytes();
        let mut dict_vec = vec![usize::MAX; 26]; //the initial value doesn't matter, if we need it, we have it
        let mut ret_vec = Vec::new();
        for i in 0..s.len() {
            dict_vec[(s[i] - b'a') as usize] = i;
        }
        let mut start = 0;
        while start < s.len() {
            let mut last = dict_vec[(s[start] - b'a') as usize];
            let mut i = start + 1;
            while i < last {
                last = std::cmp::max(dict_vec[(s[i] - b'a') as usize], last);
                i += 1;
            }
            ret_vec.push((last - start) as i32 + 1);
            start = last + 1;
        }
        ret_vec
    }
}

fn main() {
    assert_eq!(
        Solution::partition_labels_opt("ababcbacadefegdehijhklij".to_string()),
        vec![9, 7, 8]
    );
    assert_eq!(
        Solution::partition_labels_opt("caedbdedda".to_string()),
        vec![1, 9]
    );
}

#[test]
fn test_solution() {
    main();
}
