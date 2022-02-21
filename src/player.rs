pub mod player{
    pub enum Type{
        TypeX,
        TypeO
    }
    pub struct Player{
        pub name: String,
        pub char_type: Type,
        pub no_of_win: u8
    }
}