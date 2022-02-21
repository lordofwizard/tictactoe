mod game;
pub mod run;
mod player;
pub mod check;

fn main(){
    let mut game = run::Run::game_instance();
    game.run();
}