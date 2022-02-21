use crate::run::Run;
use crate::player::player::Type;
impl Run{
    pub fn win_check(&self) -> Type {  
        if
            ( // first line
                &self.board_instance.board[0][0],
                &self.board_instance.board[0][1],
                &self.board_instance.board[0][2]
            ) == (&'X',&'X',&'X'){
                return Type::TypeX;
            }
        if
            (//first colmn
                &self.board_instance.board[0][0],
                &self.board_instance.board[1][0],
                &self.board_instance.board[2][0]
            ) == (&'X',&'X',&'X'){
                return Type::TypeX;
        }
        if
            ( // first diagonal
                &self.board_instance.board[0][0],
                &self.board_instance.board[1][1],
                &self.board_instance.board[2][2]
            ) == (&'X',&'X',&'X'){
                return Type::TypeX;
        }
        if
            ( // 3rd colmn
                &self.board_instance.board[2][0],
                &self.board_instance.board[2][1],
                &self.board_instance.board[2][2]
            ) == (&'X',&'X',&'X'){
                return Type::TypeX;
        }
        if
            (//second colmn
                &self.board_instance.board[1][0],
                &self.board_instance.board[1][1],
                &self.board_instance.board[1][2]
            ) == (&'X',&'X',&'X'){
                return Type::TypeX;
        }
        if
            (// second diagonal
                &self.board_instance.board[0][2],
                &self.board_instance.board[1][1],
                &self.board_instance.board[2][0]
            ) == (&'X',&'X',&'X'){
                return Type::TypeX;
        }
        if
            (//second line
                &self.board_instance.board[1][0],
                &self.board_instance.board[1][1],
                &self.board_instance.board[1][2]
            ) == (&'X',&'X',&'X'){
                return Type::TypeX;
        }
        if
            ( // third line
                &self.board_instance.board[2][0],
                &self.board_instance.board[2][1],
                &self.board_instance.board[2][2]
            ) == (&'X',&'X',&'X'){
                return Type::TypeX;
        }
        if
            ( // first line
                &self.board_instance.board[0][0],
                &self.board_instance.board[0][1],
                &self.board_instance.board[0][2]
            ) == (&'O',&'O',&'O'){
                return Type::TypeO;
            }
        if
            (//first colmn
                &self.board_instance.board[0][0],
                &self.board_instance.board[1][0],
                &self.board_instance.board[2][0]
            ) == (&'O',&'O',&'O'){
                return Type::TypeO;
        }
        if
            ( // first diagonal
                &self.board_instance.board[0][0],
                &self.board_instance.board[1][1],
                &self.board_instance.board[2][2]
            ) == (&'O',&'O',&'O'){
                return Type::TypeO;
        }
        if
            ( // 3rd colmn
                &self.board_instance.board[2][0],
                &self.board_instance.board[2][1],
                &self.board_instance.board[2][2]
            ) == (&'O',&'O',&'O'){
                return Type::TypeO;
        }
        if
            (//second colmn
                &self.board_instance.board[1][0],
                &self.board_instance.board[1][1],
                &self.board_instance.board[1][2]
            ) == (&'O',&'O',&'O'){
                return Type::TypeO;
        }
        if
            (// second diagonal
                &self.board_instance.board[0][2],
                &self.board_instance.board[1][1],
                &self.board_instance.board[2][0]
            ) == (&'O',&'O',&'O'){
                return Type::TypeO;
        }
        if
            (//second line
                &self.board_instance.board[1][0],
                &self.board_instance.board[1][1],
                &self.board_instance.board[1][2]
            ) == (&'O',&'O',&'O'){
                return Type::TypeO;
        }
        if
            ( // third line
                &self.board_instance.board[2][0],
                &self.board_instance.board[2][1],
                &self.board_instance.board[2][2]
            ) == (&'O',&'O',&'O'){
                return Type::TypeO;
        }
        else {
            Type::TypeO
        }
        
    }
}