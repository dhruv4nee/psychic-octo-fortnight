#![allow(unused)]

fn main() {
    let t:(bool,char,u32) = (true,'x',23);
    println!("{},{},{}",t.0,t.1,t.2);
    let nested = ((1,'a',true),(2.3,false));
    println!("nested .0.0:{}",(nested.0).0);
    let (a,b,_) = t;
    println!("a:{a},b:{b}");
}
