fn longuest<'a, 'b>(left: &'a str, right: &'b str) -> &'b str
{
    if left.len() > right.len(){
        left
    }
    else {
        right
    }
}


fn main() {
    let string1 = String::from("This string is longuest");
    let long;
    {
        let string2 =  String::from("Shorter");

        long = longuest(string1.as_str(), string2.as_str());
    }
    println!("Longuest string is {}", long);
}
