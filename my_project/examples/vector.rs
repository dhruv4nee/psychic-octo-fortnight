#! [allow(unused)]

pub fn init(x:u32,y:u32,z:u32) -> Vec<u32> {
    return vec![x,y,z];
}

fn main(){
    let mut v: Vec<i32> = Vec::new();
    v.push(1);v.push(2);v.push(3);
    println!("v: {:#?}",v);

    let v: Vec<i8> = vec![11,22,33];
        println!("v: {:#?}",v);
    let v = vec![1u8,2,3];
        println!("v: {:#?}",v);
    let v: Vec<i8> = vec![0i8;101];
        println!("v: {:#?}",v);
    let i:usize = 20;
    println!("v[{}]: {:#?}",i,v.get(i));

    let mut v: Vec<i8> = vec![23,45,67];
    v[0]=22;

    let x: Option<i8> = v.pop();
    println!("pop: {:?}",x);
    let x: Option<i8> = v.pop();
    println!("pop: {:?}",x);
    let x: Option<i8> = v.pop();
    println!("pop: {:?}",x);
    let x: Option<i8> = v.pop();
    println!("pop: {:?}",x);

    let v = vec![10,20,30,40,50];
    let s = &v[1..3];
    println!("slice: {:?}",s);

    


}