#![allow(unused)]
fn main() {
    let rect1 = Rectangle {
        width: 30, height: 50,
    };

    //println!("The rectangle's area is {:?}", area(rect1));
    println!("The rectangle's area is {:?}", area(&rect1));
    
    println!("The rectangle is {:?}", rect1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
//fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
