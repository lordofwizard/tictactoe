mod game;
pub mod run;




fn main(){
    let game = run::Run::game_instance();
    game.run();
}