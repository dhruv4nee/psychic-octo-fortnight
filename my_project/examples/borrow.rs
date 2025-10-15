#![allow(unused)]

fn take(s: String) {
    println!("take {s}")
}

fn main() {
    let mut s = String::from("rust");

    let s1 = &s;
    let s2 = &s;
    let s3 = s2;
    
    println!("{}",*s3);
    
    let s0 = &mut s;
      //  let s21: &mut String = &mut s;


    s0.push_str("ty!");

    let s21: &mut String = &mut s;
    s21.push_str(" foobar");

    take(s);
}