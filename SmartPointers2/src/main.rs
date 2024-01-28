enum List{                  // With Rc
    Cons(i32,Rc<List>),
    Nil
}

#[derive(Debug)]
enum List2{                 // With refCell and Rc
    ConsR(Rc<RefCell<i32>>,Rc<List2>),
    NilR
}


use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::List::{Cons, Nil};
use crate::List2::{ConsR,NilR};

struct Node {
    value: i32,
    child: Option<Rc<RefCell<Node>>>,
    parent: RefCell<Option<Weak<RefCell<Node>>>>,
}



fn main(){
    // let a = List::Cons(1, Box::new(Cons(2, Box::new(Nil))));
    // let b = List::Cons(3,Box::new(a));
    // let c = List::Cons(4,Box::new(a));


    /* Now this code won't work since the ownership of `a` is transferred when statement 2 is executed.
       To handle this,we could introduce lifetimes or even use the a.clone() method,however this will create deep
       copies of a and won't be very effective in the long run.
       So we implement the Reference Counting Pointers to share the data without transferring ownerships.
       The Rc::clone() method doesn't create a deep copy,but rather just increments the reference counts.
     */

    let a = Rc::new(Cons(1,Rc::new(Cons(2,Rc::new(Nil)))));
    println!("Counts after creating a {}",Rc::strong_count(&a));
    let b = Cons(3,Rc::clone(&a));
    println!("Counts after creating a {}",Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));  // Now they can share the value
        println!("Counts after creating a {}", Rc::strong_count(&a));        // Reference counts
    }
    println!("Counts after c goes out of scope {}",Rc::strong_count(&a));


    /*
    Refcell<T> is a type that gives us mutable access to the data even when there are immutable references to the data.
    This is checked at runtime and will panic if the rules are broken.
     */
    // Integrating the above code with RefCell<T>
    let value = Rc::new(RefCell::new(5));
    let v1 = Rc::new(ConsR(Rc::clone(&value),Rc::new(NilR)));
    let v2 = ConsR(Rc::new(RefCell::new(3)),Rc::clone(&v1));
    let v3 = ConsR(Rc::new(RefCell::new(4)),Rc::clone(&v1));
    *value.borrow_mut() += 10;
   println!("v1 after = {:?}",v1);
    println!("v2 after = {:?}",v2);
    println!("v3 after = {:?}",v3);





    // A simple reference cycle with strong_count() and weak_count() methods implemented.

     let node1 = Rc::new(RefCell::new(Node {
        value: 1,
        child: None,
        parent: RefCell::new(None),
    }));

    let node2 = Rc::new(RefCell::new(Node {
        value: 2,
        child: None,
        parent: RefCell::new(None),
    }));

    node1.borrow_mut().child = Some(node2.clone());
    node2.borrow_mut().parent = RefCell::new(Some(Rc::downgrade(&node1)));

/*
strong_count: This is the count of Rc pointers that point to the object. When strong_count goes to zero, the object is
              deallocated.
weak_count: This is the count of Weak pointers that point to the object. Weak pointers do not contribute to the
            strong_count. They do not prevent the object from being deallocated, but they can safely detect this
            event via the upgrade method.
 */
    println!("node1 strong count: {}, weak count: {}", Rc::strong_count(&node1), Rc::weak_count(&node1));
    println!("node2 strong count: {}, weak count: {}", Rc::strong_count(&node2), Rc::weak_count(&node2));
}