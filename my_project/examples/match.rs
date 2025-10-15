#![allow(unused)]

fn main(){
    let x =3;
    match x {
        i @ 1..4=> println!("1..3: {i}"),
        4 | 5 | 6=> println!("4..6"),
        7 | 8 | 9=> println!("7..9"),
        _=>println!("Others")
    }

    let x: Option<i32> = Some(9);
    let x: Option<i32> = None;
    match x {
        Some(val) => println!("Option is {val}"),
        None => println!("None")

    }

    let res: Result<i32,String>= Ok(234);
    let res: Result<i32,String>= Err("Gotcha!".to_string());
    match res {
        Ok(val)=>println!("ok {val}"),
        Err(err)=>println!("err:{err}")
    }
  println!("{}",num_to_string(2));
  println!("{}",num_to_string(100));
  println!("{}",unwrap_or_default(Some(90), 20));
  println!("{}",unwrap_or_default(None, 20));
   
   let x: Option<i32> = Some(22);
  if let Some(val) = x {
    println!("Option is {val}")
  }

}

pub fn num_to_string(num:u32) -> String {
    match num {
        1=>String::from("One"),
        2=>String::from("Two"),
        3=>String::from("Three"),
        _=>String::from("other")
    }
}

pub fn unwrap_or_default(x:Option<u32>,v:u32) ->u32 {
    // match x {
    //     Some(val)=>val,
    //     None=>v
    // }
    if let Some(val) = x {
        return val;
    }
    else {v}
}