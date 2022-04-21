//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        //4 cases for low and high
        //low-odd high-odd
        //(high-low+2)/2 <= (high-low)/2 + 1
        //low-odd high-even (high-low must be odd)
        //(high-low+1)/2 <= (high-low)/2 + 1
        //low-even high-odd (high-low must be odd)
        //(high-low+1)/2 <= (high-low)/2 + 1
        //low-even high-even
        //(high-low)/2
        ((hight - low) + low % 2 + high % 2) / 2
    }
}

fn main(){

}

#[test]
fn test_solution(){
    main();
}
