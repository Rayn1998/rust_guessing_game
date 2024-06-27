use std::{
    io, 
    cmp::Ordering, 
    process,
};
use rand::Rng;
use colorized::*;

fn main() {
    guessing_game();
}

fn guessing_game() {
    let mut attempts: u16 = 5;

    println!("{} \nType 'quit' anytime", "It's a Guessing game".color(Colors::BrightCyanBg));

    let random_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        if &attempts == &0 {
            println!("Sorry, you lost");
            end_game_question();
            break;
        }

        println!("You have {} attempts \nInput your guess between 1 and 100:", &attempts);
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("There's an error");

        if &guess.trim() == &"quit" {
            process::exit(0);
        }

        let guess = match guess.trim().parse::<u32>() {
            Ok(val) => val,
            Err(err) => {
                println!("There's an error: {}", err);
                continue;
            }
        };
        match guess.cmp(&random_number) {
            Ordering::Less => {
                println!("{}", "Too small".color(Colors::RedFg));
                attempts -= 1;
            }
            Ordering::Greater => {
                println!("{}", "Too big".color(Colors::RedFg));
                attempts -= 1;
            }
            Ordering::Equal => {
                println!("{}", "You win!".color(Colors::GreenFg));
                end_game_question();
                break;
            }
        }
    }
}

fn end_game_question() {
    let mut answer: String = String::new();

    println!("Do you wanna play again?   y / n");

    loop {
        answer.clear();
        io::stdin().read_line(&mut answer).expect("Answer the question");
    
        let answer: String  = answer.trim().to_lowercase();
        let answer: &str = answer.as_str();
    
        match answer {
            "y" => guessing_game(),
            "n" => process::exit(0),
            _ => {
                println!("Type 'y' or 'n', please");
                continue;
            }
        }
    }
}