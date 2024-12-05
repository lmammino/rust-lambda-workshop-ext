fn main() {
    let items: Vec<u32> = vec![];
    let first_item = items.first();
    println!("{:?}", first_item.unwrap_or(&1)); // 1
}
