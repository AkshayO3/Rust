fn main() {
    let mut x:i32 = 64;
    println!("{x}");
    let x:i32 = 99;
    const Y:u32 = 98475;    // A constant cannot be mutated once initialized,also they have to be type defined
    let z = 33;
    let z = 55;      // This statement shadows the value of the above z,without making it mutable.
    println!("{x} {Y} {z}");
    basic_control();
    loops();
}

// Control Flow
fn basic_control() {
    let num = 99;
    if(num<100) {
        print!("Alright it's short of a century.")
    }else if(num==100){
        print!("Well it's what was promised.")
    }else{
        print!("Jeez you over performed.")
    }

    let condition:bool = true;
    let num = if condition{5} else {6}; //The value assigned is 5 if the condition is true,otherwise false
}

fn loops() {
    loop{       // Will execute forever unless a break is applied
        print!("Forever");
        break;
    }

    let mut num = 4;
    while num!=0{               // Classic while loop
        print!("{num}\t");
        num-=1;     // 'num++' 'num--' are apparently not supported˳
    }
    println!("LIFTOFF!!");

    let arr = [4,5,6,7,8];
    for ele in arr.iter(){         // Iterative Loop
        print!("{ele}");
    }
    println!();

    for number in 10..15{        // Prints all the numbers in the range,the number on right must be bigger
        print!("{number}\t")
    }
}

