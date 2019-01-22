fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let mut six_point_four = x.1;

    six_point_four = 6.5;

    let one = x.2;

    println!("{} {} {}", five_hundred, six_point_four, one);
    println!("{} {} {}", x.0, x.1, x.2);
}
