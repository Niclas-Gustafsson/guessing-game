use std::{cmp::Ordering, io};
use std::process;
use colored::*;
use rand::*;

#[derive(PartialEq, Debug)]
pub enum GameStatus {
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
        self.update_game_state(GameStatus::IsGame);
    }

    fn update_game_state(&mut self, state: GameStatus) {
        self.state = state;
    }
    //Reset current game object.
    pub fn new_game(&mut self) {
        self.answer = 0;
        self.guess = 0;
        self.total_guesses = 5;
        self.guesses_made = 0;
        self.state = GameStatus::IsNewGame
    }

    pub fn run(&mut self) {
        while self.state == GameStatus::IsNewGame || self.state == GameStatus::IsGame {

            if self.state == GameStatus::IsNewGame {
                //if new game reset stats and set new answer?
                self.new_game();
                self.set_answer();
            } else if self.state == GameStatus::IsGame {
                println!("Guesses left: {}.\nType 'quit' to exit.\nEnter a number between 1-20:", self.total_guesses - self.guesses_made);
                self.make_guess();
  
                self.check_guess();
    
                if self.guesses_made == self.total_guesses {
                    self.state = GameStatus::IsNotGame;
                }
            } 
        } 

        if self.state == GameStatus::IsNotGame {
            self.ask_for_new_game();
    
            self.run();
        }
    }


    //Takes user input and tries parsing to number.
    pub fn make_guess(&mut self){
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("Failed to read guess.");
        let guess = guess.trim();

        // guess
        // println!("Guess from make_guess {}", guess);
        let parse_ok = self.parse_guess(&guess);

        if parse_ok.is_err() {
            println!("Please insert a number and try again:");
            self.make_guess();
        }
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
    pub fn ask_for_new_game(&mut self) {
        let mut answer = String::new();
        println!("Do you want to play again? Type y/yes, n/no");
        io::stdin().read_line(&mut answer).expect("Failed to read guess.");
        let answer = answer.trim().to_lowercase();

        if answer == "y" || answer == "yes" {
            //start new game
            // Self::update_game_state(self, GameStatus::IsNotGame);
            Self::update_game_state(self, GameStatus::IsNewGame);
        } else if answer == "n" || answer == "no" {
            process::exit(0);
        }
    }


    pub fn check_guess(&mut self) {
        match self.guess.cmp(&self.answer) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                Self::update_game_state(self, GameStatus::IsNotGame);   
            },
        }
    }
}