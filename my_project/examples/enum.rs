#![allow(unused)]
#[derive(Debug, PartialEq)]
enum Command {
    Play,Stop,Skip(u32),Back(u32),Resize{width:u32,height:u32}
}
pub enum Color {
    Red,
    Green, 
    Blue,
    Rgba(u8,u8,u8,f32)
}

fn main() {
    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(11);
    let cmd: Command = Command::Resize { width: 120, height: 70 };
    println!("{:?}",cmd);
    println!("{}",cmd0==cmd1);
    //Option<T> = Some<T> | None
    let x: Option<i32> = Some(12);
    let y: Option<i32> = None;
    // Result<T,E> = Ok(T) | Error(E)
    let alpha: Result<i32,String> = Ok(100);
    let beta: Result<i32,String> = Err("Failed to parse string into number".to_string());



}