fn main() {
    let items: Vec<u32> = vec![];
    let first_item = items.first();
    println!("{:?}", first_item.unwrap()); // 1
}
