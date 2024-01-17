// It's crucial for debugging that things start implementing ToString and Display. Perhaps this 
// code will be moved out to another spot later, but I'm not doing that quite yet because the 
// *parser* for UnwrappedFen isn't fully general (it can only handle standard castling rules). 

use crate::chess::abstracts::{helper_types::*, helper_traits::*};
use super::board_rep::*;

// *Apparently* this isn't in the standard library for Rust. It's fine, I need like zero fancy 
// functionality from it- only so I can separate some distinct but related blocks of code without 
// breaking the dependency of one on the other. 
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl ToString for EnumRank {
    #[inline(always)]
    fn to_string(&self) -> String {
        match *self {
            EnumRank::One => (*"1").to_string(),
            EnumRank::Two => (*"2").to_string(),
            EnumRank::Three => (*"3").to_string(),
            EnumRank::Four => (*"4").to_string(),
            EnumRank::Five => (*"5").to_string(),
            EnumRank::Six => (*"6").to_string(),
            EnumRank::Seven => (*"7").to_string(),
            EnumRank::Eight => (*"8").to_string(),
        }
    }
}

impl ToString for EnumFile {
    #[inline(always)]
    fn to_string(&self) -> String {
        match *self {
            EnumFile::A => (*"A").to_string(),
            EnumFile::B => (*"B").to_string(),
            EnumFile::C => (*"C").to_string(),
            EnumFile::D => (*"D").to_string(),
            EnumFile::E => (*"E").to_string(),
            EnumFile::F => (*"F").to_string(),
            EnumFile::G => (*"G").to_string(),
            EnumFile::H => (*"H").to_string(),
        }
    }
}

pub(crate) struct StandardSquare {
    rank: EnumRank,
    file: EnumFile,
}

#[inline(always)]
pub(crate) fn standardize<SquareRep: Squarey> (square: SquareRep) -> StandardSquare {
    StandardSquare {
        rank: square.get_rank(),
        file: square.get_file(),
    }
}

impl ToString for StandardSquare {
    #[inline(always)]
    fn to_string(&self) -> String {
        let rank_string = self.rank.to_string();
        let mut square_string = self.file.to_string();
        square_string.push_str(&rank_string);
        square_string
    }
}

impl ToString for EnumPiecesUncolored {
    #[inline(always)]
    fn to_string(&self) -> String {
        match *self {
            Self::Pawn => (*"").to_string(),
            Self::Knight => (*"N").to_string(),
            Self::Bishop => (*"B").to_string(),
            Self::Rook => (*"R").to_string(),
            Self::Queen => (*"Q").to_string(),
            Self::King => (*"K").to_string(),
        }
    }
}

impl std::fmt::Display for ChessMove<i8, i8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (start_sq, end_sq) = match *self {
            Self::NullMove => {
                ("00".to_string(), "00".to_string())
            },
            Self::StandardMove(some_move) => {
                (standardize(some_move.from_square).to_string(), standardize(some_move.to_square).to_string())
            },
            Self::EnPassantMove(some_move) => {
                (standardize(some_move.from_square).to_string(), standardize(some_move.to_square).to_string())
            },
            Self::CastlingMove(some_move) => {
                (standardize(some_move.king_from).to_string(), standardize(some_move.king_to).to_string())
            },
            Self::PromotionMove(some_move) => {
                (standardize(some_move.from_square).to_string(), standardize(some_move.to_square).to_string())
            },
        };
        match write!(f, "{}", start_sq) {
            Ok(_val) => {},
            Err(error) => return Result::Err(error)
        };
        match write!(f, "{}", end_sq) {
            Ok(_val) => {},
            Err(error) => return Result::Err(error)
        };
        match *self {
            Self::PromotionMove(some_move) => {
                write!(f, "={}", some_move.promotion_choice.get_piece_type().to_string())
            },
            _ => {
                Result::Ok(())
            },
        }
    }
}


// Code for parsing a true FEN string into an UnwrappedFen. 

// For the finite state machine I'll be using to parse FENs. 
enum FenInterpretationState {
    ReadingPieces(i8),
    ReadingColor,
    ReadingCastling,
    ReadingEPFile,
    ReadingEPRank(EnumFile),
    ReadingHalfMove(i8),
    ReadingFullMove(i16),
}


