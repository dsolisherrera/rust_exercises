struct Counter {
    count: u32
}

impl Counter{
    fn new() -> Self{
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;


    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 5 {
            None
        }
        else {
            let x = self.count;
            self.count += 1;
            Some(x)
        }
        
    }
}

fn main() {
    let counter = Counter::new();

    let new_list : Vec<u32> = counter.zip(Counter::new().skip(1)).map(|(a,b)| a * b).collect();

    for item in new_list.iter() {
        println!("{}", item)
    }
}
