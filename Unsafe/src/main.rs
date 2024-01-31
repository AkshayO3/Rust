/*
Writing unsafe rust gives us 5 superpowers:
1. Dereference a raw pointer
2. Call an unsafe function or method
3. Access or modify a mutable static variable
4. Implement an unsafe trait
5. Access fields of unions
 */
use std::slice;
use std::ops::Add;
use std::fmt;
static HELLO_WORLD: &str = "Hello, world!";

extern "C" {
    fn abs(input: i32) -> i32;
}
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}


fn main() {
    //------------------------------------Raw Pointers------------------------------------------
    let mut num = 5;
    let r1 = &num as *const i32;        // Immutable raw pointer
    let r2 = &mut num as *mut i32;       // Mutable raw pointer
    unsafe {
        println!("r1 is {}",*r1);
        println!("r2 is {}",*r2);
    }
    //------------------------------------Calling an unsafe function or method------------------
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }
    //------------------------------------Creating a safe abstraction over unsafe code---------
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        assert!(mid <= len);
        unsafe {
            (slice::from_raw_parts_mut(ptr, mid),
             slice::from_raw_parts_mut(ptr.add(mid), len - mid))
        }
    }


    //------------------------------------Using extern functions to call external code---------
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    //------------------------------------Accessing or modifying a mutable static variable-----
    println!("name is: {}", HELLO_WORLD);

    //------------------------------------Implementing an unsafe trait-------------------------
    unsafe trait Foo {
        // methods go here
    }

    //------------------------------------Accessing fields of a union--------------------------
    union MyUnion {
        f1: u32,
        f2: f32,
    }




    // ------------------------------------Advanced traits-------------------------------------
    // Associated types
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }
    struct Counter {
        count: u32,
    }
    impl Iterator<u32> for Counter {
        fn next(&mut self) -> Option<u32> {
            Some(0)
        }
    }

    // Default generic type parameters and operator overloading
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // New-type pattern
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);


    // Type alias
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    // Never type
    fn bar() -> ! {
        panic!("This function never returns!");
    }

    // Dynamically sized types and the Sized trait
    fn generic<T>(t: T) {
        // --snip--
    }
    let s1 = "Hello, world!";
}
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl crate::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Add<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

// Calling methods with the same name
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// Super-traits
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}



// Newtype pattern to implement external traits on external types.
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}