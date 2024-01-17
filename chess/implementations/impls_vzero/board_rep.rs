// Some basic unoptimized types for holding chess data that are hopefully better than the 
// naive implementation one might come up with using abstracts::helper_types.

use crate::chess::abstracts::{helper_traits::*, helper_types::*};

// Colored for i8.
// Pairing 0 and 1 together, 2 and 3, and so forth. Last bit is color info.
impl Colored for i8 {
    #[inline(always)]
    fn get_color(&self) -> crate::chess::abstracts::helper_types::EnumColor {
        match self % 2i8 {
            0i8 => EnumColor::White,
            _ => EnumColor::Black,
        }
    }
    #[inline(always)]
    fn set_color(&mut self, color: EnumColor) -> () {
        *self = 2i8 * (*self / 2i8) + match color {
            EnumColor::White => 0i8,
            EnumColor::Black => 1i8,
        }
    }
}

// Piecey for i8.
impl Piecey for i8 {
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn get_contents(&self) -> Option<Self::Content> {
        match *self < 0i8 {
            true => None,
            false => Some(*self),
        }
    }
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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

    #[inline(always)]
    fn query_square(&self, square: Self::PositionRep) -> Self::ContentsRep {
        self[square as usize]
    }

    #[inline(always)]
    fn set_square(&mut self, square: Self::PositionRep, new_contents: Self::ContentsRep) -> () {
        self[square as usize] = new_contents;
    }
}

// PlyCounting for i8.
impl PlyCounting for i8 {
    #[inline(always)]
    fn get_ply_count(&self) -> i8 {
        *self
    }
    #[inline(always)]
    fn set_ply_count(&mut self, ply_count: i8) -> () {
        *self = ply_count;
    }
}

// MoveCounting for i16.
impl MoveCounting for i16 {
    #[inline(always)]
    fn get_move_count(&self) -> i16 {
        *self
    }

    #[inline(always)]
    fn set_move_count(&mut self, move_count: i16) -> () {
        *self = move_count;
    }
}

// The struct we'll be using to store FEN data in this impl.
#[derive(Clone, Copy, Debug)]
pub(crate) struct UnwrappedFen {
    pub(super) board: [i8; 64],
    pub(super) moving_side: EnumColor,
    pub(super) ply_count: i8,
    pub(super) move_count: i16,
    pub(super) raw_castling_data: [Option<(i8, i8, i8, i8)>; 4],
    pub(super) ep_data: i8,
    pub(super) w_king_square: i8,
    pub(super) b_king_square: i8,
}

// FENnec for UnwrappedFen.
impl HasBoard for UnwrappedFen {
    type PositionRep = i8;
    type ContentsRep = i8;
    type MoveRep = ChessMove<i8, i8>;

    const CANONICAL_ARRAY: [Self::PositionRep; 64] = <[i8; 64] as HasBoard>::CANONICAL_ARRAY;

    #[inline(always)]
    fn query_square(&self, square: Self::PositionRep) -> Self::ContentsRep {
        self.board.query_square(square)
    }
    #[inline(always)]
    fn set_square(&mut self, square: Self::PositionRep, new_contents: Self::ContentsRep) -> () {
        self.board.set_square(square, new_contents);
    }
}
impl Colored for UnwrappedFen {
    #[inline(always)]
    fn get_color(&self) -> EnumColor {
        self.moving_side
    }
    #[inline(always)]
    fn set_color(&mut self, color: EnumColor) -> () {
        self.moving_side = color;
    }
}
impl PlyCounting for UnwrappedFen {
    #[inline(always)]
    fn get_ply_count(&self) -> i8 {
        self.ply_count.get_ply_count()
    }
    #[inline(always)]
    fn set_ply_count(&mut self, ply_count: i8) -> () {
        self.ply_count.set_ply_count(ply_count)
    }
}
impl MoveCounting for UnwrappedFen {
    #[inline(always)]
    fn get_move_count(&self) -> i16 {
        self.move_count.get_move_count()
    }
    #[inline(always)]
    fn set_move_count(&mut self, move_count: i16) -> () {
        self.move_count.set_move_count(move_count)
    }
}
impl FENnec for UnwrappedFen {
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn get_w_king_square(&self) -> Self::PositionRep {
        self.w_king_square
    }
    #[inline(always)]
    fn set_w_king_square(&mut self, square: Self::PositionRep) -> () {
        self.w_king_square = square;
    }
    #[inline(always)]
    fn get_b_king_square(&self) -> Self::PositionRep {
        self.b_king_square
    }
    #[inline(always)]
    fn set_b_king_square(&mut self, square: Self::PositionRep) -> () {
        self.b_king_square = square;
    }
    #[inline(always)]
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
    #[inline(always)]
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

// Necessary because UnwrappedFen reads off rows in reverse order from what's canonical for FENs. 
// I'll probably change conventions for the next version in an impls_vone module to avoid this and 
// improve readability. 
#[inline(always)]
pub(super) fn vertical_flip_index(square: i8) -> i8 {
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