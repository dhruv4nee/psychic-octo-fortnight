#![allow(unused)]

use std::collections::HashMap;
pub fn init(address: String,amount:u32) -> HashMap<String,u32> {
    let mut hashing: HashMap<String,u32> = HashMap::new();
    hashing.insert(address, amount);
    return hashing;
}
fn main(){
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("red".to_string(),300 );
    scores.insert("indigo".to_string(),250 );
    println!("scores: {:#?}",scores);

    let score: Option<&u32> = scores.get("red");
    println!("Red Score: {:#?}",score);
    let score: Option<&u32> = scores.get("blue");
    println!("Blue Score: {:#?}",score);

    let score: &mut u32 = scores.entry("black".to_string()).or_insert(0);
    *score+=101;

    let score: Option<&u32> = scores.get("black");
    println!("Black Score: {:#?}",score)

}

