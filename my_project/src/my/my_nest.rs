use crate::foo;

pub fn print() {
    foo::print();

    println!("rusty");
}

pub struct S {
    name: String,
    pub id: u32,
}
pub fn build(id: u32) -> S {
    S {
        id,
        name: "test".to_string(),
    }
}
