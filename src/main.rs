mod game;
pub mod run;


use std::io;



fn main(){
    let game = run::Run::game_instance();
    game.run();
}