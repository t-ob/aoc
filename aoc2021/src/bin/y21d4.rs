use std::str::FromStr;

struct BingoGame {
    draws: Vec<u16>,
    boards: Vec<[[u16; 5]; 5]>
}

impl FromStr for BingoGame {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

fn main() {
    let bingo_game: BingoGame = common::io::read_stdin();
}