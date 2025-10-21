#![allow(unused)]

//Lifetime: Every Reference has a lifetime

fn longest_str<'a>(x:&'a str,y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

fn print_refs<'a,'b>(x:&'a str, y:&'b str) {
    println!("{x} {y}");
}

#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
}

impl<'a> Book<'a> {
    fn edit(&mut self,new_title: &'a str) {
        self.title = new_title;
    }
}

fn main() {
    let x = "Hello".to_string();
    let y = "Hello Rust".to_string();
    let z = longest_str(&x, &y);
    println!("{z}");
    print_refs(&x,&y);

    let s:&'static str = "Hello" ;

    let s:&'_ str = "Rust";
}