mod utils;
fn main() {
    println!("Hello, world!");
    println!("Eq is for a and b,{}",utils::eq('a','b'));
    println!("add is for 1.2 and 3.4 and 5.6,{}",utils::add(1.2,3.4,5.6));
    println!("cast is for 12 ,-9 and 3.9,{}",utils::cast(12,-9,3.9));



}
