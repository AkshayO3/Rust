pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


use OOPS::{Button, Draw, Screen};
struct SelectBox{
    width:u32,
    height:u32,
    options:Vec<String>
}
impl Draw for SelectBox{
    fn draw(&self) {
        todo!()
    }
}
fn main() {
    let screen = Screen{
        components: vec![
        Box::new(SelectBox{
            height:100,
            width:100,
            options:vec![String::from("Yes"),
                         String::from("No"),
                         String::from("Maybe")]
        }),
            Box::new(Button{
                height:100,
                width:100,
                label:String::from("OK");
            })
        ]
    }
}
/*
Static Dispatch:
Also known as early binding or compile-time dispatch.
The method to be executed is known at compile time.
In Rust, static dispatch is achieved using generics and monomorphization. Monomorphization is the process of turning
generic code into specific code by filling in the concrete types that are used when compiled.
Static dispatch allows the compiler to perform optimizations like inlining because it knows exactly which function
will be called at compile time.
However, it can lead to code bloat if used excessively, as a new copy of the function is created for each type it is
used with.

Dynamic Dispatch:
Also known as late binding or runtime dispatch.
The method to be executed is decided at runtime.
In Rust, dynamic dispatch is achieved using trait objects (Box<dyn Trait> or &dyn Trait).
Dynamic dispatch allows for more flexibility and can handle more complex cases where the exact type cannot be known
at compile time.
However, it can be slower than static dispatch due to the need for indirection (dereferencing a pointer to the vtable
to find the method to call) and it prevents certain compiler optimizations.
 */