//modules

//struct define
struct Solution;
//code
impl Solution {
    //VERY SLOW, will it be optimized by compiler?
    pub fn reverse_left_words(mut s: String, n: i32) -> String {
        for _ in 0..n{
            let temp = s.remove(0);
            s.push(temp);
        }
        s
    }
    //better version, optimized with unsafe code
    pub fn reverse_left_words_unsafe(mut s: String, n: i32) -> String {
        //if n > len/2, then save the tail as buffer, otherwise, save the head
        let mut my_vec = s.into_bytes();
        let original_len = my_vec.len();
        if n as usize > my_vec.len()/2{
            my_vec.reserve_exact( original_len - n as usize);
            my_vec.extend_from_within(n as usize..);
            unsafe{
                std::ptr::copy(my_vec.as_ptr(), my_vec.as_mut_ptr().offset(original_len as isize - n as isize), n as usize);
                std::ptr::copy(my_vec.as_ptr().offset(original_len as isize), my_vec.as_mut_ptr(), original_len - n as usize);
                my_vec.truncate(original_len);
                String::from_utf8_unchecked(my_vec)
            }
            
        }else{
            my_vec.reserve_exact(n as usize);
            my_vec.extend_from_within(..n as usize);
            unsafe{
                std::ptr::copy(my_vec.as_ptr().offset(n as isize), my_vec.as_mut_ptr(), original_len - n as usize);
                std::ptr::copy(my_vec.as_ptr().offset(original_len as isize), my_vec.as_mut_ptr().offset(original_len as isize - n as isize), n as usize);
                my_vec.truncate(original_len);
                String::from_utf8_unchecked(my_vec)
            }
        }
    }
    //but first expanding the capacity of Vec will cause it to reallocate, causing unnecessary moves
    //Therefore, using a separate Vec to store the buffered info will be better
    pub fn reverse_left_words_unsafe_opt(mut s: String, n: i32) -> String {
        //if n > len/2, then save the tail as buffer, otherwise, save the head
        let mut my_vec = s.into_bytes();
        if n as usize > my_vec.len()/2{
            //clone slice as Vec
            let mut temp_vec = my_vec[n as usize..].to_vec();
            unsafe{
                std::ptr::copy(my_vec.as_ptr(), my_vec.as_mut_ptr().offset(temp_vec.len() as isize), n as usize);
                std::ptr::copy(temp_vec.as_ptr(), my_vec.as_mut_ptr(), temp_vec.len());
                String::from_utf8_unchecked(my_vec)
            }
        }else{
            let mut temp_vec = my_vec[.. n as usize].to_vec();
            unsafe{
                std::ptr::copy(my_vec.as_ptr().offset(n as isize), my_vec.as_mut_ptr(), my_vec.len() - n as usize);
                std::ptr::copy(temp_vec.as_ptr(), my_vec.as_mut_ptr().offset(my_vec.len() as isize - n as isize), n as usize);
                String::from_utf8_unchecked(my_vec)
            }
        }
    }
    //There's one solution that append itself to itself, but it requires more capacity and more memory, which is worse than the last one
    pub fn reverse_left_words_unsafe_bad(mut s: String, n: i32) -> String {
        let mut my_vec = s.into_bytes();
        let original_len = my_vec.len();
        my_vec.reserve_exact(my_vec.len()); //first copy happens here, copied len items
        my_vec.extend_from_within(..); //second copy happens here, copied len items
        unsafe{
            std::ptr::copy(my_vec.as_ptr().offset(n as isize), my_vec.as_mut_ptr(), original_len); //third copy here, copied len items
            my_vec.set_len(original_len);
            String::from_utf8_unchecked(my_vec)
        }
    }
    //There's also a solution with O(1) memory, but O(n) time
    //This solution rotates the string in place, requiring O(1) buffer space
    pub fn reverse_left_words_no_buffer(mut s: String, n: i32) -> String {
        let mut my_vec = s.into_bytes();
        my_vec[..n as usize].reverse();
        my_vec[n as usize..].reverse();
        my_vec.reverse();
        unsafe{
            String::from_utf8_unchecked(my_vec)
        }
    }
}

//test
#[test]
fn test_solution(){
}
