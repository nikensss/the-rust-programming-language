use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));

    let c_message;
    {
        let c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("count after creating c = {}", Rc::strong_count(&a));
        c_message = format!("c is {:?}", c);
    }

    println!("count after removing c = {}", Rc::strong_count(&a));

    println!("a is {:?}", a);
    println!("b is {:?}", b);
    println!("{}", c_message);
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
