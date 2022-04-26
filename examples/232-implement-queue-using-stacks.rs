//Method 1: use one stack, every time an item is pushed, store the items in temporary stack,
//push the new item to the bottom of the stack.

//Method 2: Two stacks, when the pop takes place, pop from pop_stack, if it's empty, then move all
//items from push_stack to pop_stack
//This is almost the same implementation as VecDeque, but VecDeque uses a more efficient approach
//0ms, 2.2MB
struct MyQueue_double_stack {
    pop_stack: Vec<i32>,
    push_stack: Vec<i32>,
}
impl MyQueue_double_stack {
    fn new() -> Self {
        Self{
            pop_stack: Vec::new(),
            push_stack: Vec::new()
        }
    }
    fn push(&mut self, x: i32) { self.push_stack.push(x); }
    fn pop(&mut self) -> i32 {
        if self.pop_stack.is_empty(){
            while !self.push_stack.is_empty(){
                self.pop_stack.push(self.push_stack.pop().unwrap());
            }
        }
        self.pop_stack.pop().unwrap_or(-1)
    }
    fn peek(&mut self) -> i32 {
        if self.pop_stack.is_empty(){
            while !self.push_stack.is_empty(){
                self.pop_stack.push(self.push_stack.pop().unwrap());
            }
        }
        *self.pop_stack.last().unwrap_or(&-1)
    }
    fn empty(&self) -> bool { self.pop_stack.is_empty() && self.push_stack.is_empty() }
}

//Method 3
//std::collections::VecDeque
//0ms, 2MB
struct MyQueue_deque {
    deque: std::collections::VecDeque<i32>,
}
impl MyQueue {
    fn new() -> Self { Self { deque: std::collections::VecDeque::new(), } }
    fn push(&mut self, x: i32) { self.deque.push_back(x); }
    fn pop(&mut self) -> i32 { self.deque.pop_front().unwrap_or(-1) }
    fn peek(&self) -> i32 { *self.deque.front().unwrap_or(&-1) }
    fn empty(&self) -> bool { self.deque.is_empty() }
}

fn main() {}

#[test]
fn test_solution() {
    main();
}
