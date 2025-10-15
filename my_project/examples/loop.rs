use core::num;

fn main() {
    let mut i=0;
    let val = 5;
    let z:&str = loop {
        println!("loop {} x {} = {}",i,val,i*val);
        i+=1;
        if i>10 {break "Loop Ends Here";}
    };
    println!("Message: {z}");

    let arr = [1,2,3,4,5,6];
    let n:usize = arr.len();
    for i in 0..n {
        println!("Index: {i}, Element: {}",arr[i]);
    }

    let v = vec![10,20,30,40,50];
    for ele in v.iter() {
        println!("{ele}");
    }
    
    let nums:Vec<i32> = vec![1,2,3,4,5,6,7,8];
    let copi:Vec<i32> = nums.clone();
    println!("The Array: {:?},\n And its sum: {}",nums,sum(copi));

    let my_fill: Vec<u32> = fill(21, 25);
    println!("{:?}",my_fill);


}

pub fn sum(nums:Vec<i32>) -> i32 {
    let mut sum:i32 = 0;
    for i in nums.iter() {
     sum+=i;
    }
    sum
}
pub fn fill(i:u32,n:usize) -> Vec<u32> {
    let my_vec:Vec<u32> = vec![i;n];
    return my_vec;
}