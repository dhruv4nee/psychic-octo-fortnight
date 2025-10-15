#![allow(unused)]

use std::{i32, u32};

fn main() {
    let i:i8 = -1;
    let i2 = 23;
    let b:bool= true;
    let c:char = 'a';
    let u:u32 = i as u32;
    let i_max = i32::MAX;
    let u_max = u32::MAX;
    println!("{i} as u32 = {u}");
    println!("i_32 max is: {i_max}, And u_32 max is: {u_max}");

}