// pub trait Iterator{
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
// The iterators use these traits, the next is mutable since it requires the next value to be fetched.

#[test]
fn iterator_sum(){              // Methods that consume the iterator
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total:i32 = v1_iter.sum();
    assert_eq!(total,6);
}

#[test]
fn produce_iterators(){         // Taking a closure and returning iterators
    let v1 = vec![1,2,3];
    let v2:Vec<_> = v1.iter().map(|x|x+1).collect();
    for x in v2{
        print!("{} ",x);
    }
}


// Using iterators with closures that capture their environment
#[derive(PartialEq,Debug)]
struct Shoe{
    size:u32,
    style:String
}
fn shoes_of_my_size(list:Vec<Shoe>,shoe_size:u32) -> Vec<Shoe>{
    list.into_iter().filter(|s|s.size==shoe_size).collect()    // Creates a iter that contains shoes of sizes
}

#[test]
fn filter(){                        // To see if the filter function works
    let shoes = vec![
       Shoe{size:10,style:String::from("Sneakers")},
         Shoe{size:13,style:String::from("Sandals")},
            Shoe{size:10,style:String::from("Boots")},
    ];
    let my_size = shoes_of_my_size(shoes,10);
    assert_eq!(my_size,vec![Shoe{size:10,style:String::from("Sneakers")},
                            Shoe{size:10,style:String::from("Boots")}])
}

// Creating our own iterators
struct Counter{
    count:u32
}

impl Counter{
    fn new() -> Counter{
        Counter{count:0}
    }
}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5{
            self.count +=1;
            Some(self.count)
        }else{
            None
        }
    }
}

#[test]
fn counter(){                               // Test to check if the custom iterator is Alright
    let mut counter = Counter::new();
    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);
}



fn main() {
    let v1 = vec![1,2,3];
    for value in v1.iter(){     //Iterator is used by the for loop
        print!("{} ",value);
    }
}

/*
Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s
capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators
 are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost
 abstractions.
 */