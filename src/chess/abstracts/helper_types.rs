
// This module contains types for representing chess-specific information. It exists 
// largely to add an extra layer of type safety, so that it becomes impossible for 
// instance to accidentally read a file as a rank in the movegen code. THESE TYPES 
// ARE NOT MEANT TO BECOME FIELDS IN ANY ``FEN" STRUCT LATER. I'm not crazy enough to 
// build on these directly, they're just here to keep me honest when I say I've implemented 
// the traits in the next module! If all goes well, almost everything here will be inlined 
// completely out of existence by the compiler everywhere they appear, and later types 
// implementing efficient chess-related traits will probably avoid explicit reference to 
// these as much as possible. I am really taking Rust at its word when it advertises 
// zero-cost abstractions here. 

#![allow(dead_code)]
use super::helper_traits::*;

// The color of a chess piece, or the side to move in an ongoing game. 
// This had better be a synonym for a boolean under the hood. 
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum EnumColor {
    White,
    Black,
}

// The kinds of chess pieces. 
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum EnumPiecesUncolored {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

// The ranks on a chess board. I know I could use i8s or somthing here. 
// I'm trusting Rust to make the smart structural decisions that I refuse 
// to directly have anything to do with in this module. 
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum EnumRank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

// The files on a chess board. See above. 
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum EnumFile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

// This one specifically has got to be extra bad. Pieces move 
// at most two squares in a given direction in a single step. 
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum SmallOffset {
    MinusTwo,
    MinusOne,
    Stay,
    PlusOne,
    PlusTwo,
}

// Rays should be useful for sliding pieces later (before implementing bitboards). 
// They become iterators later with conditions on Pos and Dir. 
#[derive(Debug)]
pub(crate) struct Ray<Pos, Dir> {
    pub(crate) curr_pos: Pos,
    pub(crate) direction: Dir,
}

// Building up to the match-statement-amenable representation of chess moves. 
#[derive(Clone, Copy, Debug)]
pub(crate) struct StandardMove<PositionRep: Squarey> {
    pub(crate) from_square: PositionRep,
    pub(crate) to_square: PositionRep,
}

// En passant moves affect a third square. Even if I represent actual moves 
// differently later, I need those that parse into en passant moves to make 
// the identity of all three squares readily available to the code that 
// updates the board state. If you find this painful, be glad I'm not having
// these things also store the type of piece being moved! I did consider that,
// but it's extracted from the board instead in the default implementation
// (and probably copied from data already grabbed by some parent function in 
// an efficient version).
#[derive(Clone, Copy, Debug)]
pub(crate) struct EnPassantMove<PositionRep: Squarey> {
    pub(crate) from_square: PositionRep,
    pub(crate) taken_square: PositionRep,
    pub(crate) to_square: PositionRep,
}

// Yes, this is overkill. Counterpoint: however I do eventually represent
// castling moves, I will need to be able to extract all of this information.
// That's the role this type serves here. As a bonus, this makes castling
// compatible with all the weird movesets for free as long as you have a type
// that can handle it. 
#[derive(Clone, Copy, Debug)]
pub(crate) struct CastlingMove<PositionRep: Squarey> {
    pub(crate) king_from: PositionRep,
    pub(crate) rook_from: PositionRep,
    pub(crate) king_to: PositionRep,
    pub(crate) rook_to: PositionRep,
}

// You get the drill. Promotion moves move a pawn to the last rank, and must 
// also commit to a particular type of piece for that pawn to promote to. 
// Movegen doesn't admit promoting to an opponent's piece, but this type 
// could support it for the chess variant where that's possible. 
#[derive(Clone, Copy, Debug)]
pub(crate) struct PromotionMove<PositionRep: Squarey, PieceRep: Piecey> {
    pub(crate) from_square: PositionRep,
    pub(crate) to_square: PositionRep,
    pub(crate) promotion_choice: PieceRep,
}

// Great type. 10 out of good. Would have the inliner 
// cast it into the abyss as often as possible. 
#[derive(Clone, Copy, Debug)]
pub(crate) enum ChessMove<PositionRep: Squarey, PieceRep: Piecey> {
    StandardMove(StandardMove<PositionRep>),
    EnPassantMove(EnPassantMove<PositionRep>),
    CastlingMove(CastlingMove<PositionRep>),
    PromotionMove(PromotionMove<PositionRep, PieceRep>),
    NullMove,
}
