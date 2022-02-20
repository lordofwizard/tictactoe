pub mod Player{
    enum Type{
        TypeX = 'X',
        TypeO = 'O'
    }
    pub struct Player{
        name: String,
        char_type: Type,
        no_of_win: u8
    }
}