// This is the first and most basic implementation of the chess traits. 
// Nothing fancy going on, including any kind of bitboard. It also 
// contains functionality used to perft-test the movegen code.

pub(crate) mod board_rep;

pub(crate) mod io_code;

pub(crate) mod movegen;

pub(crate) mod eval_code;
