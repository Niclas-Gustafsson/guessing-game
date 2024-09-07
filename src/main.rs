mod game;
use game::*;

fn main() {
    //Initiate game object and set answer
    let mut game = Game::new();
    Game::set_answer(&mut game);
    //Initiate game loop
    game.run();
}
