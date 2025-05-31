use dotenv::dotenv;
use std::env;
fn main() {
    dotenv().ok();
    let test_var = env::var("ENV_TEST");

    match test_var {
        Ok(val) => println!("ENV_TEST: {:?}", val),
        Err(e) => println!("Error ENV_TEST: {}", e),
    }
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
