fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess > secret {
        1
    }
    else if guess < secret {
        -1
    }
    else {
        0
    }
}

fn main() {
    let secret: i32 = 69;
    let mut guess = 0;
    let mut counter = 0;

    loop {
        counter += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! The number was {}!", guess);
            break;
        } else if result == 1 {
            println!("Your guess, {}, is too high.", guess); 
        } else {
            println!("Your guess, {}, is too low.", guess);
        }

        guess += 23;
    }

    println!("It took you {} guesses.", counter);

    check_guess(guess, secret);

}