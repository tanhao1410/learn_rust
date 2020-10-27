mod handle;


use std::net::TcpListener;
use std::io;
mod controller{
    pub(crate) mod route;
}
use controller::route as route;
fn main2() {
    // let listener = TcpListener::bind("127.0.0.1:8090").unwrap();
    // for i in listener.incoming(){
    //     let stream = i.unwrap();
    //     handle::handle_connection(stream);
    // }
    // route::route();


}

trait Trait {}

fn foo<X: Trait>(t: X) {}

impl<'a> Trait for &'a i32 {}

fn main() {
    let t: & i32 = & 0;
    foo(t);
}

