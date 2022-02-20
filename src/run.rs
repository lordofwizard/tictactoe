use crate::game::game::Board;
pub struct Run{
    board_instance : Board
}
impl Run{
    fn game_instance(&self){
        let board_instance = Board::new();
    }
    fn run(){
        use std::io; // using it here... because i don't want to keep std functions 
                     // global scope
        let mut input :String = String::new();
        let mut game_state : bool = true;
        
        while game_state == true{
            std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the message");
        }

        
    }
    
}