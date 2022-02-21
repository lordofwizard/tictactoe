mod game;
pub mod run;
mod player;


fn main(){
    let mut game = run::Run::game_instance();
    game.run();
}