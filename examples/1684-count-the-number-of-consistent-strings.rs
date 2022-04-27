//modules

//struct define
struct Solution;
//code
impl Solution {
    //original solution, using HashSet and labeled loop
    pub fn count_consistent_strings_original(allowed: String, words: Vec<String>) -> i32 {
        let mut ret = 0_i32;
        //load 'allowed' to HashSet
        let mut my_set = std::collections::HashSet::<u8>::new();
        for i in allowed.bytes(){
            my_set.insert(i);
        }
        'outer: for i in words{
            for k in i.bytes(){
                if !my_set.contains(&k){
                    continue 'outer;
                }
            }
            ret += 1;
        }
        ret
    }
    //Iterator and Vector Set approach
    impl Solution {
        pub fn count_consistent_strings_iter(allowed: String, words: Vec<String>) -> i32 {
            let mut my_set: [bool;26] = [false; 26];
            allowed.bytes().for_each(|i| {my_set[(i-b'a') as usize] = true;} );
            words.iter().filter(|s| s.bytes().all(|b| my_set[(b-b'a') as usize])).count() as i32
        }
    }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
