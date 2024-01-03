
// This module provides default implementations for several of the naive types 
// you might use to hold chess-relevant data. While I do not plan on using these 
// types for computations in general, several traits implement default methods 
// for the user which simply offload the work onto relevant helper types, which 
// are expected to directly know how to implement those methods. Thus it is 
// important that those helper types implement the relevant traits and overload 
// these lazy default methods to avoid circular dependency. 
// Also, it is very important for a lot of the logic that rays implement the 
// Iterator trait. It's very convenient to have access to ``for square in ray" 
// syntax. 

use super::{helper_types::*,helper_traits::*};

impl Colored for EnumColor {
    fn get_color(&self) -> EnumColor {
        *self
    }

    fn set_color(&mut self, color: EnumColor) -> () {
        *self = color;
    }

    fn get_opposite_color(&self) -> EnumColor {
        match *self {
            EnumColor::White => EnumColor::Black,
            EnumColor::Black => EnumColor::White,
        }
    }
}

#[derive(Clone, Copy)]
struct RawPieceRep {
    color: EnumColor,
    piece_type: EnumPiecesUncolored,
}

impl Colored for RawPieceRep {
    fn get_color(&self) -> EnumColor {
        self.color.get_color()
    }
    fn set_color(&mut self, color: EnumColor) -> () {
        self.color.set_color(color)
    }
    fn get_opposite_color(&self) -> EnumColor {
        self.color.get_opposite_color()
    }
}

impl Piecey for RawPieceRep {
    fn get_piece_type(&self) -> EnumPiecesUncolored {
        self.piece_type
    }

    fn build_piece(piece_color: EnumColor, piece_type: EnumPiecesUncolored) -> Self {
        RawPieceRep {color: piece_color, piece_type: piece_type}
    }

    fn set_piece_type(&mut self, piece_type: EnumPiecesUncolored) -> () {
        self.piece_type = piece_type
    }
}

impl<T: Piecey> Contentsy for Option<T> {
    type Content = T;
    fn get_contents(&self) -> Option<Self::Content> {
        *self
    }
    fn build_contents(contents: Option<Self::Content>) -> Self {
        contents
    }
}

impl Ranked for EnumRank {
    fn get_rank(&self) -> EnumRank {
        *self
    }
    fn set_rank(&mut self, rank: EnumRank) -> () {
        *self = rank
    }
    fn rank_shift(&self, shift: SmallOffset) -> Option<Self> {
        let rank_number: i8 = match *self {
            EnumRank::One => 0,
            EnumRank::Two => 1,
            EnumRank::Three => 2,
            EnumRank::Four => 3,
            EnumRank::Five => 4,
            EnumRank::Six => 5,
            EnumRank::Seven => 6,
            EnumRank::Eight => 7,
        };
        let offset_number: i8 = match shift {
            SmallOffset::MinusTwo => -2,
            SmallOffset::MinusOne => -1,
            SmallOffset::Stay => 0,
            SmallOffset::PlusOne => 1,
            SmallOffset::PlusTwo => 2,
        };
        match rank_number + offset_number {
            0 => Some(EnumRank::One),
            1 => Some(EnumRank::Two),
            2 => Some(EnumRank::Three),
            3 => Some(EnumRank::Four),
            4 => Some(EnumRank::Five),
            5 => Some(EnumRank::Six),
            6 => Some(EnumRank::Seven),
            7 => Some(EnumRank::Eight),
            _ => None
        }
    }
    fn rank_gap(&self, other_rank: &Self) -> i8 {
        let rank_number: i8 = match *self {
            EnumRank::One => 0,
            EnumRank::Two => 1,
            EnumRank::Three => 2,
            EnumRank::Four => 3,
            EnumRank::Five => 4,
            EnumRank::Six => 5,
            EnumRank::Seven => 6,
            EnumRank::Eight => 7,
        };
        let other_rank_number: i8 = match *other_rank {
            EnumRank::One => 0,
            EnumRank::Two => 1,
            EnumRank::Three => 2,
            EnumRank::Four => 3,
            EnumRank::Five => 4,
            EnumRank::Six => 5,
            EnumRank::Seven => 6,
            EnumRank::Eight => 7,
        };
        other_rank_number - rank_number
    }
}

