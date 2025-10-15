#![allow(unused)]

mod lib;
fn main() {
    let msg: String = String::from("Hello Rust!");
    let msg: String = "Hello Rust".to_string();
    let length:usize = msg.len();
    let msg: String = String::from("Hello Rust");
    let s:&str = &msg[0..5];
    println!("s = {s}");

    let s: &str = "Hello World";
    let x: String = s.to_string();

printish(s);

let mut msg: String = String::from("Hello Uni");
msg+=" World";
println!("{msg}");

let lang = "rust";
let emj = "crabby";
//String Interpolation
let s = format!("Hello {lang} {emj}");
println!("{s}");
let a = lib::hello();
println!("{a}");
let b=lib::greet("Jack");
println!("{b}");
let mut joe = "Hello Joe".to_string();
let c = lib::append(joe);
println!("{c}");

}


fn printish(s: &str) {
    println!("{s}");
}