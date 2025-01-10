#![allow(unused)]
fn main() {

    //String does not implement Copy for performance reason
    //let s1 = String::from("hello");
    let s1 = 1;
    let s2 = s1;
    println!("{s1}, world!");
    
    let ms1 = MyStruct {f1: 42, f2: 2};
    let ms2 = ms1; //make a copy
    let ms3 = ms1.clone(); //make a clone
    
    println!("\n {:?}", ms1);
    addr_of(&ms1);
    
    println!("\n {:?}", ms2);
    addr_of(&ms2);
    
    println!("\n {:?}", ms3);   
    addr_of(&ms3);
}

#[derive(Clone, Copy)]
#[derive(Debug)]
struct MyStruct {
  f1: u32,
  f2: u32
}

fn addr_of(x: &MyStruct) {
    println!(" >> addr {:p}", x);
}
