#![allow(unused)]

fn main() {
    let mut x = u32::MAX;
    x+=1;
    println!("x:{x}");
    let x = u32::checked_add(3,1);
    println!("checked_add:{:?}",x);
    println!("checked_add:{:?}",u32::checked_add(u32::MAX,1));
}