use lib::Checkers;
use std::io;
    
fn main() {
    let stdin = io::stdin();
    let mut directive = String::new();
    let game = Checkers{ ..Default::default() };
    let mut board_out = game.get_print_string();
    //still need to add the turn number and pretty printing to prompt
    let prompt = game.get_turn().to_string();
    println!("You are now playing checkers.  Here is the board:");
    loop {
	//prompt
	println!("{}", board_out);
	println!("{}", prompt);
	//receive directive
	stdin.read_line(&mut directive);
	//interpret directive
	if directive.eq(&String::from("q")) {
	    break;
	}
	//call appropriate function
	//generate new board output and prompt
	board_out = game.get_print_string();
	break;
    }
    println!("You have finished playing checkers.  Goodbye!");
}
