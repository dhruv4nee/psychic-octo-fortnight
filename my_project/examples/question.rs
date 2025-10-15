#![allow(unused)]

use core::num;

fn f1() -> Result<u32,String> {
    println!("f1");
    Ok(1)
}

fn f2() -> Result<u32,String> {
    println!("f2");
    Ok(2)
}

fn f1_f2_match() -> Result<u32,String> {
    let res1=f1();
    let out1= match res1 {
     Ok(num)=>num,
     Err(_)=> return Err("Error from f1".to_string())
    };
    let res2 = f2();
    let out2= match res2 {
     Ok(num)=>num,
     Err(_)=> return Err("Error from f2".to_string())
    };
    Ok(out1+out2)

}

fn f1_f2_question() -> Result<u32,String> {
    let out_1 = f1()?;
    let out_2=f2()?;
    Ok(out_1+out_2)

}
fn sum(nums:&[&str])->Result<u32,String> {
 let mut ans = 0u32;
 for s in nums {
    let val: u32 = s.parse::<u32>().map_err(|e| e.to_string())?;
        
    ans+=val;
 }
 Ok(ans)

}

fn main(){
    println!("{:?}",f1_f2_question());

    let arr = ["1","11","21","31","41","51","61","71","81"];
    println!("{:?}",sum(&arr));

}