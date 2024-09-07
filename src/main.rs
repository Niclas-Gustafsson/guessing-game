mod game;
use game::*;

fn main() {
let mut game = game::Game::new();
Game::set_answer(&mut game);
println!("Game object: {:?}", game);
println!("Answer: {:?}", game.answer);
    println!("-----------------GUESS THE NUMBER------------------");
    println!("Make a guess between 1-20 in number. E.g 4, 14, 1");
    Game::make_guess(&mut game);
    // println!("Game object: {:?}", game);

    //Parse guess and make comparison
}
