use std::error::Error;
use std::fs;

pub struct Config{
    pub query:String,
    pub file:String
}
impl Config{
    pub fn new(args: &[String]) -> Result<Config,&str> {
        if args.len()<3{
            return Err("Well the arguments weren't enough")
        }
        let query = args[1].clone();
        let file = args[2].clone();
        Ok(Config{query,file})
    }
}
pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let text = fs::read_to_string("hello.txt")?;
    for line in search(&config.query,&text) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query:&str,content:&'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn one_result(){            // Designing a test for the search function
        let query = "duct";
        let contents = "/
Well Rust is
Fast,Productive,Safe
What else would you want?";
        assert_eq!(vec!["Fast,Productive,Safe"],search(query,contents))
    }
}