use std::borrow::Borrow;

use crate::game::game::Board;
use crate::player::player::Type;
pub struct Run{
    pub board_instance : Board
}
#[allow(dead_code)]
macro_rules! input {
    ($($var:ident)*) => {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut words = buf.split_whitespace();
        $(
            $var = words.next().unwrap().parse().unwrap();
        )*
    }
}
impl Run{
    
    pub fn game_instance() -> Run {
        let board_instance = Board::new();
        Run { board_instance: board_instance }
    }
    pub fn run(&mut self){
        //use std::io; // using it here... because i don't want to keep std functions 
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
            if input.trim() == "start" {
                Run::game(self);    
            }
            else{
                println!("Valid commands expected");
            }
        }
    }
    fn clear_screen(){
        print!("{esc}c", esc = 27 as char);
    }
    fn welcome(){
        // Basic run screen
        use colored::Colorize;
        println!("{}","     Welcome to Tic-Tac-Toe".blue());
        println!("\n \n");
        println!("The rules are simple. This is a 2 player tictactoe game..\n One with a row/column of same element wins.\n For more documentation refer to \n https://github.com/lordofwizard/tictactoe.git");
        println!("{} {}","Project Author :-".yellow(),"LordOfWizard".red());
    }

    #[allow(dead_code)]
    fn help(){
        Run::clear_screen();
        println!("Commands to play this game");
        println!("start - Starts the game.. obi");
        println!("exit - Stops the game and closes it");
    }

    fn game(&mut self){
        use crate::player::player::Player;
        let (p1_name,p2_name) = Run::players_names();
        let mut Player1 :Player =  Player{
            name:p1_name,
            char_type:Type::TypeX,
            no_of_win : 0
        };
        let mut Player1 :Player =  Player{
            name:p2_name,
            char_type:Type::TypeX,
            no_of_win : 0
        };

        match self.win_check() {
            Type::TypeX => println!("X wins"),
            Type::TypeO => println!("O wins"),
            _ => println!("Draw")
        }
        
    }
    fn players_names() -> (String,String){
        Run::clear_screen();
        println!("Welcome you are in the game now");
        println!("Input Player1 and Player2 Names in space separated values");
        let  n1 : String;
        let  n2 : String;
        input!(n1 n2);
        (n1, n2)
    }
    
    fn is_full(&mut self) -> bool{
        let mut count = 0;
        for i in 0..3 {
            for j in 0..3{
                if (&self.board_instance.board[i][j] == &'X' || &self.board_instance.board[i][j] == &'O'){
                    count += 1;    
                }
            }
        }
        if (count == 9) {
            return true;
        }
        else {
            return false;
        }
            
    }
    
    
}