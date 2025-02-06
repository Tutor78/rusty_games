use inquire::{CustomType, Confirm, Select};
use rand::{Rng};
use std::cmp::Ordering;
use crate::menus;

pub fn guessing_game() {
    loop {
        let difficulty_options = vec!["Easy", "Normal", "Hard"];

        let difficulty_select = Select::new("Choose your difficulty!", difficulty_options)
            .prompt()
            .unwrap()
            .to_string();

        let secret_number = difficulty(difficulty_select);

        loop {
            let player_guess = CustomType::<i32>::new("Try to guess: ")
                .with_error_message("Please pick a valid number!")
                .prompt();

            match player_guess.unwrap().cmp(&secret_number) {
                Ordering::Greater => println!("That number is too big!"),
                Ordering::Less => println!("That number is too small!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                },
            }
        }

        let play_again = Confirm::new("Play again?").prompt();

        if play_again.unwrap() == false {
            println!("Thanks for guessing!");
            break;
        }
    }

    menus::main_menu();
}

fn difficulty(difficulty: String) -> i32 {
    if difficulty == "Easy" {
        println!("Alright I am thinking of a number between 1 and 25");
        rand::rng().random_range(1..=25)
    } else if difficulty == "Normal" {
        println!("Alright I am thinking of a number between 1 and 50");
        rand::rng().random_range(1..=50)
    } else {
        println!("Alright I am thinking of a number between 1 and 100");
        rand::rng().random_range(1..=100)
    }
}