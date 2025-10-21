#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut vals: Vec<i32> = vec![1, 2, 3, 4, 5];

    let vals2: Vec<i32> = vals.iter().map(|x: &i32| x + 1).collect();
    println!("vals:{:?}", vals);
    println!("vals2:{:?}", vals2);

    let vals: Vec<(&str, u32)> = vec![("abc", 1), ("def", 2), ("ghi", 3), ("jkl", 4)];
    let v: Vec<(String, u32)> = vals.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();
    println!("v: {:?}", v);
    let w: HashMap<String, u32> = vals.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();
    println!("w: {:?}", w);

    let ver:Vec<u32> = vec![1,2,3,4,5,6,7,8,9,10];
    let even:Vec<u32>=ver.iter().filter(|x| *x%2==0).map(|x| *x).collect();
    println!("even: {:?}",even);

    for v in vals.iter() {}

    //for v in vals.iter_mut() {

    // }

    //for v in vals.into_iter() {

    //}
}
