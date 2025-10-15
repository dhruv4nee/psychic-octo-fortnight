#![allow(unused)]
use my_project::{my};

fn main() {
    my::print();
    my::my_nest::print();
    let s = my::my_nest::build(1);

}