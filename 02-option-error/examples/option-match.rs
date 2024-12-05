fn main() {
    let items: Vec<u32> = vec![1, 2, 3];
    let first_item = items.first();
    match first_item {
        Some(value) => println!("{}", value),
        None => println!("No items"),
    }
}
