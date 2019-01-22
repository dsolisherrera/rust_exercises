use std::collections::HashMap;

fn main() {
    let mut x = HashMap::new();

    x.insert("Hi", 12);
    x.insert("ko", 15);

    println!("ko {}", x["ko"]);

    if let Some(value) = x.get_mut("ko"){
        *value = 25;
    }
    println!("ko {}", x["ko"]);
}
