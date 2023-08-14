use std::convert::TryInto;
use std::fmt;
use std::io::{self, Read};

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Copy, Clone, PartialEq)]
enum PlayerToken {
    Token(Player),
    NoToken,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

impl fmt::Display for PlayerToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlayerToken::Token(Player::X) => write!(f, "X"),
            PlayerToken::Token(Player::O) => write!(f, "O"),
            PlayerToken::NoToken => write!(f, " "),
        }
    }
}

#[derive(Copy, Clone)]
struct TicTacToe {
    board: [[PlayerToken; 3]; 3],
    current_player: Player,
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.board
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|token| token.to_string())
                        .collect::<Vec<_>>()
                        .join(" ┃ ")
                })
                .collect::<Vec<_>>()
                .join("\n━━╋━━━╋━━\n")
        )
    }
}

impl TicTacToe {
    fn cell_is_empty(&self, x: i32, y: i32) -> bool {
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        match self.board.get(y).and_then(|row: &_| row.get(x)) {
            Some(PlayerToken::NoToken) => return true,
            _ => return false,
        }
    }

    fn get_next_state(self, x: i32, y: i32) -> TicTacToe {
        if self.cell_is_empty(x, y) {
            let mut next_state = TicTacToe {
                board: self.board,
                current_player: match self.current_player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                },
            };
            let x: usize = x.try_into().unwrap();
            let y: usize = y.try_into().unwrap();
            next_state.board[y][x] = PlayerToken::Token(self.current_player);
            return next_state;
        }
        self
    }

    fn is_draw(self) -> bool {
        for row in self.board.iter() {
            for token in row.iter() {
                match token { PlayerToken::NoToken => return false, _ => continue}
            }
        }
        true
    }

    fn winner(self) -> Option<Player> {
        fn check_group(group: [PlayerToken; 3]) -> Option<Player> {
            if let PlayerToken::Token(p) = group[0] {
                if group.iter().all(|&token| token == PlayerToken::Token(p)) {
                    return Some(p);
                }
            }
            None
        }
        check_group(self.board[0])
    }
}

fn parse_input(input: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = input.trim().split(',').collect();
    if parts.len() == 2 {
        if let (Ok(x), Ok(y)) = (parts[0].parse(), parts[1].parse()) {
            return Some((x, y));
        }
    }
    None
}

fn main() {
    let mut ttt: TicTacToe = TicTacToe {
        board: [[PlayerToken::NoToken; 3]; 3],
        current_player: Player::X,
    };

    while ttt.winner().is_none() && !ttt.is_draw() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", ttt);

        let mut input = String::new();
        println!("Please make a move, {}", ttt.current_player);
        
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Some((x, y)) = parse_input(&input) {
            ttt = ttt.get_next_state(x, y);
            println!("x: {}, y: {}", x, y);
        }
        println!("Input: {}", input);
    }

    println!("{}", ttt);
    if let Some(winner) = ttt.winner() {
        println!("{} has won!", winner);
    } else {
        println!("It's a draw!");
    }
}
    
