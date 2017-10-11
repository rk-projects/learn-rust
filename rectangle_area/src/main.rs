fn main() {
    let length = 50;
    let width = 30;

    println!("Area of Rectangle with length {}cm and bredth {}cm is {}cm^2", length, width, area(length, width) );


}

fn area(length: u32, width: u32) -> u32 {
    length * width

}