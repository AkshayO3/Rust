pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name:&str) -> String{
    format!("Hello {}",name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result,5);   // Will return true if the params are not equal
    }

    #[test]
    #[should_panic]     // Implies that the function should panic(will pass if panic)
    fn fail(){
        panic!("This test must fail.");
    }

    #[test]
    fn check_greeting(){
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain a name,it was only `{}`",result   //Custom error message
        );
        print!("This test has passed and this output proves it")
    }

    #[test]
    #[ignore]   //Ignores the test in normal cases,unless done specifically [ use cargo test -- --ignored]
    fn test_return() -> Result <(),String> {
        if 2+2==5{
            Ok(())
        }else{
            Err(String::from("This test has failed."))
        }
    }
}

// cargo test -- --test-threads=1 will run the tests in a serial order.
// cargo test -- --show-output will show the output even for the passed tests
// cargo test {name}  will test only that function or functions with that common name