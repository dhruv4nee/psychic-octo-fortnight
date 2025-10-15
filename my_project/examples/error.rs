#![allow(unused)]

fn main() {
    let v = vec![1,2,3];
#[derive(Debug)]
    enum MathError {
        DivByZero,
        Other
    }

    let x: Option<&i32> = v.get(1);

    match x {
        Some(val)=>println!("x: {:?}",val),
        None=> println!("x: None")
    }

    let x=1;let y=0;
    // panic: let q=x/y;
    let q: Result<i32,MathError> = if y!=0 {
        Ok(x/y)
    } else {
        Err(MathError::DivByZero)
    };

    match q {
        Ok(val) => println!("x/y = {:?}",val),
        Err(err)=>println!("x/y error {:?}",err)
    }

}