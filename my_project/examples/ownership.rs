#! [allow(unused)]

//Ownership rules
// 1. Each value has an owner
// 2. There can only be one Owner at a time
// 3. When the owner goes out of scope the value will be dropped

use std::mem::take;

fn main(){
    //s and i are owners
    let s:String = String::from("rusty");
    let i:u32= 1;
    //owner is x
    let x = String::from("Doggerel");
    //ownership of the above string shifts to y
    let y:String = x;
    println!("{y}");
    {
        let z = y;
    }
   // y and z goes out of scope println!("{y}")
   //println!("{z}")
   


}