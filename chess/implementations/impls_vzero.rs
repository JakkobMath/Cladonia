// This is the first and most basic implementation of the chess traits. 
// Nothing fancy going on, including any kind of bitboard. It also 
// contains functionality used to perft-test the movegen code.

use super::*;

// First: some basic unoptimized types for holding chess data that are hopefully better than the 
// naive implementation one might come up with using abstracts::helper_types.

// Colored for i8.
// Pairing 0 and 1 together, 2 and 3, and so forth. Last bit is color info.
impl Colored for i8 {
    fn get_color(&self) -> crate::chess::abstracts::helper_types::EnumColor {
        match self % 2i8 {
            0i8 => EnumColor::White,
            _ => EnumColor::Black,
        }
    }
    fn set_color(&mut self, color: EnumColor) -> () {
        *self = 2i8 * (*self / 2i8) + match color {
            EnumColor::White => 0i8,
            EnumColor::Black => 1i8,
        }
    }
}

// Piecey for i8.
impl Piecey for i8 {
    fn get_piece_type(&self) -> EnumPiecesUncolored {
        match self / 2i8 {
            0i8 => EnumPiecesUncolored::Pawn,
            1i8 => EnumPiecesUncolored::Knight,
            2i8 => EnumPiecesUncolored::Bishop,
            3i8 => EnumPiecesUncolored::Rook,
            4i8 => EnumPiecesUncolored::Queen,
            _ => EnumPiecesUncolored::King,
        }
    }
    fn build_piece(piece_color: EnumColor, piece_type: EnumPiecesUncolored) -> Self {
        2i8 * match piece_type {
            EnumPiecesUncolored::Pawn => 0i8,
            EnumPiecesUncolored::Knight => 1i8,
            EnumPiecesUncolored::Bishop => 2i8,
            EnumPiecesUncolored::Rook => 3i8,
            EnumPiecesUncolored::Queen => 4i8,
            EnumPiecesUncolored::King => 5i8,
        } + match piece_color {
            EnumColor::White => 0i8,
            EnumColor::Black => 1i8,
        }
    }
}

// Contentsy for i8.
impl Contentsy for i8 {
    type Content = i8;
    fn get_contents(&self) -> Option<Self::Content> {
        match *self < 0i8 {
            true => None,
            false => Some(*self),
        }
    }
    fn build_contents(contents: Option<Self::Content>) -> Self {
        match contents {
            None => -1i8,
            Some(value) => value,
        }
    }
}

// Ranked for i8.
// Rank is stored in the penultimate three bits.
impl Ranked for i8 {
    fn get_rank(&self) -> EnumRank {
        match (*self / 8) % 8 {
            0 => EnumRank::One,
            1 => EnumRank::Two,
            2 => EnumRank::Three,
            3 => EnumRank::Four,
            4 => EnumRank::Five,
            5 => EnumRank::Six,
            6 => EnumRank::Seven,
            _ => EnumRank::Eight,
        }
    }
    fn set_rank(&mut self, rank: EnumRank) -> () {
        *self = 8 * match rank {
            EnumRank::One => 0,
            EnumRank::Two => 1,
            EnumRank::Three => 2,
            EnumRank::Four => 3,
            EnumRank::Five => 4,
            EnumRank::Six => 5,
            EnumRank::Seven => 6,
            EnumRank::Eight => 7,
        } + (*self % 8);
    }
}

// Filed for i8.
// File is stored in the last three bits.
impl Filed for i8 {
    fn get_file(&self) -> EnumFile {
        match *self % 8 {
            0 => EnumFile::A,
            1 => EnumFile::B,
            2 => EnumFile::C,
            3 => EnumFile::D,
            4 => EnumFile::E,
            5 => EnumFile::F,
            6 => EnumFile::G,
            _ => EnumFile::H,
        }
    }
    fn set_file(&mut self, file: EnumFile) -> () {
        *self = 8 * (*self / 8) + match file {
            EnumFile::A => 0,
            EnumFile::B => 1,
            EnumFile::C => 2,
            EnumFile::D => 3,
            EnumFile::E => 4,
            EnumFile::F => 5,
            EnumFile::G => 6,
            EnumFile::H => 7,
        }
    }
}

// Squarey for i8.
impl Squarey for i8 {
    fn build_square(rank: EnumRank, file: EnumFile) -> Self {
        8 * match rank {
            EnumRank::One => 0,
            EnumRank::Two => 1,
            EnumRank::Three => 2,
            EnumRank::Four => 3,
            EnumRank::Five => 4,
            EnumRank::Six => 5,
            EnumRank::Seven => 6,
            EnumRank::Eight => 7,
        } + match file {
            EnumFile::A => 0,
            EnumFile::B => 1,
            EnumFile::C => 2,
            EnumFile::D => 3,
            EnumFile::E => 4,
            EnumFile::F => 5,
            EnumFile::G => 6,
            EnumFile::H => 7,
        }
    }
}

