//modules

//struct define
struct Solution;
//code
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        //encoded[i] = arr[i] ^ arr[i+1]
        //Then
        //arr[i+1] = encoded[i] ^ arr[i]
        let mut ret = Vec::with_capacity(encoded.len()+1);
        ret.push(first);
        let mut last = first;
        for i in &encoded{
            last = *i ^ last;
            ret.push(last); //repeated addition of length caused by repeated push can be optimized
        }
        ret
    }
}

//test
#[test]
fn test_solution(){
}