impl Filed for EnumFile {
    fn get_file(&self) -> EnumFile {
        *self
    }
    fn set_file(&mut self, file: EnumFile) -> () {
        *self = file
    }
    fn file_shift(&self, shift: SmallOffset) -> Option<Self> {
        let file_number:i8 = match *self {
            EnumFile::A => 0,
            EnumFile::B => 1,
            EnumFile::C => 2,
            EnumFile::D => 3,
            EnumFile::E => 4,
            EnumFile::F => 5,
            EnumFile::G => 6,
            EnumFile::H => 7,
        };
        let offset_number: i8 = match shift {
            SmallOffset::MinusTwo => -2,
            SmallOffset::MinusOne => -1,
            SmallOffset::Stay => 0,
            SmallOffset::PlusOne => 1,
            SmallOffset::PlusTwo => 2,
        };
        match file_number + offset_number {
            0 => Some(EnumFile::A),
            1 => Some(EnumFile::B),
            2 => Some(EnumFile::C),
            3 => Some(EnumFile::D),
            4 => Some(EnumFile::E),
            5 => Some(EnumFile::F),
            6 => Some(EnumFile::G),
            7 => Some(EnumFile::H),
            _ => None
        }
    }
    fn file_gap(&self, other_file: &Self) -> i8 {
        let file_number:i8 = match *self {
            EnumFile::A => 0,
            EnumFile::B => 1,
            EnumFile::C => 2,
            EnumFile::D => 3,
            EnumFile::E => 4,
            EnumFile::F => 5,
            EnumFile::G => 6,
            EnumFile::H => 7,
        };
        let other_file_number:i8 = match other_file {
            EnumFile::A => 0,
            EnumFile::B => 1,
            EnumFile::C => 2,
            EnumFile::D => 3,
            EnumFile::E => 4,
            EnumFile::F => 5,
            EnumFile::G => 6,
            EnumFile::H => 7,
        };
        other_file_number - file_number
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct RawSquareRep {
    rank: EnumRank,
    file: EnumFile,
}

impl Ranked for RawSquareRep {
    fn get_rank(&self) -> EnumRank {
        self.rank
    }
    fn set_rank(&mut self, rank: EnumRank) -> () {
        self.rank = rank
    }
}

impl PartialOrd for RawSquareRep {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_file().cmp(&other.get_file()) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Equal => {
                match self.get_rank() == other.get_rank() {
                    true => Some(std::cmp::Ordering::Equal),
                    false => None
                }
            },
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl Filed for RawSquareRep {
    fn get_file(&self) -> EnumFile {
        self.file
    }
    fn set_file(&mut self, file: EnumFile) -> () {
        self.file = file
    }
}

impl Squarey for RawSquareRep {
    fn set_square(&mut self, rank: EnumRank, file: EnumFile) -> () {
        *self = RawSquareRep {rank: rank, file: file}
    }
    fn build_square(rank: EnumRank, file: EnumFile) -> Self {
        RawSquareRep { rank: rank, file: file }
    }
}

impl<SquareRep> Iterator for Ray<SquareRep, (SmallOffset, SmallOffset)>
where SquareRep: Squarey {
    type Item = SquareRep;
    // Move and THEN return the square: this means that the base of the ray is excluded.
    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_pos.rank_shift(self.direction.0) {
            None => None,
            Some(rank_shifted_square) => {
                self.curr_pos = rank_shifted_square;
                match self.curr_pos.file_shift(self.direction.1) {
                    None => None,
                    Some(new_square) => {
                        self.curr_pos = new_square;
                        Some(new_square)
                    }
                }
            }
        }
    }
}

impl<PositionRep, PieceRep> Movey<PositionRep, PieceRep> for ChessMove<PositionRep, PieceRep>
where PositionRep: Squarey, PieceRep: Piecey {
    fn get_move(&self) -> ChessMove<PositionRep, PieceRep> {
        *self
    }
    fn set_move(&mut self, new_move: ChessMove<PositionRep, PieceRep>) -> () {
        *self = new_move
    }
    fn build_move(new_move: ChessMove<PositionRep, PieceRep>) -> Self {
        new_move
    }
}
