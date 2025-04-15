use std::{thread, time::Duration};

//task 2
fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Total: {}", tracker);
    };

    update();
    update();
}


//task 3
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
}

//task 5
struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                let result = (self.computation)();
                self.value = Some(result.clone());
                result
            }
        }
    }
}

fn main() {

    //task 1
    let operation = |a: i32, b: i32| {
        a * b
    };

    println!("Result: {}", operation(10, 5));


    // task 2
    track_changes();


    // task 3
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        x * 2
    });

    let replaced = process_vector(numbers, |x| {
        if x > 2 { 0 } else { x }
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);


    // task 5
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());

    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}