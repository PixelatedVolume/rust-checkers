use array2d::Array2D;
use std::fmt;

#[derive(Copy, Clone)]
pub enum PlayerColor {
    White,
    Black,
}

impl fmt::Display for PlayerColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "{}",
	       match self {
		   White => "White",
		   Black => "Black"
	       })
    }
}

#[derive(Copy, Clone)]
pub enum MoveError {
    WrongTurn,
    NoPiece,
    NotInLine,
    DestOccupied,
    Backwards
}

#[derive(Copy, Clone)]
pub struct Piece {
    team: PlayerColor,
    king: bool
}

pub struct Checkers {
    pub board: Array2D<Option<Piece>>,
    pub turn: PlayerColor,
    pub turn_num: u32,
    pub num_white: u8,
    pub num_black: u8
}

impl Default for Checkers {
    fn default() -> Checkers {
	let mut new_board = Array2D::fill_with(None, 8, 8);
	//somehow set the initial board position
	Checkers {
	    board: new_board,
	    turn: PlayerColor::White,
	    turn_num: 0,
	    num_black: 12,
	    num_white: 12,
	}
    }
}

impl Checkers {
    pub fn step(&mut self, src_coords: (u8, u8), dst_coords: (u8, u8), player: PlayerColor) -> Result<(), MoveError> {
	unimplemented!();
    }

    pub fn get_turn(&self) -> PlayerColor {
	self.turn
    }
    pub fn get_print_string(&self) -> String {
	unimplemented!();
    }
    
    fn check_win(&self) -> bool {
	unimplemented!();
    }
}
    
