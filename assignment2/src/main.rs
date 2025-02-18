fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // numbers
    let numbers: [i32; 10] = [42, 17, 93, 5, 68, 24, 11, 77, 30, 89];

    // fizz buzz
    for &n in numbers.iter() {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        }
        else if n % 3 == 0 {
            println!("Fizz");
        }
        else if n % 5 == 0 {
            println!("Buzz");
        }
        else {
            if is_even(n) {
                println!("{} is even", n);
            }
            else {
                println!("{} is odd", n);
            }
        }
    }

    // sum of all numbers
    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("The sum of all numbers is: {}", sum);

    // find largest number
    let mut largest = 0;

    for &n in numbers.iter() {
        if largest < n {
            largest = n;
        }
        else {

        }
    }

    println!("Largest number in array: {}", largest);
}