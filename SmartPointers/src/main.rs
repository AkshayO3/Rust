use std::ops::Deref;    // Importing the deref trait

enum List{
    Cons(i32,Box<List>),         // Like a linked list
    Nil
}
use List::{Cons,Nil};

// Since the size of the List enum is not known at compile time,we need to use a pointer.
/* The Box<T> type is a smart pointer because it implements the Deref trait,which allows Box<T> values to be
   treated like references and have a fixed space in the stack,with a arbitrary value in the heap. */




struct MyBox<T>(T);
impl <T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {      // To implement the `deref` trait.
        &self.0     // Returning the reference to the first item of the tuple
    }
}



struct CustomSmartPointer{
    data:String
}
impl Drop for CustomSmartPointer{
    fn drop(&mut self) {            // Implementing the `drop` trait.although already included in standard library.
        println!("Dropping the customPointer with data {}",self.data);
    }
}

fn hello(x:&str){
    println!("Hello {}",x)
}

fn main() {
    let a = 5;
    let b = Box::new(a);    // The Box smart pointer,will get deallocated once out of scope
    println!("b = {}",b);
    let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));


    let y = MyBox::new(b.clone());
    assert_eq!(5,a);
    assert_eq!(5,*(b.deref()));




    // Implicit Deref Coercions --> Rust does deref coercion when it finds types and trait implementations in three cases:
    // 1. From &T to &U when T: Deref<Target=U>
    // 2. From &mut T to &mut U when T: DerefMut<Target=U>
    // 3. From &mut T to &U when T: Deref<Target=U>
    let m = MyBox::new(String::from("Rust"));
    hello(&m);      // Works because rust does &MyBox<String> -> &String -> &str automatically



    let c = CustomSmartPointer{
        data:String::from("Some data")
    };
    println!("Custom Pointer created");
    //c.drop();       // Will throw an error since explicit destructor calls are not allowed
    drop(c);    // We can use the standard function instead.
    println!("CustomPointer dropped before the end of main.")
}

