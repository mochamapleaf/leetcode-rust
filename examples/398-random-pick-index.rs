use rand::Rng;
struct Solution_original {
    list: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}
impl Solution_original {
    //My original solution
    fn new(nums: Vec<i32>) -> Self {
        Solution_original {
            list: nums,
            rng: rand::thread_rng(),
        }
    }
    fn pick(&mut self, target: i32) -> i32 {
        let mut search_res = Vec::new();
        for i in 0..self.list.len() {
            if self.list[i] == target {
                search_res.push(i);
            }
        }
        if search_res.len() == 1 {
            return search_res[0] as i32;
        }
        search_res[self.rng.gen::<usize>() % (search_res.len())] as i32
    }
}

//use rand::Rng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::SmallRng;

struct Solution_Hash {
    my_map: std::collections::HashMap<i32, Vec<usize>>,
    rng: rand::rngs::SmallRng,
    //rng: rand::rngs::ThreadRng,
}
impl Solution_Hash {
    fn new(nums: Vec<i32>) -> Self {
        let mut tmp = std::collections::HashMap::new();
        for i in 0..nums.len(){
            let mem_list = tmp.entry(nums[i]).or_insert(Vec::<usize>::new());
            (*mem_list).push(i);
        }
        Solution_Hash {
            my_map: tmp,
            rng: SmallRng::from_entropy(),
            //rng: rand::thread_rng(),
        }
    }
    fn pick(&mut self, target: i32) -> i32 {
        *self.my_map.get(&target).unwrap().choose(&mut self.rng).unwrap() as i32
        //let choices = self.my_map.get(&target).unwrap();
        //if choices.len() ==  1{ return choices[0] as i32;}
        //choices[self.rng.gen_range(0,choices.len())] as i32
    }
}

//use rand::Rng;
use rand::seq::IteratorRandom;
struct Solution_oneline_iter {
    list: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}
impl Solution_oneline_iter {
    //My original solution
    fn new(nums: Vec<i32>) -> Self {
        Solution_oneline_iter {
            list: nums,
            rng: rand::thread_rng(),
        }
    }
    fn pick(&mut self, target: i32) -> i32 {
        self.list.iter().enumerate().filter(|&v| *v.1 == target).choose(&mut self.rng).unwrap().0 as i32//target必然在list中，直接unwrap
    }
}

//use rand::Rng;
struct Solution_Reservoir_Sampling {
    list: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}
impl Solution_Reservoir_Sampling {
    fn new(nums: Vec<i32>) -> Self {
        Solution_Reservoir_Sampling {
            list: nums,
            rng: rand::thread_rng(),
        }
    }
    fn pick(&mut self, target: i32) -> i32 {
        let mut ret = self.list.iter().position(|&v| v == target).unwrap(); //target一定在数组中，直接unwrap
        let mut counter = 3;
        for i in (ret + 1)..self.list.len() {
            if self.list[i] == target{
                if self.rng.gen_range(0..counter) == 0{ // P(1/(counter-1))
                    ret = i;
                }
                counter += 1;
            }
        }
        ret as i32
    }
}


fn main() {
    let test_list = vec![1,2,3,3,3];
    let test_val = vec![3;1000];
    let mut test_struct = Solution_oneline_iter::new(test_list.clone());
    //print!("[");
    for &v in test_val.iter(){
        let result = test_struct.pick(v);
        assert_eq!(test_list[result as usize], v);
        //print!("{}, ", result);
    }
    //println!("]");
}

#[test]
fn test_solution() {
    main();
}
