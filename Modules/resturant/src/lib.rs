mod front_of_house;     //Using external files

// use std::io;
// use std::io::Write;  These two statements can be written more precisely as
use std::io::{self,Write};  //Imports both functionalities in a single function
use std::io::*;     // Imports all the public functionalities


use crate::front_of_house::hosting;
pub enum Cutlery{   //The enum variants are accessible if the enum itself is accessible [unique]
    Fork,
    Spoon
}

// A parent cannot access the functions in any of their children,but the child can.
// The snippet below wouldn't work unless both the required function and it's parent are turned public.
pub fn eat_at_resturant() {
    crate::front_of_house::hosting::add_to_waitlist();   //Absolute path `::` instead of `/`
    front_of_house::hosting::add_to_waitlist();  // Relative path
    let spoon = Cutlery::Spoon;



    // Rust has the `use` keyword to bring a path into the scope.
    pub use self::front_of_house::hosting;  //External code can now reference hosting as well
    hosting::add_to_waitlist(); //Start from the scope previously defined in the use
}

