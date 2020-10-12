use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x,*y);
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}