// The finite state machine for parsing FENs. This may accept non-FEN strings, but should always 
// parse valid FEN strings correctly. 
#[allow(dead_code)]
pub(crate) fn interpret_fen(fen_str: String) -> Result<UnwrappedFen, String> {

    // Data needed to produce an UnwrappedFen, mostly with invalid default values. The real values 
    // will be filled out by the state machine, which will return an error message if something 
    // goes wrong or not all the necessary pieces of data are changed. 
    let mut board_state = [-1i8; 64];
    let mut color = None;
    let mut w_king_square = -1i8;
    let mut b_king_square = -1i8;
    let mut castle_rules = [None, None, None, None];
    let mut ep_square = None;
    let mut half_moves = 0i8;
    let mut full_moves = 0i16;

    // The mutable state variable. 
    let mut curr_state = FenInterpretationState::ReadingPieces(0);

    // Read in the characters of the FEN string one at a time. FENs canonically list out their 
    // data in a certain order with spaces as separators and sometimes '-' characters that 
    // effectively just improve the visibility of empty fields for humans. The states correspond 
    // to the different kinds of data we might be reading at the moment, and we generally move to 
    // the next state after encountering a space. 
    for character in fen_str.chars() {
        match curr_state {
            FenInterpretationState::ReadingPieces(square_index) => {

                // We will either read in a piece (left) or an instruction to skip some number of 
                // empty squares (right). Sprinkled throughout are line separators which we ignore 
                // under the assumption that the FEN string is correctly formatted, with a space 
                // indicating transition to the next state. 
                let read_result = match character {
                    'P' => Either::Left(i8::build_piece(
                        EnumColor::White, 
                        EnumPiecesUncolored::Pawn
                    )),
                    'p' => Either::Left(i8::build_piece(
                        EnumColor::Black, 
                        EnumPiecesUncolored::Pawn
                    )),
                    'N' => Either::Left(i8::build_piece(
                        EnumColor::White, 
                        EnumPiecesUncolored::Knight
                    )),
                    'n' => Either::Left(i8::build_piece(
                        EnumColor::Black, 
                        EnumPiecesUncolored::Knight
                    )),
                    'B' => Either::Left(i8::build_piece(
                        EnumColor::White, 
                        EnumPiecesUncolored::Bishop
                    )),
                    'b' => Either::Left(i8::build_piece(
                        EnumColor::Black, 
                        EnumPiecesUncolored::Bishop
                    )),
                    'R' => Either::Left(i8::build_piece(
                        EnumColor::White, 
                        EnumPiecesUncolored::Rook
                    )),
                    'r' => Either::Left(i8::build_piece(
                        EnumColor::Black, 
                        EnumPiecesUncolored::Rook
                    )),
                    'Q' => Either::Left(i8::build_piece(
                        EnumColor::White, 
                        EnumPiecesUncolored::Queen
                    )),
                    'q' => Either::Left(i8::build_piece(
                        EnumColor::Black, 
                        EnumPiecesUncolored::Queen
                    )),
                    'K' => {
                        w_king_square = vertical_flip_index(square_index);
                        Either::Left(i8::build_piece(
                            EnumColor::White, 
                            EnumPiecesUncolored::King
                        ))
                    },
                    'k' => {
                        b_king_square = vertical_flip_index(square_index);
                        Either::Left(i8::build_piece(
                            EnumColor::Black, 
                            EnumPiecesUncolored::King
                        ))
                    },
                    ' ' => {
                        curr_state = FenInterpretationState::ReadingColor;
                        Either::Right(None)
                    },
                    '/' => Either::Right(None),
                    _ => Either::Right(character.to_digit(10)),
                };
                match read_result {
                    Either::Left(piece) => {
                        board_state[vertical_flip_index(square_index) as usize] = piece;
                        curr_state = FenInterpretationState::ReadingPieces(square_index + 1);
                    },
                    Either::Right(skipping) => {
                        match skipping {
                            None => {},
                            Some(number) => {
                                curr_state = FenInterpretationState::ReadingPieces(square_index + number as i8);
                            },
                        }
                    },
                }
            },
            FenInterpretationState::ReadingColor => {
                match character {
                    'w' => {
                        color = Some(EnumColor::White)
                    },
                    'b' => {
                        color = Some(EnumColor::Black)
                    },
                    ' ' => {
                        curr_state = FenInterpretationState::ReadingCastling;
                    },
                    _ => return Err("Couldn't read active color.".to_string())
                };
            },
            FenInterpretationState::ReadingCastling => {

                // This is the least general part of the parser: it will only construct 
                // UnwrappedFENs where the castling rules send kings to the C or G files and 
                // Rooks to the D or F files. This establishes a convention that the four entries 
                // of the UnwrappedFEN castling rules array represent kingside castling first and 
                // then queenside, but I have no plans to ever use this as an assumption. 
                match character {
                    'K' => castle_rules[0] = Some((
                        i8::build_square(EnumRank::One, EnumFile::E),
                        i8::build_square(EnumRank::One, EnumFile::H),
                        i8::build_square(EnumRank::One, EnumFile::G),
                        i8::build_square(EnumRank::One, EnumFile::F)
                    )),
                    'Q' => castle_rules[1] = Some((
                        i8::build_square(EnumRank::One, EnumFile::E),
                        i8::build_square(EnumRank::One, EnumFile::A),
                        i8::build_square(EnumRank::One, EnumFile::C),
                        i8::build_square(EnumRank::One, EnumFile::D)
                    )),
                    'k' => castle_rules[2] = Some((
                        i8::build_square(EnumRank::Eight, EnumFile::E),
                        i8::build_square(EnumRank::Eight, EnumFile::H),
                        i8::build_square(EnumRank::Eight, EnumFile::G),
                        i8::build_square(EnumRank::Eight, EnumFile::F)
                    )),
                    'q' => castle_rules[3] = Some((
                        i8::build_square(EnumRank::Eight, EnumFile::E),
                        i8::build_square(EnumRank::Eight, EnumFile::A),
                        i8::build_square(EnumRank::Eight, EnumFile::C),
                        i8::build_square(EnumRank::Eight, EnumFile::D)
                    )),
                    ' ' => curr_state = FenInterpretationState::ReadingEPFile,
                    '-' => {},
                    _ => return Err("Unexpected character when reading castling rules".to_string()),
                }
            },
            FenInterpretationState::ReadingEPFile => {

                // Ignore the empty field character completely. If we can detect a valid file, 
                // move to looking for the rank next. Any invalid character (canonically this 
                // should only ever be a space for valid FENs) sends us to the next state. 

                let try_ep_file = match character {
                    'a' => Some(EnumFile::A),
                    'b' => Some(EnumFile::B),
                    'c' => Some(EnumFile::C),
                    'd' => Some(EnumFile::D),
                    'e' => Some(EnumFile::E),
                    'f' => Some(EnumFile::F),
                    'g' => Some(EnumFile::G),
                    'h' => Some(EnumFile::H),
                    '-' => {
                        continue;
                    }
                    _ => None,
                };
                curr_state = match try_ep_file {
                    None => FenInterpretationState::ReadingHalfMove(0i8),
                    Some(ep_file) => FenInterpretationState::ReadingEPRank(ep_file),
                }
            },
            FenInterpretationState::ReadingEPRank(ep_file) => {

                // We only ever end up here if we read in a valid ep_file, which is passed 
                // in as part of the state data to ensure that we have access to it without 
                // making a dedicated mutable variable to hold it up with the FEN data 
                // we've been writing to. 

                let try_ep_rank = match character {
                    '1' => Some(EnumRank::One),
                    '2' => Some(EnumRank::Two),
                    '3' => Some(EnumRank::Three),
                    '4' => Some(EnumRank::Four),
                    '5' => Some(EnumRank::Five),
                    '6' => Some(EnumRank::Six),
                    '7' => Some(EnumRank::Seven),
                    '8' => Some(EnumRank::Eight),
                    _ => None,
                };
                match try_ep_rank {
                    None => return Err("Unexpected character when trying to read ep rank.".to_string()),
                    Some(ep_rank) => ep_square = Some(i8::build_square(ep_rank, ep_file)),
                };
                curr_state = FenInterpretationState::ReadingHalfMove(0)
            },
            FenInterpretationState::ReadingHalfMove(prev_digits) => {

                // Halfmoves and full moves are given as base-10 numbers, so we need some memory 
                // as part of the state to hold whatever number is currently in the digit-reading 
                // accumulator thing. If we can read in another digit the prev_digits are 
                // reinterpreted as being one more base-10 place to the left than on the previous 
                // step. Overflows are possible, but not by passing in a FEN string that arises 
                // from a legal game of chess starting at the canonical startpos or a 960 position. 
                match character {
                    ' ' => {
                        half_moves = prev_digits;
                        curr_state = FenInterpretationState::ReadingFullMove(0);
                    },
                    _ => {
                        match character.to_digit(10) {
                            None => return Err("Unexpected character when trying to read halfmove count.".to_string()),
                            Some(digit) => curr_state = FenInterpretationState::ReadingHalfMove(10 * prev_digits + digit as i8),
                        }
                    }
                }
            },
            FenInterpretationState::ReadingFullMove(prev_digits) => {

                // See the half move description. The difference here is that we need to store the 
                // accumulator at each step since we aren't sure when the string will end and we 
                // don't get a terminating space. 
                match character.to_digit(10) {
                    None => return Err("Unexpected character when trying to read fullmove count.".to_string()),
                    Some(digit) => {
                        full_moves = 10 * prev_digits + digit as i16;
                        curr_state = FenInterpretationState::ReadingFullMove(full_moves);
                    },
                }
            },
        }
    }

    // Apologies for the braces here. This code just makes sure 
    // each piece of the FEN we're building is valid. 
    match color {
        None => Err("Color is somehow missing.".to_string()),
        Some(true_color) => {
            match w_king_square < 0 {
                true => Err("White king undetected.".to_string()),
                false => {
                    match b_king_square < 0 {
                        true => Err("Black king undetected.".to_string()),
                        false => {
                            Ok(
                                UnwrappedFen {
                                    board: board_state,
                                    moving_side: true_color,
                                    ply_count: half_moves,
                                    move_count: full_moves,
                                    raw_castling_data: castle_rules,
                                    ep_data: match ep_square {
                                        None => -1i8,
                                        Some(square) => square,
                                    },
                                    w_king_square: w_king_square,
                                    b_king_square: b_king_square
                                }
                            )
                        }
                    }
                }
            }
        }
    }
}