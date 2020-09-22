use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
mod drop_trait;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


struct MyBox<T>(T); // struct called MyBox of type tuple containing genertic type T, not known at compile time.
 
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { // method new which returns type MyBox, while accepting generic type T as param, 
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> { // in order to dereference this struct we must use Deref from std library. 
    type Target = T;

    fn deref(&self) -> &T { // return self and Rust handles deref coercion. 
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello: {}", name);
}

fn main() {
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
     println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    

    let x = 5;
    let y = MyBox::new(x); // because y is equal to the struct type MYBox and is of type genertic T, and is referencing the value of x...

    assert_eq!(5, x);
    assert_eq!(5, *y); // we must reference y as a pointer with *, so as to capture the value of the address y is pointing to.

    let m = MyBox::new(String::from("Rust")); // again, m will be the string value of MyBox stored in the tuple as &String which can be DeRef as &str
    hello(&m); // and called here as &m

    let c = drop_trait::CustomSmartPointer { data: String::from("my stuff"), }; // c will drop second
    let d = drop_trait::CustomSmartPointer { data: String::from("other stuff"), }; // d will drop first
    println!("CustomSmartPointer created.");
}
