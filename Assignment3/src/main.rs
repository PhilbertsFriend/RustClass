fn check_guess(guess: i32, secret: i32) -> i32{
    if guess < secret {
    return -1;
    }
    
    else if guess == secret {
    return 0;
    }

    else if guess > secret {
    return 1;
    }

    else {
        println!("Something went wrong.");
        return 2;
    }
}

use rand::Rng;

fn main() {
    let mut secret = 25;
    let mut guessamt = 0;

    loop {
        let guess = rand::thread_rng().gen_range(0..100); // Random num between 0 and 99
        guessamt += 1;

        if check_guess(guess, secret) == 0 {
            println!("The guess {} is correct.", guess);
            break;
        }

        else if check_guess(guess, secret) == -1 {
            println!("The guess {} is too low.", guess);
        }

        else if check_guess(guess, secret) == 1 {
            println!("The guess {} is too high.", guess);
        }

    }
    println!("There was a total of {} guesses.", guessamt);
}