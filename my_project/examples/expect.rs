#![allow(unused)]
fn main() {
    let x:Option<i32> = Some(3);
    let v:i32 = match x {
        Some(val)=>val,
        None=>panic!("no value")
    };

    let i=x.unwrap();
    println!("{i}");

    let x:Result<i32,String> = Ok(36);
    let v:i32 = match x {
        Ok(val)=>val,
        Err(err)=>panic!("{:?}",err)
    };

    let i=x.unwrap();
    println!("{i}");

    let x:Result<i32,String> = Ok(22);
    x.expect("Something Failed");

    parse_and_add("1","2" );
    parse_and_add("i","2" );



}
fn parse_and_add(a:&str,b:&str) -> u32 {
    let x:u32 = a.parse().expect("Failed to parse variable");
    let y:u32 = b.parse().expect("Failed to parse variable");
    x+y
}

fn unwrap_and_add(x:Option<u32>,y:Option<u32>) ->u32 {
    let a:u32 = x.unwrap();
    let b:u32 = y.unwrap();
    a+b


}