// This is the first and most basic implementation of the chess traits. 
// Nothing fancy going on, including any kind of bitboard. It also 
// contains functionality used to perft-test the movegen code.

use super::*;

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

enum FenInterpretationState {
    ReadingPieces(i8),
    ReadingColor,
    ReadingCastling,
    ReadingEPFile,
    ReadingEPRank(EnumFile),
    ReadingHalfMove(i8),
    ReadingFullMove(i16),
}

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

enum Either<L, R> {
    Left(L),
    Right(R),
}

#[allow(dead_code)]
pub(crate) fn interpret_fen(fen_str: String) -> Result<UnwrappedFen, String> {
    // State machine?
    let mut board_state = [-1i8; 64];
    let mut color = None;
    let mut curr_state = FenInterpretationState::ReadingPieces(0);
    let mut w_king_square = -1i8;
    let mut b_king_square = -1i8;
    let mut castle_rules = [None, None, None, None];
    let mut ep_square = None;
    let mut half_moves = 0i8;
    let mut full_moves = 0i16;

    for character in fen_str.chars() {
        // println!("Processing character \"{}\"", character);
        match curr_state {
            FenInterpretationState::ReadingPieces(square_index) => {
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