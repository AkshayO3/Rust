fn main() {
    let mut x:i32 = 5;  // mut keyword to signify mutable,but only in the same type
    x+=1;
    assert_eq!(x,6);        // The program compiles if the value is equal to 6
    println!("Success");
    const Y:i32 = 99;   // Immutable throughout,once initiated.
    println!("{}",Y);
    scope();
    data_types();
    tuples_arrays();
    functions();
    bitwise();
}
fn scope(){
    let x:i32 = 69;
    {
        let x:i32 = 55;
        assert_eq!(x,55);   // Will fail for any other value than 55,since the previous statement
    }                       // overshadows the value of x.
    println!("{}",x);       // The inner scope expires and the initial value of x gets printed.
}
fn data_types(){
    let x:i32 = 56;
    let y:f32 = 2.53;
    let z:bool = false;
    let a:char = 'A';
    let str:&str = "This is a string.";
    println!("{x}\t{y}\t{z}\t{a}\t{str}");
}
fn tuples_arrays(){
    let tup:(i32,f32,bool) = (500,22.4,true); // Defining a tuple
    let (a,b,c) = tup;      // Destructuring a tuple
    let x:i32 = tup.0;  // Accessing an element in a tuple
    println!("{x}");

    let arr = [1,2,3,4,5];
    let arr1:[i32;3] = [2,3,4];   // Syntax to initialize an array of type 32-bit float with 5 elements
    println!("{}",arr1[1]);
    let a = [3;5];  //Initializes an array with initial value 3 and 5 elements
}
fn functions(){
    parameters(69);
    println!("{}",return_type(3));
    statements_expressions();
    fn parameters(x:i32){   // Parameter is a 32-bit integer,and mandatory type declaration
        println!("Value of the input number -> {x}")
    }

    fn return_type(x:i32) -> i32{   // The function accepts and returns an integer.
        x + 1
    }

    fn statements_expressions(){
        let x = {let y:i32=68;y+1}; // The value of y gets bounded to x
        println!("{}",x);
    }
}
fn bitwise(){
    println!("0001 AND 0101 give {:04b}",0b0001u32 & 0b0101);
    println!("0001 OR 0101 give {:04b}",0b0001u32 | 0b0101);
    println!("0001 XOR 0101 give {:04b}",0b0001u32 ^ 0b0101);
    println!("1 << 5 is {}",1u32 << 5);
    println!("2 >> 6 is {}",2u32 >> 6);
}