// HasBoard for [i8; 64].
impl HasBoard for [i8; 64] {
    type PositionRep = i8;
    type ContentsRep = i8;
    type MoveRep = ChessMove<i8, i8>;

    const CANONICAL_ARRAY: [Self::PositionRep; 64] = [
            0,  1,  2,  3,  4,  5,  6,  7, 
            8,  9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23, 
        24, 25, 26, 27, 28, 29, 30, 31, 
        32, 33, 34, 35, 36, 37, 38, 39, 
        40, 41, 42, 43, 44, 45, 46, 47, 
        48, 49, 50, 51, 52, 53, 54, 55, 
        56, 57, 58, 59, 60, 61, 62, 63, 
    ];

    fn query_square(&self, square: Self::PositionRep) -> Self::ContentsRep {
        self[square as usize]
    }

    fn set_square(&mut self, square: Self::PositionRep, new_contents: Self::ContentsRep) -> () {
        self[square as usize] = new_contents;
    }
}

// PlyCounting for i8.
impl PlyCounting for i8 {
    fn get_ply_count(&self) -> i8 {
        *self
    }
    fn set_ply_count(&mut self, ply_count: i8) -> () {
        *self = ply_count;
    }
}

// MoveCounting for i16.
impl MoveCounting for i16 {
    fn get_move_count(&self) -> i16 {
        *self
    }

    fn set_move_count(&mut self, move_count: i16) -> () {
        *self = move_count;
    }
}

// The struct we'll be using to store FEN data in this impl.
#[derive(Clone, Copy, Debug)]
pub(crate) struct UnwrappedFen {
    board: [i8; 64],
    moving_side: EnumColor,
    ply_count: i8,
    move_count: i16,
    raw_castling_data: [Option<(i8, i8, i8, i8)>; 4],
    ep_data: i8,
    w_king_square: i8,
    b_king_square: i8,
}

// FENnec for UnwrappedFen.
impl HasBoard for UnwrappedFen {
    type PositionRep = i8;
    type ContentsRep = i8;
    type MoveRep = ChessMove<i8, i8>;

    const CANONICAL_ARRAY: [Self::PositionRep; 64] = <[i8; 64] as HasBoard>::CANONICAL_ARRAY;

    fn query_square(&self, square: Self::PositionRep) -> Self::ContentsRep {
        self.board.query_square(square)
    }
    fn set_square(&mut self, square: Self::PositionRep, new_contents: Self::ContentsRep) -> () {
        self.board.set_square(square, new_contents);
    }
}
impl Colored for UnwrappedFen {
    fn get_color(&self) -> EnumColor {
        self.moving_side
    }
    fn set_color(&mut self, color: EnumColor) -> () {
        self.moving_side = color;
    }
}
impl PlyCounting for UnwrappedFen {
    fn get_ply_count(&self) -> i8 {
        self.ply_count.get_ply_count()
    }
    fn set_ply_count(&mut self, ply_count: i8) -> () {
        self.ply_count.set_ply_count(ply_count)
    }
}
impl MoveCounting for UnwrappedFen {
    fn get_move_count(&self) -> i16 {
        self.move_count.get_move_count()
    }
    fn set_move_count(&mut self, move_count: i16) -> () {
        self.move_count.set_move_count(move_count)
    }
}
impl FENnec for UnwrappedFen {
    fn get_castling(&self, color: EnumColor) -> [Option<CastlingMove<Self::PositionRep>>; 2] {
        let offset = match color {
            EnumColor::White => 0,
            EnumColor::Black => 2,
        };
        let mut castle_rules = [None, None];
        for i in [0, 1] {
            match self.raw_castling_data[i + offset] {
                None => {}
                Some((king_from_square, rook_from_square, king_to_square, rook_to_square)) => {
                    castle_rules[i] = Some(CastlingMove {king_from: king_from_square, rook_from: rook_from_square, king_to: king_to_square, rook_to: rook_to_square});
                }
            }
        }
        castle_rules
    }
    fn set_castling(&mut self, color: EnumColor, new_rules: [Option<CastlingMove<Self::PositionRep>>; 2]) -> () {
        let offset = match color {
            EnumColor::White => 0,
            EnumColor::Black => 2,
        };
        for i in [0, 1] {
            match new_rules[i] {
                None => self.raw_castling_data[i + offset] = None,
                Some(castling_rule) => {
                    self.raw_castling_data[i + offset] = Some((castling_rule.king_from, castling_rule.rook_from, castling_rule.king_to, castling_rule.rook_to))
                }
            }
        }
    }
    fn get_w_king_square(&self) -> Self::PositionRep {
        self.w_king_square
    }
    fn set_w_king_square(&mut self, square: Self::PositionRep) -> () {
        self.w_king_square = square;
    }
    fn get_b_king_square(&self) -> Self::PositionRep {
        self.b_king_square
    }
    fn set_b_king_square(&mut self, square: Self::PositionRep) -> () {
        self.b_king_square = square;
    }
    fn try_get_ep_square(&self) -> Option<(Self::PositionRep, Self::PositionRep)> {
        let ep_to_taken = match self.get_color() {
            EnumColor::White => -8,
            EnumColor::Black => 8,
        };
        match self.ep_data < 0 {
            true => None,
            false => Some((self.ep_data + ep_to_taken, self.ep_data))
        }
    }
    fn set_ep_square(&mut self, value: Option<(Self::PositionRep, Self::PositionRep)>) -> () {
        match value {
            None => self.ep_data = -1,
            Some((_ep_taken, ep_square)) => self.ep_data = ep_square,
        }
    }
}

