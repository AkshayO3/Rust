/*
Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can
create the closure in one place and then call the closure elsewhere to evaluate it in a different context. Unlike
functions, closures can capture values from the scope in which they’re defined.
 */

use std::thread;
use std::time::Duration;

struct Cacher<T>    //Caching the values so that we need to call the function only once
    where
    T: Fn(u32) -> u32,      // 3 fn traits are available
    {
        calculation:T,
        value:Option<u32>
    }

impl <T> Cacher <T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation:T) -> Cacher<T> {        //Constructor function
        Cacher{
            calculation,
            value:None
        }
    }
    fn value(&mut self,arg:u32) -> u32{
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


fn generate_workout(intensity:u32, random_number:u32){
    // let expensive_closure = |num:i32| -> i32{       // Defining a closure
    //     println!("Calculating slowly.");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut cached_result = Cacher::new(|num|->u32{    // Calculates only once since gets cached
        println!("Calculating slowly.");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25{
        println!("Today do {} pushups.",cached_result.value(intensity));
        println!("Then do {} situps.",cached_result.value(intensity));
    }
}

fn main(){
    generate_workout(24,4);
}