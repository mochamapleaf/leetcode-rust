//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn minimum_sum(mut num: i32) -> i32 {
        let mut my_vec = Vec::with_capacity(4);
        for _ in 0..4 {
            my_vec.push(num % 10);
            num /= 10;
        }
        my_vec.sort();
        my_vec[3] + my_vec[2] + 10 * (my_vec[1] + my_vec[0])
    }
}

fn main(){

}

#[test]
fn test_solution(){
    main();
}
