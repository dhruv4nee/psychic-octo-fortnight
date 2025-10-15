pub fn first(t:(bool,u32,char)) -> bool {
  t.0
}

pub fn last(t:(bool,u32,char)) -> char {
  t.2
}

pub fn swap(t:(u32,u32)) -> (u32,u32) {
    (t.1,t.0)
}
pub fn zeros() -> [u32;100] {
  [0;100]
}
pub fn first_3(s:&[u32]) -> &[u32] {
  &s[0..3]
}
pub fn last_3(s:&[u32]) -> &[u32] {
  &s[s.len()-3..]
}
pub fn hello() -> String {
  return String::from("Hello Rust");
}
pub fn greet(name: &str) -> String {
  return format!("Hello {name}");
}
pub fn append(mut s:String) -> String {
  s=s+"!";return s;
}
fn main() {
    todo!();
}
