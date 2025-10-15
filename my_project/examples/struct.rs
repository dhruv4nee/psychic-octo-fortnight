#![allow(unused)]
#[derive(Debug)]
struct Point2D {
    x:i32,
    y:i32
}
#[derive(Debug)]

struct Point3D(i32,i32,i32);
#[derive(Debug)]

struct Empty;
#[derive(Debug)]

struct Circle {
    radius: u32,
    center: Point2D

}

#[derive(Debug)]
pub struct Account{
   pub address: String,
   pub balance: u32
}
pub fn new(address: String) -> Account {
    return Account{address,balance:0};
}
fn main(){
let p = Point2D{x:0,y:0};
println!("{:?}",p);
println!("x: {}, y:{}",p.x,p.y);

let p=Point3D(11,11,11);
println!("{:?}",p);
println!("x: {}, y:{}, z:{}",p.0,p.1,p.2);
let empty = Empty;
let circle = Circle {
    radius: 21,
    center: Point2D{x:10,y:11}
};

println!("Empty {:?}",empty);
println!("Circle: {:#?}",circle);
println!("radius: {}, \n center: {:#?}",circle.radius,circle.center);

let my_account:Account = new("Clojk".to_string());

println!("{:#?}",my_account);


}