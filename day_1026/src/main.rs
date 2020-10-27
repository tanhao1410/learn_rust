use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::borrow::Borrow;

pub fn is_happy(n: i32) -> bool {
    use std::collections::HashSet;
    let mut m = HashSet::new();
    let mut nn = n;
    loop {
        if m.get(&nn).is_some() {
            return false;
        } else {
            m.insert(nn);
        }
        let mut sum = 0;
        while nn != 0 {
            sum += ((nn % 10) * (nn % 10));
            nn = nn / 10;
        }
        if sum == 1 {
            return true;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //递归方式
        let mut res = vec![];
        if let Some(node) = root {
            let mut vec1 = Solution::inorder_traversal(node.borrow_mut().left.clone());
            res.append(&mut vec1);
            res.push(node.borrow_mut().val);
            let mut vec2 = Solution::inorder_traversal(node.borrow_mut().right.clone());
            res.append(&mut vec2);
        }
        res
    }
}


fn main() {
    // let nums = vec![1,2,3,4];
    // let mut list = create_list(nums);
    //
    // let mut head = &list;
    //
    // while head.is_some(){
    //     println!("{}",head.as_ref().unwrap().val);
    //     head = &head.as_ref().unwrap().next
    // }

    f1(S { val: 1 })
}

struct S {
    val: i32,
}

fn f1(ref s: S) {}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(num: i32) -> Node {
        Node { val: num, next: None }
    }
}

fn create_list(v: Vec<i32>) -> Option<Box<Node>> {
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
