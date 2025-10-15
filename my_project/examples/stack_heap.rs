#![allow(unused)]

use std::boxed;

fn main() {
    //Stack
    let x:i32 = 1;
    let arr:[i32;10] = [11;10];

    //Heap
    let mut s:String = "Hello".to_string();
    s+=" World!";
    let mut v = vec![];
    v.push(0);v.push(1);v.push(2);
    //Forcing to store in heap
    let boxed = Box::new(1u32);

}