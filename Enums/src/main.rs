use std::num::FpCategory::Nan;

// An enum is a type that can have a fixed set of values, and those values are called the enum’s variants.
enum IpAddrKing{
    V4(u8,u8,u8,u8),    // enum variant,takes four u8 values
    V6(String)          // enum variant,takes one String value
}

struct IpAddr{
    kind: IpAddrKing,
    name: String
}


fn main(){
    let localhost = IpAddrKing::V4(127,0,0,1);
    let localhost6 = IpAddrKing::V6(String::from("4ef65:493fg:4f"));


// Rust has no NULL values,and contains the option ENUM. If a value can be NULL,it's wrapped around the option ENUM
    let x = Some(45);   // Some signifies that it has a value,but can become NULL,explicit type specification not req
    let y = Some("Zulu");
    let z:Option<i32>= None;    // Since the value already is NULL,the type specification is required.

    let xx = x.unwrap_or(0);    // Unwraps the value to operate on,reverts to default if NULL.
    let yy = y.unwrap_or(" ");
    let zz = z.unwrap_or(3);
    println!("{xx} {yy} {zz}");


    let some_value = Some(3);
    match some_value{           // Takes the value to be matched
        Some(3) => println!("{}",some_value.unwrap_or(0)+1),   // Specific types can be checked
        _ => println!("Something else.")    // `_` covers all the remaining types
    }

    // The match conditions have to cover all the possible conditions, `if-let` keyword can be used to ignore all the
    // redundant conditions.
    if let Some(3) = some_value{    // Same condition executed under if let.
        println!("{}",some_value.unwrap_or(0)+1);
    }
}
