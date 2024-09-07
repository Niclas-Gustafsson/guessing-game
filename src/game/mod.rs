use std::{cmp::Ordering, io};
use std::process;
use colored::*;
use rand::*;

#[derive(Debug)]
enum GameStatus {
    IsGame,
    IsNotGame,
    IsNewGame
}
#[derive(Debug)]
pub struct Game {
    pub answer: u8,
    pub guess: u8,
    pub total_guesses: u8,
    pub guesses_made: u8,
    pub state: GameStatus
}

impl Game {
    pub fn new() -> Self {
        Self { answer: 0, guess: 0, total_guesses: 5, guesses_made: 0, state: GameStatus::IsNewGame }
    }

    pub fn set_answer(&mut self) {
        let answer: u8 = rand::thread_rng().gen_range(1..=20);
        self.answer = answer;
    }

    pub fn make_guess(&mut self)-> &str{
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("Failed to read guess.");
        let guess = guess.trim();

        guess
        // println!("Guess from make_guess {}", guess);
        // let parse_ok = self.parse_guess(&guess);

        // if parse_ok.is_err() {
        //     println!("Please insert a number and try again");
        //     self.make_guess();
        // }


       
    }

    fn parse_guess(&mut self, guess: &str) -> Result<(), &str>{
        if guess == "quit" {
            process::exit(0);
        }
        if let Ok(parsed_number) = guess.parse::<u8>() {
            self.guess = parsed_number;
            self.guesses_made += 1;
            Ok(())
     
        } else {
        //    println!("Please enter a number");
           Err("Couldn't parse number.")
        }
    }

    pub fn check_guess(&self) {
        match self.guess.cmp(&self.answer) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                  
            },
        }
    }

 
}