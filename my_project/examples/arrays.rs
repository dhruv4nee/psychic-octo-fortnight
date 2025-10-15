#![allow(unused)]

fn main() {
    let mut arr: [u32;4] = [2,3,4,5];
    println!("arr[0]: {}",arr[0]);
    arr[2]=999;
    println!("arr[2]: {}",arr[2]);

    let arr:[u32;10] = [11;10];
    println!("arr: {:?}",arr);

    let mut nums:[i32;11]=[1,2,3,4,5,6,7,8,-9,-10,-100];
    println!("nums: {:?}",nums);

    let  s: &mut [i32] = &mut nums[0..4];
    println!("nums slice[0..4]: {:?}",s);

    s[0]=23;
    println!("nums slice[0..4]: {:?}",s);



    
}