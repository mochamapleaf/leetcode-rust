//modules

//struct define
struct SortedStack {
    stack: Vec<i32>,
}

impl SortedStack {
    //When pushing a new item, peek the stack to see if it can be pushed directly, if not, push the
    //items in the temporary stack, and later push them back
    fn new() -> Self {
        Self { stack: Vec::new() }
    }
    fn push(&mut self, val: i32) {
        let mut temp_stack = Vec::new();
        while let Some(&x) = self.stack.last() {
            if x >= val {
                break;
            }
            temp_stack.push(self.stack.pop().unwrap());
        }
        self.stack.push(val);
        while !temp_stack.is_empty() {
            self.stack.push(temp_stack.pop().unwrap());
        }
    }
    fn pop(&mut self) {
        self.stack.pop();
    }
    fn peek(&self) -> i32 {
        match self.stack.last() {
            Some(&x) => x,
            None => -1,
        }
    }
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

/**
 * Your SortedStack object will be instantiated and called as such:
 * let obj = SortedStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.is_empty();
 */

fn main() {}

#[test]
fn test_solution() {
    main();
}
