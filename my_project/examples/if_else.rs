#![allow(unused)]

pub fn min(x:i32,y:i32) -> i32 {
    if x<y {
        x
    }
    else {
        y
    }
}

pub fn max(x:i32,y:i32) -> i32 {
    if x>y {
        x
    }
    else {
        y
    }
}

pub fn sign(x:i32) -> i32 {
    if x>0 {
        1
    }
    else if x<0{
        -1
    }
    else {
        0
    }
}
fn main(){
    let x:i32 =11;

    let z:i32 = if x>0 {
        println!("x>0");
        1
    } else if x<0 {
        println!("x<0");
        -1
    } else {println!("x=0");0};

    println!("z = {z}");

}