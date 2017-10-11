fn main() {
    let rectangle = (50, 30);

    println!("Area of Rectangle with length {}cm and bredth {}cm is {}cm^2", rectangle.0, rectangle.1, area(rectangle) );

}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
