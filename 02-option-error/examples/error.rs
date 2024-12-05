use std::env;

fn main() {
    let region = env::var("AWS_REGION");
    println!("{:?}", region);
}
