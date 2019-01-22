#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}


impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let x = std::f32::consts::PI;
    println!("Area is {}" , x * 8.2);
    let rect = Rectangle{width: 300, height: 20};
    let rect1 = Rectangle{width: 30, height: 2};

    println!("The rectangle is: {:?}", rect);

    println!(
        "The area of the rectangle is={}",
        rect.area()
    );

    println!("Rect can hold Rect1?: {}", rect.can_hold(&rect1));
}


