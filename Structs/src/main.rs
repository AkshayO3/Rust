#[derive(Debug)]
struct User{
    name:String,
    email:String,
    sign_in_count:u64,
    active:bool
}

impl User{      // `impl` keyword holds the methods and attributes of a struct
    fn welcome(&self) {
        println!("Welcome {}",self.name);
    }
}

impl User{
    fn welcome(user:String){        // An associative function doesn't take self as a parameter.
        println!("Hello from an associative function.,");
    }
}


fn main() {
    let mut User1 = User{
        name:String::from("Akshay"),
        email:String::from("abc@in.com"),
        sign_in_count: 4,
        active:true
    };
    let name = User1.name;
    User1.name = String::from("Bhandari");
    let User2 = build_user(String::from("Vector"),String::from("mail@zail.com"));
    println!("{} {} {} {}",User2.name,User2.email,User2.sign_in_count,User2.active);


    // A new user can be created by using some fields of an existing user.
    let User3 = User{
        name:String::from("Crispy"),
        ..User2     // Contains the same data other than the name
    };


    println!("{:#?}",User1);    //Printing the whole struct at once
    User1.welcome();
}

fn build_user(username:String,email:String) -> User{
    User{
        email:email,
        name:username,
        sign_in_count:1,
        active:true
    }
}