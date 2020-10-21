use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    let z = &y;
    assert_eq!(x,*y);
    assert_eq!(x,**z);


    let yy = Box::new(x);
    let zz = &yy;
    let zzz = &zz;
    assert_eq!(x,*yy);
    assert_eq!(x,**zz);
    assert_eq!(x,***zzz);
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