#![allow(unused)]
 
// enum Option<T> {
//     Some(T),
//     None
// }

// enum Result<T,E> {
//     Ok(T),
//     Err(E)
// }

struct Point<T> {
    x:T,
    y:T
}

fn swap<A,B>(a:A,b:B) -> (B,A) {
    (b,a)
}
fn first<A,B>(t:(A,B)) -> A{ t.0}
fn last<A,B>(t:(A,B)) -> B{ t.1}

struct Rectangle<T> {
    pub top:T,
    pub left:T,
    pub width:T,
    pub height:T,
}

fn main() {

    let v: Vec<i32> = vec![1i32,2,3];
    let p: Point<f32> = Point { x: 0.05, y: 0.01 };
    let a =1;
    let b = 0.5;
    let (x,y):(f64,i32)=swap(a, b);


}