
struct Point<T> {
    x: T,
    y: T,
}

fn get_max<T>(list : &[T]) -> &T 
    where T: PartialOrd
{
    let mut largest = &list[0];

    for number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}


pub trait Summary{
    fn summarize(&self) -> String;
}

struct Newsletter{
    header: String,
    body: String,
    footer: String
}

impl Newsletter {
    fn new(header: String, body: String, footer: String) -> Self {
        Self {
            header,
            body,
            footer
        }
    }
}

impl Summary for Newsletter {
    fn summarize(&self) -> String {
        format!("{} {} {}", self.header, self.body, self.footer)
    }
}

struct Tweet {
    body: String,
    metadata: String
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", self.body, self.metadata)
    }
}

fn notify(item: impl Summary) -> String {
    format!("Breaking News!!!: {}", item.summarize())
}

fn main() {

    let point = Point {x: 1, y: 2};
    println!("Point {} {}", point.x, point.y);


    let tweet = Tweet {body: String::from("Tweet body"), metadata: String::from("Tweet metadata")};
    let news =  Newsletter::new(String::from("News Header"), String::from("News Body"), String::from("News footer"));

    println!("Tweet: {}", tweet.summarize());
    println!("news: {}", news.summarize());
    println!("+++++++++++: {}", notify(news));



    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_max(&number_list);

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let largest = get_max(&number_list);

    println!("The largest number is {}", largest);
}
