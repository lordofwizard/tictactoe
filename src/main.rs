mod game;
pub mod run;


use std::io;
use game::game::Board;


fn main(){
    println!("");
    let mut game = Board::new();
    game.print_board();
    println!();
    game.put_o(3, 3);
    println!();
    game.print_board();
    println!("filled ? {}",game.filled_check());
    game.fill_with_x();
    println!();
    game.print_board();
    println!("filled ? {}",game.filled_check());
}