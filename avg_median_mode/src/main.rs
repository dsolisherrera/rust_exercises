
use std::collections::HashMap;

fn average(numbers : &Vec<i32>) -> i32{
    let size = numbers.len() as i32;

    if size > 0{
        let avg = numbers.iter().fold(0, |acc, x| acc + x);
        return avg / size;
    }
    0
}

fn median(mut numbers : Vec<i32>) -> Option<i32>{
    if numbers.len() == 0{
        return None
    }

    numbers.sort();
    match numbers.len() % 2 {
        1=>{
            let median = numbers[numbers.len()/2];
            return Some(median);
        }
        0=>{
            let median = (numbers[numbers.len()/2] + numbers[numbers.len()/2 - 1]) / 2;
            return Some(median);
        }
        _=>{
            return None;
        }
    }
}

fn mode(numbers : &Vec<i32>) -> u32{
    let mut occurences : HashMap<i32, u32> = HashMap::new();
    for number in numbers{
        let count = occurences.entry(*number).or_insert(0);
        *count += 1;
    }

    let mode = occurences.iter().fold((0,0), 
        |acc, x| { 
            if *x.1 > acc.1{
                return (*x.0, *x.1)
            }
            acc
        });
    mode.0 as u32
}

fn main() {
    // let numbers = vec![1,2,3,4,5,12,89,12,23,6,989,1,2,64,23];

    // let numbers = vec![10,20,30];
    let numbers = vec![6,5,1,1,1,0];

    // let numbers : Vec<i32> = Vec::new();


    println!("Avg {}", average(&numbers));

    if let Some(x) = median(numbers.clone()){
        println!("Median {}", x);
    }
    else{
        println!("There is no median");
    }

    println!("Mode {}", mode(&numbers));
}
