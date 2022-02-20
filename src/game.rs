
pub mod game{
    pub struct Board{
        // Struct that stores a Board Array of 3x3
        // Which has characters stores init mainly 
        // ' ' Or 'X' 'O'
        board : [[char;3];3],
    }
    impl Board{
        pub fn new() -> Self{
            Board::make_board()
        }
        fn make_board() -> Self {
            Board{
                board : [
                    [' ',' ',' '],
                    [' ',' ',' '],
                    [' ',' ',' ']
                ]
            }
        }
        pub fn print_board(&self){
            /*
                Eazy Implementation of Priting board values based on their indexces
                Keep in mind :-
                x = 0, y = 0
                is equal to
                row = 1 column = 1
            */
            println!("{} | {} | {}",self.board[0][0],self.board[0][1],self.board[0][2]);
            println!("--|---|--");
            println!("{} | {} | {}",self.board[1][0],self.board[1][1],self.board[1][2]);
            println!("--|---|--");
            println!("{} | {} | {}",self.board[2][0],self.board[2][1],self.board[2][2]);
        }
        #[allow(dead_code)]
        pub fn put_x(&mut self,row:usize,col:usize){
            /*  This function takes 2 ints and puts them in 
                Proper positions.. and col-1 and row-1 the "-1" exists
                because array starts at 0
            */
            let mut temp_board = self.board;
            temp_board[row-1][col-1] = 'X';
            self.board = temp_board;
        }
        #[allow(dead_code)]
        pub fn put_o(&mut self,row:usize,col:usize){
            let mut temp_board = self.board;
            temp_board[row-1][col-1] = 'O';
            self.board = temp_board;
        }
        pub fn filled_check(&self) -> bool{
            for i in 0..3{
                for j in 0..3{
                    if self.board[i][j] == ' '{
                        return false;
                    }
                }
            }
            true
        }
        pub fn fill_with_x(&mut self){
            self.board = [
                        ['X','X','X'],
                        ['X','X','X'],
                        ['X','X','X']
                    ];
            
        }
    }
}