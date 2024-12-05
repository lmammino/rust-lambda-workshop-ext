use std::env;

fn main() {
    let region = env::var("AWS_REGION");

    match region {
        Ok(region) => println!("Selected region: {}", region),
        Err(_) => println!("Error: AWS_REGION not set"),
    }
}
