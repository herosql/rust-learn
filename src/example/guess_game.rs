use rand::Rng;
use std::{cmp::Ordering, io};

fn guess_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut count = 0;
    while count < 5 {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    // 引入外部函数
    use super::*;

    // 单元测试
    #[test]
    fn test_guess() {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is: {secret_number}");
        let mut count = 0;
        while count < 5 {
            let guess = rand::thread_rng().gen_range(1..=100);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
            count += 1;
        }

    }
}
