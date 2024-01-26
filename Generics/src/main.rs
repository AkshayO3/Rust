use std::fmt::Display;

fn main() {
    let f = vec![2, 3, 66, 33, 99];
    let c = vec![1.3, 6.7, 4.9, 7.8];
    println!("Results {} {}", get_largest(f), get_largest(c));
    let p1 = Point { x: 2, y: 4 };
    let p2 = Point { x: 3.3, y: 4.4 };
    println!("{} {}", p1.x, p2.y);


// -------------------------------- Traits -----------------------------------
    let tweet = Tweet {
        content: String::from("I am under the water"),
        author: String::from("Some southern guy"),
        reply: false,
        retweet: false
    };
    let article = Article {
        content: String::from("Man drowns under the water and asks for help."),
        author: String::from("Some local Journalist"),
        headline: String::from("UNDER WATER CIRCUS")
    };
    println!("{}\n{}\n{}\n{}", tweet.summerize(), tweet.sum_author(), article.summerize(), article.sum_author());
    press(&tweet, &article);
    let x = release(String::from("Hello"), String::from("My"), String::from("Cutie"));
    println!("{}", x.summerize());


// -------------------------------- LifeTimes -----------------------------------
    // Often used when references are taken into consideration.
    let result;
    let s1 = String::from("This is the first string in the comparison.");
    let s2 = String::from("This is the string it's being compared to.");
    result = longest(&s1, &s2);
    println!("{}",result);
}





// Generics in Rust are similar to templates in Cpp,used to reduce duplications.
// Important enums like option and result also implement generics.

fn get_largest <T: PartialOrd + Copy> (list:Vec<T>) -> T {  // The traits narrow types down to compatible ones.
    let mut largest = list[0];
    for num in list{
        if num > largest{
            largest = num;
        }
    }
    largest
}

// Generics can also be implemented on the point types
struct Point <T,U> {    // Both points can be of different types as well
    x:T,
    y:U
}
impl <T,U> Point<T,U>{  // Implementing a generic struct.
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}





// -------------------------------- Traits -----------------------------------

// A trait can be used to define shared behaviour between different types.
pub struct Article{
    pub author:String,
    pub headline:String,
    pub content:String
}

impl Summary for Article{
    fn summerize(&self) -> String {
        format!("{} : {}",self.headline,self.author)
    }
}

pub struct Tweet{
    pub author:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool
}

impl Summary for Tweet{
    fn summerize(&self) -> String {
        format!("{}, by {}",self.content,self.author)
    }
    fn sum_author(&self) -> String {
        format!("{} If it's a retweet? {}",self.author,self.retweet)
    }
}

pub trait Summary{
    fn summerize(&self) -> String;  // A function with no default implementation

    fn sum_author(&self) -> String{     // A default implementation,custom implementation will override this.
        String::from("Author Anonymous")
    }
}

fn press<T:Summary,U:Summary>(a:&T,b:&U){// The function has trait bounds,and can only accept parameters that follow the trait
    println!("{} {}",a.summerize(),b.sum_author());
}

fn release(c:String,d:String,e:String) -> impl Summary{ // Can only return types that implement the summary trait
    Article{
        content:c,
        headline:d,
        author:e
    }
}

// Implementations can be done with or without adding traits,and can function independently.
pub struct Pair<T>{
    x:T,
    y:T
}
impl <T> Pair<T>{
    // Function that can handle all data types
}
impl <T:PartialOrd+Display> Pair<T>{
    // Function that can take comparable values.
}





// -------------------------------- LifeTimes -----------------------------------

/*
1. Each param that is a reference gets its own lifetime parameter.
2. If there is exactly one input lifetime param,that lifetime is assigned to all output lifetime params.
3. If there are multiple input lifetime params,but one of them is &self or &mut self,the lifetime of self is
   assigned to all output lifetime params.
 */

// fn dangle(){
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("{}",&r);  // Borrow-Checker returns the error, as x does not live long enough to dereference.
// }

fn longest <'a> (x:&'a str,y:&'a str) -> &'a str {
    if x.len() > y.len() {          // A generic lifetime 'a is defined to draw a relationship b/w the res and params
        x
    }else{
        y
    }
}

struct Sen <'a>{        // Lifetimes with struct
    x:&'a str
}
impl <'a> Sen <'a>{
    fn x(&self) -> &'a str{
        println!("Lifetimes inside an implementation");
        self.x
    }
}

// Static Lifetimes are the longest possible lifetime,they live for the entire duration of the program.
fn staticlife(){
    let s: &'static str = "I have static lifetime.";
}