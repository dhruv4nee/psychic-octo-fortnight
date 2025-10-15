pub fn eq(a:char,b:char) -> bool {
    return a==b;
}

pub fn add(a:f32,b:f32,c:f32) -> f32 {
    return a+b+c;
}
pub fn cast(x:u8,y:i8,z:f32) -> f32{
    let f1:f32 = x as f32;
    let f2:f32 = y as f32;
    let f3:f32 = z as f32;
    return f1+f2+f3;
}