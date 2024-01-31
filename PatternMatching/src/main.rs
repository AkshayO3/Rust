// -------------------------------------------Match Expressions-------------------------------------
#[derive(Debug)]
enum Languages {
    English,
    Hindi,
    French,
    Spanish
}

struct Point{
    x:i32,
    y:i32
}


fn main(){
    let language = Languages::English;
    match language {
        Languages::English => println!("Hello"),
        Languages::Hindi => println!("Namaste"),
        Languages::French => println!("Bonjour"),
        lang => println!("Sorry,unsupported language.{:?}",lang)   // Takes into account all the other cases
    }


// --------------------------------------Conditional if-let statements-------------------------------
    let authorization_state : Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8,_> = "34".parse();
    if let Some(status) = authorization_state{
        println!("Authorization level --> {}",status);
    }
    else if is_admin{
        println!("Authorization level --> Admin");
    }
    else if let Ok(group_id) = group_id{
        if group_id > 30 {
            println!("Authorization level --> Privileged")
        }
        else if group_id < 30 {
            println!("Authorization level --> Basic");
        }
        else{
            println!("Authorization level --> Guest");
        }
    }



// -----------------------------------------While let conditional loops---------------------------------
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(s) = stack.pop(){
        println!("Element at top {}",s);
    }


// ------------------------------------------For loops-------------------------------------------------
    let v = vec!['a','b','c'];
    for (index,value) in v.iter().enumerate(){
        println!("{} at index {}",value,index);
    }


// -------------------------------------------let statements-------------------------------------------
    let x = 5;
    let (x,y,z) = (1,2,3);


// -----------------------------------------Function parameters------------------------------------------
    let point = (3,5);
    print_coordinates(&point);
    fn print_coordinates(&(x,y):&(i32,i32)){
        println!("Current location ({},{})",x,y);
    }
    /*
Refutable and irrefutable patterns are terms used in Rust programming language.

An irrefutable pattern is a pattern that will match for any possible value of the data type. For example, an
irrefutable pattern can be used to destructure a tuple because a tuple has a known, fixed size, so a pattern that
specifies a variable for each element of the tuple will always match.

A refutable pattern is a pattern that can fail to match for some possible value of the data type. For example, if you
have an `Option<T>` value, a pattern that tries to match against `Some(t)` is refutable because it will not match if
the value is `None`.

In Rust, `if let` and `while let` expressions are designed to handle refutable patterns. They perform pattern matching
and execute code based on whether the pattern matches or not. On the other hand, `let` statements and function
parameters can only accept irrefutable patterns. If you try to use a refutable pattern with them, you will get a
compile-time error.
 */




    match x{
        1 | 2 | 3 =>println!("Top three"),
        5..=10 => println!("Somewhere between 5 and 10"),
        // 'a' => println!("If x is char")
        _ => println!("Something else.")
    }

    // Patterns can also be used to destruct and take apart values.
    let p = Point{x:0,y:7};
    match p{
        Point{x,y:0} => println!("On the x-axis"),
        Point{x:0,y} => println!("On the y-axis."),
        Point{x,y} => println!("Not on either axis")
    }


    // Ignoring a value in a pattern
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value,new_setting_value) {
        (Some(_),Some(_)) => println!("Cannot overwrite an existing value."),   // _ ignores that parameter
        _ => setting_value = new_setting_value
    }


    // Match Guards
    let x = 4;
    match x{
        4 | 5 | 6  if y => println!("Yes"),     // The if condition is a match guard
        _ => println!("No")
    }

    //In Rust, the @ symbol is used in pattern matching to bind a name to a value in a pattern.
}

