#![allow(unused)]

use std::fmt::format;

# [derive(Debug)]
struct Point<T> {
    x:T,
    y:T
}

impl<T> Point<T> {

    fn new(x:T,y:T) -> Self{
        Self { x, y }
    }

    fn move_to(&mut self,x:T,y:T){
        self.x=x;
        self.y=y;

    }
}

struct Solidity {
    version:String
}
struct Vyper {
    version:String
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
    fn help(&self) -> String{
        "Good Luck".to_string()
    }
}

impl Compiler for Solidity {
    fn compile(&self,file_path: &str) -> String {
        format!("solc {}",file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self,file_path: &str) -> String {
        format!("vyper {}",file_path)
    }
}

fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)

}


fn main() {
    let mut p = Point{x:0.0,y:0.1};
    Point::move_to(&mut p,1.0,1.0);
    let a= Point::new(1, 2);
    let sol = Solidity{version:"0.8".to_string()};
    let vy = Vyper{version:"0.5".to_string()};
    println!("sol compile: {}",sol.compile("hello.sol"));
    println!("vy compile: {}",vy.compile("hello.vy"));

    println!("sol compile: {}",compile(&sol, "hello.sol"));
    println!("vy compile: {}",compile(&vy,"hello.vy"));

    println!("sol help: {}",Compiler::help(&sol));
    println!("vy help: {}",Compiler::help(&vy));

}