// Probably the most important position to have on hand.

pub(crate) const STARTPOS: UnwrappedFen = UnwrappedFen {
    board: [
        6,  2,  4,  8, 10,  4,  2,  6, 
        0,  0,  0,  0,  0,  0,  0,  0, 
        -1, -1, -1, -1, -1, -1, -1, -1, 
        -1, -1, -1, -1, -1, -1, -1, -1, 
        -1, -1, -1, -1, -1, -1, -1, -1, 
        -1, -1, -1, -1, -1, -1, -1, -1, 
        1,  1,  1,  1,  1,  1,  1,  1, 
        7,  3,  5,  9, 11,  5,  3,  7, 
    ],
    moving_side: EnumColor::White,
    ply_count: 0,
    move_count: 1,
    raw_castling_data: [
        Some((
            4,
            0,
            2,
            3,
        )),
        Some((
            4,
            7,
            6,
            5,
        )),
        Some((
            56 + 4,
            56 + 0,
            56 + 2,
            56 + 3,
        )),
        Some((
            56 + 4,
            56 + 7,
            56 + 6,
            56 + 5,
        )),
    ],
    ep_data: -1,
    w_king_square: 4,
    b_king_square: 56 + 4,
};

// It's crucial for debugging that things start implementing ToString and Display. Perhaps this 
// code will be moved out to another spot later, but I'm not doing that quite yet because the 
// *parser* for UnwrappedFen isn't fully general (it can only handle standard castling rules). 

