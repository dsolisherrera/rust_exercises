use std::thread;
use std::collections::HashMap;
use std::time::Duration;


struct Cacher<T, U, W>
    where T: Fn(&U) -> W,
    U: std::cmp::Eq + std::hash::Hash,
    W: std::clone::Clone
{
    calculation: T,
    values: HashMap<U,W>
}

impl <T, U, W> Cacher<T, U, W> 
    where T: Fn(&U) -> W,
        U: std::cmp::Eq + std::hash::Hash,
        W: std::clone::Clone
{
    fn new(calculation: T) -> Cacher<T, U, W> {
        Cacher {
            calculation,
            values: HashMap::new()
            }
    }

    fn value(&mut self, arg: U) -> W {
        if self.values.contains_key(&arg) {
            self.values.get(&arg).unwrap().clone()
        }
        else {        
            let v = (self.calculation)(&arg);
            self.values.insert(arg, v.clone());
            v
        }
    }
}

fn simulate_expensive_computation(intensity: &u32) -> u32 {
    println!("Starting slow computation");
    thread::sleep(Duration::from_secs(2));

    intensity.clone()
}

fn generate_workout(intensity: u32, variety_factor: u32) {

    let mut cacher = Cacher::new(simulate_expensive_computation);

    if intensity < 25 {
        println!(
            "Today do {} push-ups!",
            cacher.value(intensity)
        );
        println!(
            "Next do {} sit-ups",
            cacher.value(intensity)
        );
    }
    else {
        if variety_factor == 3 {
            println!("Take a break today, remember to stay hydrated!.");
        }
        else {
            println!(
                "Today run for {} minutes",
                cacher.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_requested_intensity = 20;
    let simulated_random_number = 3;

    generate_workout(simulated_requested_intensity, simulated_random_number);
}
