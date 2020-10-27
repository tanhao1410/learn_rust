use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::borrow::Borrow;



fn main() {
    let nums = vec![1,2,3,4];
    let mut list = create_list(nums);

    let mut head = &list;

    while head.is_some(){
        println!("{}",head.as_ref().unwrap().val);
        head = &head.as_ref().unwrap().next
    }

}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>
}

impl Node {
    pub fn new(num: i32) -> Node{
       Node{val:num,next:None}
    }
}

fn create_list(v :Vec<i32>)->Option<Box<Node>>{
    if v.is_empty() {
        return None;
    }
    let mut itr = v.iter().rev();
    //v.iter().rev
    let mut h = Some(Box::new(Node::new(*itr.next().unwrap())));
    for &ele in itr {
        h = Some(Box::new(Node {
            val: ele,
            next: h,
        }));
    }
    h
}
