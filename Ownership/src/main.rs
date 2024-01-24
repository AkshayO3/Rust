/* --------------------------OWNERSHIP RULES---------------------------
    1. Each value in Rust has a variable that is called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope,the value is dropped.
 */


/* The ownership concept in rust is used to manage memory in a way that the compiler can check at compile time
   whether all the memory is being used or not.
   Rust uses a stack and a heap to manage memory.
   Rust never automatically creates deep copies of data.
   Rust uses a concept called borrowing to allow multiple variables to access the same data without taking ownership.
   The scope is the range within a program for which an item is valid.
   A variable is valid from the point at which it is declared until the end of the current scope.
   The String type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
   The memory must be requested from the operating system at runtime.
   The memory is automatically returned once the variable that owns it goes out of scope.
   When a variable goes out of scope,Rust calls a special function for us.This function is called drop,and
    it’s where the author of String can put the code to return the memory.
   Therefore,any automatic copying can be assumed to be inexpensive in terms of runtime performance.
   To deeply copy the heap data of the String,we can use a common method called clone.
   The clone method will make a copy of the string data,allocating on the heap.
   The clone method is often unexpectedly expensive.
   To change the value that a variable is pointing to,we have to use the dereference operator (*).
   The dereference operator allows us to access the data at the reference’s location.
   The opposite of referencing by using & is dereferencing,which is accomplished with the dereference operator, *.
*/



fn main() {
    {
        let s = "hello";  // An immutable string that is valid inside the scope.
        let s1:String = String::from("dynamic hello");  // A dynamic string.
        println!("{s} {s1}");
    }
    // Rust drops the value as soon as the owner(s) goes out of scope.


    let x = 5;
    let y = x;  // Simple Copy
    println!("{x} {y}");



    let s1 = String::from("Hello");
    let s2 = s1;    // Move(not shallow copy)
    let s3 = s2.clone(); // The value is now cloned,taking up more resources
    //println!("{s1} {s2}");      // Would return an error saying because s1's value has been moved



    // Simple data types like int,double,boolean etc. get copied and hence can be accessed after being passed into fn.
    let z = String::from("Bonjour");
    takes_memory(z);
    //println!("{z}")         //  This statement would also return an error,as the ownership of z is taken by the fn.



    // To solve the problem created by ownership transfer,references to variables are used.
    let r = String::from("This would be accessible");
    takes_reference(&r);
    println!("{r}");        // Accessible since no ownership transfer takes place while using references



    // References are immutable by value,unless a mutable variable is used in collaboration with a mutable reference.
    let mut e = String::from("This will be mutated without ownership transfer.");
    println!("{e}");
    let e1 = &mut e;
    takes_mutable_refernce(e1);

    /* The basic rules:
       --> Any number of immutable references can be created and used for a variable.
       --> Only one mutable reference can be created and used,another would throw an error.
       --> Mutable and immutable references cannot be used in collaboration.
     */


    // let reference_to_nothing = dangle();
    // println!("{reference_to_nothing}")



    // Slice returns a specific part of the string or an array
    let q = String::from("Hola Senorita");
    // Like references,only followed by [start .. end]
    let first_word = &q[0..4];  // Can be written as [..4]
    let second_word = &q[5..13]; // Can be written as [5..]
    println!("{first_word} {second_word}")
}
fn takes_memory(s:String){
    println!("{s}");
}

fn takes_reference(s:&String){
    println!("{s}");
}

fn takes_mutable_refernce(s: &mut String){
    s.push_str("This is added without taking ownership.");
    println!("{s}");
}

// fn dangle() -> &String{ // Returns an error right here that dangling reference cannot be entertained.
//     let s = String::from("Well");
//     &s
// }