use std::rc::Rc;
use std::collections::{LinkedList, BTreeMap};
use std::ops::Add;
use std::cell::RefCell;
use std::borrow::Borrow;

static a: i32 = 12;
const b: i32 = 13;


fn main() {
    println!("Hello, world!");

    let vec1: Vec<i32> = vec![1, 2, 3];
    assert_eq!(&vec1[..], &[1, 2, 3]);

    let s = "一个字符串";
    let s2 = String::from("字符串2");

    let s22: &str = s2.as_str();
    //s2.c

    let sss = s.to_string();

    //std::slice::from_raw_parts()
    //std::str::from_utf8()
    //std::slice::Chunks::

    let tuple = (1, "ss", true);
    assert_eq!(tuple.0, 1);
    assert_eq!(tuple.1, "ss");

    //let结构元组
    let (x, y) = (1, 2);
    //(..)
    //std::ops::RangeFull
    let mut nums = vec![4, 5, 7, 1, 4, 2, 7, 3];
    //sort_vec(&mut nums);
    println!("{:?}", nums);

    let str1 = "xxxx";
    let ptr = str1.as_ptr();
    println!("{:p}", ptr);
    ptr_common_fat();

    let rc1 = Rc::new(vec![1, 2, 3]);
    println!("{}", (*rc1)[1]);
    println!("{}", rc1[1]);

    let x = "xxx".to_string();
    match x {
        String => println!("dd"),
        _ => println!("ddd"),
    }

    let sss: String = "xxxx".into();
    let p: People = People::from(String::from("xxxx"));

    println!("{:?}", p);
    println!("{}", a);

    let end = Node { val: 1, next: None };
    let end_rc = Rc::new(RefCell::new(end));


    let end2 = Node { val: 2, next: Some(end_rc.clone()) };//end2  -->end

    //end_rc.

    let mut first = Node { val: 3, next: None };
    first.next = Some(Rc::new(RefCell::new(end2)));

    //让end指向first
    end_rc.borrow_mut().next = Some(Rc::new(RefCell::new(first)));

    println!("{:?}", end_rc);

    let mut n = Some(10);
    while let Some(s) = n {
        if s > 2{
            n = Some(s -1);
            println!("...");
        }else{
            break
        }
    }
    println!("---");

    let  mut n2 = 32;
    let f = move || ->i32{
        n2
    };
    f();
    println!("{}",n2);

}


struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}


trait Swim {
    fn swim();
}

union UnionT {
    f: i32,
    k: i64,
}

impl From<String> for People {
    fn from(s: String) -> Self {
        People { name: s, age: 0 }
    }
}

enum Number {
    One,
    Two,
    Three,
    Four(u8),
}

struct Integer(i32);

type Int = i32;

#[derive(Clone)]
struct SizedTest {
    name: String,
    unsized1: [i32; 4],
}

trait Parent {
    fn parent();
}

trait Child: Parent {
    fn child();
}

fn my_sum<T: Add<Output=T>>(t1: T, t2: T) -> T {
    t1 + t2
}

struct Empty;

struct Point(i32, i32);

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
struct People {
    pub name: String,
    pub age: u8,
}

impl People {
    pub fn new(name: String, age: u8) -> Self {
        People { name, age }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn read_file() {
    //std::fs::
    //std::io::
    //std::fs::File
    let mut list = LinkedList::new();
    list.push_back(1);
    let mut map = BTreeMap::new();
    map.insert(String::from("key"), 32);
}

fn ptr_common_fat() {
    let size1 = std::mem::size_of::<&[u32; 5]>();
    //let size2 = std::mem::size_of::<&mut [u32]>();
    let size2 = std::mem::size_of::<Vec<()>>();
    println!("{}", format!("{}-{}", size1, size2));

    let num_str = "123";
    let num = num_str.parse::<i32>().unwrap();
}

fn sort_vec(nums: &mut Vec<i32>) {
    let box1 = Box::new(People { name: "11".to_string(), age: 0 });
    println!("{}", box1.name);

    let mut temp = 0;
    for i in 0..nums.len() - 1 {
        for j in i..nums.len() {
            if nums[i] > nums[j] {
                temp = nums[i];
                nums[i] = nums[j];
                nums[j] = temp;
            }
        }
    }
}