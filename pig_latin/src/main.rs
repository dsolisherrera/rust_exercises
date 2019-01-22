use std::io;


fn get_user_input() -> String {
     let mut input = String::new();
    let read_bytes = match io::stdin().read_line(&mut input) {
        Ok(x) => x,
        Err(_) => {
            println!("Error reading input");
            0
        }
    };
    if read_bytes > 0 {
        input = input.trim().to_string();
    }

    input
}

fn word_to_pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let first = match text.chars().next() {
        Some(x) => x,
        None => panic!("Text should not be empty"),
    };

    if vowels.contains(&first) {
        let mut pig_word = String::from(text);
        pig_word.push_str("-hay");
        return pig_word;
    } else {
        let mut pig_word = String::from(&text[1..]) + "-";
        pig_word.push(first);
        pig_word.push_str("ay");
        return pig_word;
    }
}

fn text_to_pig_latin(text: &String) -> String {
    let words = text
        .split_whitespace()
        .fold(String::new(), |words, word| {
            let pig_word = word_to_pig_latin(&word);
            words + &pig_word + " "
        })
        .trim()
        .to_string();

    words
}

fn main() {
    println!("======Pig Latin Converter=====");
    println!("Introduce a word:");

    let input = get_user_input();

    if !input.is_empty() {
        let converted = text_to_pig_latin(&input);
        println!("Pig Latin of input: {}", converted);
    }
}