impl ToString for EnumRank {
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

pub(crate) fn standardize<SquareRep: Squarey> (square: SquareRep) -> StandardSquare {
    StandardSquare {
        rank: square.get_rank(),
        file: square.get_file(),
    }
}

impl ToString for StandardSquare {
    fn to_string(&self) -> String {
        let rank_string = self.rank.to_string();
        let mut square_string = self.file.to_string();
        square_string.push_str(&rank_string);
        square_string
    }
}

impl ToString for EnumPiecesUncolored {
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

// Movegen and perft code. 

fn legal_successor_positions(fen: UnwrappedFen) -> Vec<UnwrappedFen> {
    let mut successors = Vec::new();
    for legal_move in fen.get_legal_proper_moves() {
        let mut changed_pos = fen;
        changed_pos.make_move(legal_move);
        successors.push(changed_pos)
    }
    successors
}

pub(crate) fn depth_n_total_perft(fen: UnwrappedFen, mut n: i8) -> usize {
    let mut curr_list = Vec::new();
    curr_list.push(fen);
    while n > 1 {
        let mut new_list = Vec::new();
        for old_fen in curr_list {
            new_list.append(&mut legal_successor_positions(old_fen))
        }
        curr_list = new_list;
        n -= 1;
    }
    if n == 1 {
        let mut total = 0;
        for old_fen in curr_list {
            let addition = old_fen.get_legal_proper_moves().len();
            total += addition
        }
        return total
    } 
    1
}

pub(crate) fn depth_n_better_perft(fen: UnwrappedFen, n: i8) -> (usize, Vec<(ChessMove<i8, i8>, usize)>) {
    let mut sub_perfts = Vec::new();
    let mut grand_total = 0;
    for legal_move in fen.get_legal_proper_moves() {
        let mut possible_successor = fen;
        possible_successor.make_move(legal_move);

        let successors_here = depth_n_total_perft(possible_successor, n-1);
        sub_perfts.push((legal_move, successors_here));
        grand_total += successors_here;
    }
    (grand_total, sub_perfts)
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

// Necessary because UnwrappedFen reads off rows in reverse order from what's canonical for FENs. 
// I'll probably change conventions for the next version in an impls_vone module to avoid this and 
// improve readability. 
fn vertical_flip_index(square: i8) -> i8 {
    let original_rank = square.get_rank();
    let file = square.get_file();
    let new_rank = match original_rank {
        EnumRank::One => EnumRank::Eight,
        EnumRank::Two => EnumRank::Seven,
        EnumRank::Three => EnumRank::Six,
        EnumRank::Four => EnumRank::Five,
        EnumRank::Five => EnumRank::Four,
        EnumRank::Six => EnumRank::Three,
        EnumRank::Seven => EnumRank::Two,
        EnumRank::Eight => EnumRank::One,
    };
    i8::build_square(new_rank, file)
}

// *Apparently* this isn't in the standard library for Rust. It's fine, I need like zero fancy 
// functionality from it- only so I can separate some distinct but related blocks of code without 
// breaking the dependency of one on the other. 
enum Either<L, R> {
    Left(L),
    Right(R),
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

// Some very basic search/evaluator code. Probably this file gets broken up 
// into multiple modules and then multiple files. 

// MVVLVA (most valuable victim, least valuable attacker) is used to get a 
// basic move ordering to work for an AB search. 

fn get_piece_value(piece: EnumPiecesUncolored) -> i16 {
    match piece {
        EnumPiecesUncolored::Pawn => 100,
        EnumPiecesUncolored::Knight => 300,
        EnumPiecesUncolored::Bishop => 310,
        EnumPiecesUncolored::Rook => 500,
        EnumPiecesUncolored::Queen => 900,
        EnumPiecesUncolored::King => 1900,
    }
}

fn naive_evaluation(position: &UnwrappedFen) -> i16 {
    let mut sum = 0;
    for square_index in 0..64 {
        match position.board[square_index].get_contents() {
            None => {},
            Some(piece) => {
                sum += get_piece_value(piece.get_piece_type()) * match piece.get_color() {
                    EnumColor::Black => -1,
                    EnumColor::White => 1,
                }
            },
        }
    }
    sum
}

// Higher score -> put move earlier. 
// Not doing anything fancy yet- basically just raw naive MVV-LVA with some special cases. 
// Might add in a forward movement bonus later or something. ... maybe just adding it now. 
fn mvv_lva_score(position: &UnwrappedFen, move_to_make: <UnwrappedFen as HasBoard>::MoveRep) -> i16 {
    match move_to_make {
        ChessMove::NullMove => 0,
        ChessMove::StandardMove(some_standard_move) => {
            20 * match position.query_square(some_standard_move.to_square).get_contents() {
                None => 0,
                Some(piece) => get_piece_value(piece.get_piece_type()),
            } - match position.query_square(some_standard_move.from_square).get_contents() {
                None => 0,
                Some(piece) => get_piece_value(piece.get_piece_type()),
            } + (some_standard_move.to_square.rank_gap(&some_standard_move.from_square) as i16)
        },
        ChessMove::EnPassantMove(_ep_move) => 1950,
        ChessMove::CastlingMove(_castling_move) => 4050, 
        ChessMove::PromotionMove(some_promotion_move) => {
            20 * match position.query_square(some_promotion_move.to_square).get_contents() {
                None => 0,
                Some(piece) => get_piece_value(piece.get_piece_type()),
            } + 300 + get_piece_value(some_promotion_move.promotion_choice.get_piece_type()) * 2 
            // -100 for pawn lost, + 500 * (1-x) to pull towards rough average of promotion 
            // values, + x * get_piece_value(piece) for weighting higher-value promotions more 
            // highly, x = 1/2. All that times 4. Promoting isn't as urgent as a capture perhaps, 
            // but probably more so than a normal move. This either gets discarded or tuned later. 
        },
    }
}

// TODO: MVVLVA

// Pretty much the most naive evaluation short of literally just guessing. 

// TODO: material difference in centipawns, bishop = 3.1. Not even any king
// safety stuff yet, just get any kind of search working first. 

// TODO: implement Evaluator etc for that evaluation. 

// TODO: figure out a basic data structure to hold a search tree where leaf nodes are readily 
// accessible, and implement Searches for it using the evaluator above and AB search. Maybe 
// do negamax first. 
