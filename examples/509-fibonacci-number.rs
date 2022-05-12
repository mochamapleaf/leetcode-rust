//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 { return 0;}
        let mut mat: [i32; 4] = [0,1,1,1];
        let mut pow = n as u32;
        let mut target_mat = [1,0,0,1];
        while pow != 0{
            if pow % 2 == 1{
                target_mat = [ target_mat[0]*mat[0] + target_mat[1]*mat[2], target_mat[0]*mat[1] + target_mat[1]*mat[3],
                target_mat[2]*mat[0] + target_mat[3]*mat[2], target_mat[2]*mat[1] + target_mat[3]*mat[3] ];
            }
            pow >>= 1;
            mat = [mat[0]*mat[0] + mat[1]*mat[2], mat[1]*(mat[0] + mat[3]),
                mat[2]*(mat[0] + mat[3]), mat[3]*mat[3]+mat[1]*mat[2]];
        }
        target_mat[2]
    }
}

fn main() {
    assert_eq!(Solution::fib(5), 5);
}

#[test]
fn test_solution() {
    main();
}
