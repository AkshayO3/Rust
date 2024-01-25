/*
Collections are stored in the heap rather than the stack.
Example of collections are vectors, strings, hash maps. They have dynamic sizes.
 */

use unicode_segmentation::UnicodeSegmentation;  // To print graphemes
use std::collections::HashMap;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);  // Pushing elements into a vector
    v.push(33);
    v.push(99);

    let v2 = vec![1, 2, 3];   // Creating a vector and initializing it at the same time
    //let third = &v[2];  //Accessing elements inside a vector,however an out of bounds can be thrown.

    // Better to use get method,would return a message if the value does not exist.
    match v.get(2) {
        Some(v) => println!("The value is {}", v),
        _ => println!("A third element does not exist.")
    }

    for i in &v {    // Accessing variables inside a vector
        println!("{i} ")
    }





    // In Rust, Strings are stored as a collection of UTF-8 encoded bytes.
    let s1 = String::new();
    let s2 = "Initial Commits";
    let s3 = String::from(s2);
    let s4 = s2.to_string();    // Different ways to create a string

    let mut s = String::from("John");
    s.push_str(" Cena!");
    let a = "You can't";
    let b = " see me.";
    //s = s + a + b;  // s takes ownership,a & b are no longer usable.
    let mut s1 = s + &a + &b;    // s doesn't take ownership
    let mut s2 = format!("{}{}",s1,s2);  //s still doesn't take ownership and looks more concise
    println!("{s1}");        // Different ways to add into a string.
    println!("{s2}");

    // To print a string,we cannot just take the bytes since a character can be 1-4 bytes long
    // 1. To get the bytes of the characters involved
    for b in "नमस्ते".bytes(){
        print!("{b} ")
    }
    println!();

    // 2. To get the scalar values
    for b in "नमस्ते".chars(){
        print!("{b} ")
    }

    println!();

    // 3. To get the grapheme clusters (needs unicode-segmentation to be imported)
    for b in "नमस्ते".graphemes(true){
        print!("{b} ")
    }




    // Hashmaps
    let red = String::from("Red");
    let blue = String::from("Blue");
    let mut hash = HashMap::new();
    hash.insert(red,10);
    hash.insert(blue,50);   // red,blue loose their values, can be prevented with lifetimes

    hash.insert(String::from("Green"),25);
    hash.insert(String::from("Green"),36);  // Overwrites the value

    // The entry method only writes with the value if the concerned key doesn't exist,the second statement won't work.
    hash.entry(String::from("Yellow")).or_insert(30);
    hash.entry(String::from("Yellow")).or_insert(50);


    let text = "Hello I am under the water, please help me, please";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;    // *count to dereference
    }
    println!("{:?}",map);
}