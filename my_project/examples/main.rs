mod lib;

fn main() {
    let t:(bool,u32,char) = (true,345,'#');
    println!("t: {:?}",t);
    println!("first:{}",lib::first(t));
    println!("last:{}",lib::last(t));
    let c:(u32,u32) = (23,45);
    println!("swapping {:?} into :{:?}",c,lib::swap(c));
    println!("\n\n###_Array Slices_###\n\n");
    let k:[u32;10]=[11,22,33,44,55,66,77,88,99,111];
    let h = &k[0..10];
    println!("h is {:?} \n",h);
    println!("Zeros function is: {:?}\n\n",lib::zeros());
    println!("First 3 of h is: {:?}\n\n",lib::first_3(h));
    println!("Last 3 of h is: {:?}\n\n",lib::last_3(h));
}