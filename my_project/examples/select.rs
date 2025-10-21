#![allow(unused)]

use std::time::Duration;
use tokio::{join,select};

//join!
//Polls multiple futures concurrently
//Waits for all of them to complete
//Returns a tuple of their results

//select!
//Polls multiple futures concurrently
//Returns as soon as one of them completes
//Cancels the rest(drops them)

async fn make(val:&'static str, dt:u64) -> &'static str {
    tokio::time::sleep(Duration::from_millis(dt)).await;
    val
}


#[tokio::main]
async fn main() {

    let (res1,res2,res3)=join!(
        make("coffee", 100),
        make("tea", 200),
        make("mocha",159)
    );
    println!("res1:{res1}\nres2:{res2}\nres3:{res3}");

    let res = select! {
        val = make("hamburger",101)=>{
            println!("hamburger ready!");val
        },
        val = make("coffee", 100)=>{
            println!("coffee ready!");val
        },
        val = make("tea", 200)=>{
            println!("tea ready!");val
        },
        val = make("mocha",159)=>{
            println!("mocha ready!");val
        }

    };
    println!("select:res = {:?}",res);

}