use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //panic!("Crash and Burn");   //Immediately quits the program and prints the error message


    //
    // a();    // To get the specific sequence that leads to error,use RUST_BACKTRACE=1
    // fn a(){ b();}
    // fn b(){c(22)};
    // fn c(x:i32){
    //     if(x==22){
    //         panic!("The number is 22 and we are in danger.")
    //     }
    // }



    // The result enum
    enum Result<T,E>{
        Ok(T),
        Err(E)
    }

    let f = File::open("hello.txt");
    // The open functionality returns a result enum,which can be handled with match.
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(error) => panic!("Error creating the file --> {:?}",error)
            },
            other => panic!("Problem opening the file {:?}",other)
        }
    };

// Can be replaced with a method call,as in below(does the same thing but uses closures)
//
//     let f = f.unwrap_or_else(|error| match error.kind() {
//         ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc,
//             Err(error) => panic!("Error creating the file --> {:?}", error)
//         },
//         other => panic!("Problem opening the file {:?}", other)
//     });


    let f = File::open("hello.txt").unwrap();   // Handles the match,returns the result
    let f = File::open("hello.txt").expect("Error"); //Except can be used for custom messages
    hello();



    // Result enum should be used in error propagation,and the calling user may choose the appropriate way of handling.
    // Panic can be called in exceptional circumstances,when the program can absolutely not continue.

}

// The `?` operator can be used instead of unwrap or expect,it executes if successful,or simply returns the error.
fn hello() -> Result<(),Box<dyn Error>>{
    let f = File::open("dne.txt")?;
    Ok(())
}