#[derive(Debug)]
struct Rectangle {
    length: i32,
    width: i32,
}

fn main() {
    let rectangle = Rectangle{
        length: 50,
        width: 30,
    };

    println!("Area of Rectangle is {}", area(rectangle));
}

fn area(rectangle: Rectangle) -> i32 {
    println!("{:?}", rectangle);
    rectangle.length * rectangle.width
    
}