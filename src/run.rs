use crate::game::game::Board;
pub struct Run{
    board_instance : Board
}
impl Run{
    pub fn game_instance() -> Run {
        let board_instance = Board::new();
        Run { board_instance: board_instance }
    }
    pub fn run(&self){
        use std::io; // using it here... because i don't want to keep std functions 
                     // global scope
        
        let mut game_state : bool = true;
        Run::clear_screen();
        Run::welcome();
        while game_state == true{
            let mut input :String = String::new();
            println!("Command :- ");
            std::io::stdin()
            .read_line(&mut input).unwrap();
            //.expect("Failed to read the message");
            println!("{}",input);
            if input.trim() == "exit" {
                game_state = false;
            } 
        }

    }
    fn clear_screen(){
        print!("{esc}c", esc = 27 as char);
    }
    fn welcome(){
        use colored::Colorize;
        println!("{}","     Welcome to Tic-Tac-Toe".blue());
        println!("\n \n");
        println!("The rules are simple. This is a 2 player tictactoe game..\n One with a row/column of same element wins.\n For more documentation refer to \n https://github.com/lordofwizard/tictactoe.git");
        println!("{} {}","Project Author :-".yellow(),"LordOfWizard".red());

    }
    
}