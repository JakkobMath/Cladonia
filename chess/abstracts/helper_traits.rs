
// This is the core of the abstracts module, and contains the traits that 
// control which types can be used to represent various parts of a game of 
// chess. They largely say that things have to implement setters and getters 
// for things made out of the types in the helper_types module. I considered 
// doing lenses for that, and then started trying to justify shoving profunctor 
// optics into Cladonia, but eventually decided that would probably handicap 
// the overload-things-to-make-them-efficient process. 

use super::{helper_types::*, helper_consts::*};

// Pieces have colors, boards have an active color, et cetera. 
// https://www.reddit.com/media?url=https%3A%2F%2Fi.redd.it%2F8nkzqrq6cmia1.jpg 
// I don't think I got that from Reddit originally, but this was the 
// first thing on Google when I went to find it. 
pub(crate) trait Colored {
    fn get_color(&self) -> EnumColor;
    fn set_color(&mut self, color: EnumColor) -> ();
    fn get_opposite_color(&self) -> EnumColor {
        // EnumColor overrides this method to actually do something. 
        // This is just helpful shorthand. 
        self.get_color().get_opposite_color() 
    }
}

// From set and get: pieces or things containing dedicated piece-storing data. 
// The build function demands that the piece-storing data be the only data in the type. 
pub(crate) trait Piecey: Colored + Sized + Copy {
    fn get_piece_type(&self) -> EnumPiecesUncolored;
    fn set_piece_type(&mut self, piece_type: EnumPiecesUncolored) -> () {
        *self = Self::build_piece(self.get_color(), piece_type)
    }
    fn build_piece(piece_color: EnumColor, piece_type: EnumPiecesUncolored) -> Self;
}

// Board squares may or may not contain a piece. Effectively this is the same 
// as the above, data serving as proxy for Option<piece>.
pub(crate) trait Contentsy: Sized + Copy {
    type Content: Piecey;
    fn get_contents(&self) -> Option<Self::Content>;
    #[inline(always)]
    fn set_contents(&mut self, contents: Option<Self::Content>) -> () {
        *self = Self::build_contents(contents)
    }
    fn build_contents(contents: Option<Self::Content>) -> Self;
}

// No build function: this just means *having* rank data. We also need to be 
// able to move ranked things around and figure out distances between them, 
// but those functions can be provided to the user rather than demanded of 
// them. Covariance vs contravariance sort of thing. 
pub(crate) trait Ranked: Sized + Copy {
    fn get_rank(&self) -> EnumRank;
    fn set_rank(&mut self, rank: EnumRank) -> ();
    #[inline(always)]
    fn rank_shift(&self, shift: SmallOffset) -> Option<Self> {
        match self.get_rank().rank_shift(shift) {
            None => None,
            Some(new_rank) => {
                let mut new_thing = *self;
                new_thing.set_rank(new_rank);
                Some(new_thing)
            }
        }
    }
    // The EnumRank type overloads this to do something. 
    #[inline(always)]
    fn rank_gap(&self, other_rank: &Self) -> i8 {
        self.get_rank().rank_gap(&other_rank.get_rank())
    }
}

// Similar to the above. 
pub(crate) trait Filed: Sized + Copy + PartialOrd {
    fn get_file(&self) -> EnumFile;
    fn set_file(&mut self, file: EnumFile) -> ();
    #[inline(always)]
    fn file_shift(&self, shift: SmallOffset) -> Option<Self> {
        match self.get_file().file_shift(shift) {
            None => None,
            Some(new_file) => {
                let mut new_thing = *self;
                new_thing.set_file(new_file);
                Some(new_thing)
            }
        }
    }
    // Overloaded by the EnumFile type. 
    #[inline(always)]
    fn file_gap(&self, other_file: &Self) -> i8 {
        self.get_file().file_gap(&other_file.get_file())
    }
}

// The build function guarantees that this data is *like* a description of a square 
// rather than just containing the description of a square. This is in one sense a 
// proxy for an (EnumRank, EnumFile) pair, but we can use more efficient representations 
// than that. This type also provides a lot of functions that do a lot of the heavy 
// lifting in the movegen code later, such as getting pseudolegal <piece> moves out of 
// the square being described. 
pub(crate) trait Squarey: Ranked + Filed {
    #[inline(always)]
    fn set_square(&mut self, rank: EnumRank, file: EnumFile) -> () {
        self.set_file(file);
        self.set_rank(rank);
    }
    fn build_square(rank: EnumRank, file: EnumFile) -> Self;
    #[inline(always)]
    fn try_get_offset_square(&self, rank_offset: SmallOffset, file_offset: SmallOffset) -> Option<Self> {
        match self.rank_shift(rank_offset) {
            None => None,
            Some(new_square) => new_square.file_shift(file_offset),
        }
    }
    #[inline(always)]
    fn generate_ray(&self, rank_offset: SmallOffset, file_offset: SmallOffset) -> Ray<Self, (SmallOffset, SmallOffset)> {
        Ray {
            curr_pos: *self,
            direction: (rank_offset, file_offset),
        }
    }
    #[inline(always)]
    fn get_offset(&self, other_square: Self) -> (i8, i8) {
        (self.rank_gap(&other_square), self.file_gap(&other_square))
    }
    #[inline(always)]
    fn try_get_ray_to(&self, other_square: Self) -> Option<Ray<Self, (SmallOffset, SmallOffset)>> {
        let (rank_offset, file_offset) = self.get_offset(other_square);
        match rank_offset == 0 {
            true => {
                match file_offset == 0 {
                    true => None,
                    false => {
                        let file_dir = match file_offset > 0 {
                            true => SmallOffset::PlusOne,
                            false => SmallOffset::MinusOne,
                        };
                        Some(Ray {
                            curr_pos: *self,
                            direction: (SmallOffset::Stay, file_dir),
                        })
                    }
                }
            },
            false => {
                match file_offset == 0 {
                    true => {
                        let rank_dir = match rank_offset > 0 {
                            true => SmallOffset::PlusOne,
                            false => SmallOffset::MinusOne,
                        };
                        Some(Ray {
                            curr_pos: *self,
                            direction: (rank_dir, SmallOffset::Stay)
                        })
                    },
                    false => match rank_offset.abs() == file_offset.abs() {
                        true => {
                            let rank_dir = match rank_offset > 0 {
                                true => SmallOffset::PlusOne,
                                false => SmallOffset::MinusOne,
                            };
                            let file_dir = match file_offset > 0 {
                                true => SmallOffset::PlusOne,
                                false => SmallOffset::MinusOne,
                            };
                            Some(Ray {
                                curr_pos: *self,
                                direction: (rank_dir, file_dir),
                            })
                        }
                        false => None,
                    }
                }
            }
        }
    }
    #[inline(always)]
    fn try_get_ray_away(&self, other_square: Self) -> Option<Ray<Self, (SmallOffset, SmallOffset)>> {
        let (rank_offset, file_offset) = self.get_offset(other_square);
        match rank_offset == 0 {
            true => {
                match file_offset == 0 {
                    true => None,
                    false => {
                        let file_dir = match file_offset > 0 {
                            true => SmallOffset::MinusOne,
                            false => SmallOffset::PlusOne,
                        };
                        Some(Ray {
                            curr_pos: *self,
                            direction: (SmallOffset::Stay, file_dir),
                        })
                    }
                }
            },
            false => {
                match file_offset == 0 {
                    true => {
                        let rank_dir = match rank_offset > 0 {
                            true => SmallOffset::MinusOne,
                            false => SmallOffset::PlusOne,
                        };
                        Some(Ray {
                            curr_pos: *self,
                            direction: (rank_dir, SmallOffset::Stay)
                        })
                    },
                    false => match rank_offset.abs() == file_offset.abs() {
                        true => {
                            let rank_dir = match rank_offset > 0 {
                                true => SmallOffset::MinusOne,
                                false => SmallOffset::PlusOne,
                            };
                            let file_dir = match file_offset > 0 {
                                true => SmallOffset::MinusOne,
                                false => SmallOffset::PlusOne,
                            };
                            Some(Ray {
                                curr_pos: *self,
                                direction: (rank_dir, file_dir),
                            })
                        }
                        false => None,
                    }
                }
            }
        }
    }

    fn get_king_offset_squares(&self) -> Vec<Self> {
        let mut offset_squares = Vec::new();
        for offset in PROPER_KING_OFFSETS {
            match self.try_get_offset_square(offset.0, offset.1) {
                None => {},
                Some(offset_square) => offset_squares.push(offset_square)
            }
        }
        offset_squares
    }
    fn get_knight_offset_squares(&self) -> Vec<Self> {
        let mut offset_squares = Vec::new();
        for offset in KNIGHT_OFFSETS {
            match self.try_get_offset_square(offset.0, offset.1) {
                None => {},
                Some(offset_square) => offset_squares.push(offset_square)
            }
        }
        offset_squares
    }
    
    fn get_bishop_rays(&self) -> Vec<Ray<Self, (SmallOffset, SmallOffset)>> {
        let mut rays = Vec::new();
        for direction in BISHOP_DIRECTIONS {
            rays.push(Ray {curr_pos: *self, direction: direction});
        }
        rays
    }
    fn get_rook_rays(&self) -> Vec<Ray<Self, (SmallOffset, SmallOffset)>>{
        let mut rays = Vec::new();
        for direction in ROOK_DIRECTIONS {
            rays.push(Ray {curr_pos: *self, direction: direction});
        }
        rays
    }
    fn get_queen_rays(&self) -> Vec<Ray<Self, (SmallOffset, SmallOffset)>>{
        let mut rays = Vec::new();
        for direction in QUEEN_DIRECTIONS {
            rays.push(Ray {curr_pos: *self, direction: direction});
        }
        rays
    }
}

// Getter and builder make this like a description of a move. Hopefully the conversion to 
// a ChessMove gets inlined away, that seems like it'd be inefficient. On the other hand 
// I think impls_v0 just uses the ChessMove type directly, because it was convenient and 
// making a ChessMove proxy seemed harder. It probably matters less here than for board 
// or game state data because moves should probably be consumed pretty quickly on average 
// and not take up much space. Profiler will tell. 
pub(crate) trait Movey<PositionRep, PieceRep>: Sized + Copy
where PositionRep: Squarey, PieceRep: Piecey,
{
    fn get_move(&self) -> ChessMove<PositionRep, PieceRep>;
    fn set_move(&mut self, new_move: ChessMove<PositionRep, PieceRep>) -> ();
    fn build_move(new_move: ChessMove<PositionRep, PieceRep>) -> Self;
    #[inline(always)]
    fn is_proper(&self) -> bool {
        match self.get_move() {
            ChessMove::NullMove => false,
            _ => true,
        }
    }
}

// The quality of containing boardlike data, together with provided functions for chess things 
// where board state is sufficient (that is, 50mr, castling, et cetera are unnecessary). 
pub(crate) trait HasBoard: Sized + Copy {
    type PositionRep: Squarey;
    type ContentsRep: Contentsy;
    type MoveRep: Movey<Self::PositionRep, <Self::ContentsRep as Contentsy>::Content>;

    const CANONICAL_ARRAY: [Self::PositionRep; 64];

    fn query_square(&self, square: Self::PositionRep) -> Self::ContentsRep;
    fn set_square(&mut self, square: Self::PositionRep, new_contents: Self::ContentsRep) -> ();

    // For doing things like detecting whether the king is in check. 
    fn sees_obvious_attack(&self, defending_color: EnumColor, square: Self::PositionRep) -> bool {

        let reverse_opponent_pawn_move_dir = match defending_color {
            EnumColor::White => SmallOffset::PlusOne,
            EnumColor::Black => SmallOffset::MinusOne,
        };

        for file_movement in [SmallOffset::MinusOne, SmallOffset::PlusOne] {
            match square.try_get_offset_square(reverse_opponent_pawn_move_dir, file_movement) {
                None => {},
                Some(possibly_attacking_square) => {
                    match self.query_square(possibly_attacking_square).get_contents() {
                        None => {},
                        Some(piece) => {
                            if piece.get_color() != defending_color && match piece.get_piece_type() {
                                EnumPiecesUncolored::Pawn => true,
                                EnumPiecesUncolored::Bishop => true,
                                EnumPiecesUncolored::Queen => true,
                                EnumPiecesUncolored::King => true,
                                _ => false,
                            } {
                                return true
                            }
                        }
                    }
                }
            }
        }

        for knight_square in square.get_knight_offset_squares() {
            match self.query_square(knight_square).get_contents() {
                None => {},
                Some(piece) => {
                    if (piece.get_color() != defending_color) && (piece.get_piece_type() == EnumPiecesUncolored::Knight) {
                        return true
                    }
                },
            }
        }

        for bishop_ray in square.get_bishop_rays() {
            let mut king_relevant = true;
            for possibly_attacking_square in bishop_ray {
                match self.query_square(possibly_attacking_square).get_contents() {
                    None => {},
                    Some(piece) => {
                        if piece.get_color() != defending_color && match piece.get_piece_type() {
                            EnumPiecesUncolored::Bishop => true,
                            EnumPiecesUncolored::Queen => true,
                            EnumPiecesUncolored::King => king_relevant,
                            _ => false,
                        } {
                            return true
                        }
                        break;
                    },
                }
                king_relevant = false;
            }
        }

        for rook_ray in square.get_rook_rays() {
            let mut king_relevant = true;
            for possibly_attacking_square in rook_ray {
                match self.query_square(possibly_attacking_square).get_contents() {
                    None => {},
                    Some(piece) => {
                        if piece.get_color() != defending_color && match piece.get_piece_type() {
                            EnumPiecesUncolored::Rook => true,
                            EnumPiecesUncolored::Queen => true,
                            EnumPiecesUncolored::King => king_relevant,
                            _ => false,
                        } {
                            return true
                        }
                        break;
                    },
                }
                king_relevant = false;
            }
        }

        return false
    }
    // For doing things like detecting *why* the king is in check. 
    fn get_obvious_attackers(&self, defending_color: EnumColor, square: Self::PositionRep) -> Vec<Self::MoveRep> {

        let mut attacking_moves = Vec::new();

        let (opponent_pawn_move_dir, opponent_pawn_promotion_rank) = match defending_color {
            EnumColor::White => (SmallOffset::MinusOne, EnumRank::One),
            EnumColor::Black => (SmallOffset::PlusOne, EnumRank::Eight),
        };

        for file_movement in [SmallOffset::MinusOne, SmallOffset::PlusOne] {
            match square.try_get_offset_square(opponent_pawn_move_dir, file_movement) {
                None => {},
                Some(possibly_attacking_square) => {
                    match self.query_square(possibly_attacking_square).get_contents() {
                        None => {},
                        Some(piece) => {
                            if piece.get_color() != defending_color && piece.get_piece_type() == EnumPiecesUncolored::Pawn {
                                if square.get_rank() != opponent_pawn_promotion_rank {
                                    attacking_moves.push(
                                        Self::MoveRep::build_move(
                                            ChessMove::StandardMove(
                                                StandardMove {
                                                    from_square: possibly_attacking_square, 
                                                    to_square: square})))
                                } else {
                                    for promotion_option in [
                                        EnumPiecesUncolored::Queen, 
                                        EnumPiecesUncolored::Knight, 
                                        EnumPiecesUncolored::Rook, 
                                        EnumPiecesUncolored::Bishop
                                        ] {
                                        attacking_moves.push(
                                            Self::MoveRep::build_move(
                                                ChessMove::PromotionMove(
                                                    PromotionMove {
                                                        from_square: possibly_attacking_square, 
                                                        to_square: square, 
                                                        promotion_choice: <Self::ContentsRep as Contentsy>::Content::build_piece(
                                                            defending_color.get_opposite_color(),
                                                            promotion_option)})))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for knight_square in square.get_knight_offset_squares() {
            match self.query_square(knight_square).get_contents() {
                None => {},
                Some(piece) => {
                    if piece.get_color() != defending_color && piece.get_piece_type() == EnumPiecesUncolored::Knight {
                        attacking_moves.push(
                            Self::MoveRep::build_move(
                                ChessMove::StandardMove(
                                    StandardMove {
                                        from_square: knight_square, 
                                        to_square: square})))
                    }
                },
            }
        }

        for bishop_ray in square.get_bishop_rays() {
            let mut king_relevant = true;
            for possibly_attacking_square in bishop_ray {
                match self.query_square(possibly_attacking_square).get_contents() {
                    None => {},
                    Some(piece) => {
                        if piece.get_color() != defending_color && match piece.get_piece_type() {
                            EnumPiecesUncolored::Bishop => true,
                            EnumPiecesUncolored::Queen => true,
                            EnumPiecesUncolored::King => king_relevant,
                            _ => false,
                        } {
                            attacking_moves.push(
                                Self::MoveRep::build_move(
                                    ChessMove::StandardMove(
                                        StandardMove {
                                            from_square: possibly_attacking_square, 
                                            to_square: square})))
                        }
                        break;
                    },
                }
                king_relevant = false;
            }
        }

        for rook_ray in square.get_rook_rays() {
            let mut king_relevant = true;
            for possibly_attacking_square in rook_ray {
                match self.query_square(possibly_attacking_square).get_contents() {
                    None => {},
                    Some(piece) => {
                        if piece.get_color() != defending_color && match piece.get_piece_type() {
                            EnumPiecesUncolored::Rook => true,
                            EnumPiecesUncolored::Queen => true,
                            EnumPiecesUncolored::King => king_relevant,
                            _ => false,
                        } {
                            attacking_moves.push(
                                Self::MoveRep::build_move(
                                    ChessMove::StandardMove(
                                        StandardMove {
                                            from_square: possibly_attacking_square, 
                                            to_square: square})))
                        }
                        break;
                    },
                }
                king_relevant = false;
            }
        }

        attacking_moves
    }
    
    // Freeze any extra FEN-type data (etc) and just update the position. If I 
    // implement custom NNUE code this would probably also affect the accumulators. 
    // This version copies the new position to another place and retains the original, 
    // the one beneath it alters the original in place. 
    #[inline(always)]
    fn after_frozen_move(&self, possible_move: Self::MoveRep) -> Self {
        let mut after_pos = *self;
        after_pos.frozen_make_move(possible_move);
        after_pos
    }
    fn frozen_make_move(&mut self, possible_move: Self::MoveRep) -> () {
        match possible_move.get_move() {
            ChessMove::NullMove => {},
            ChessMove::StandardMove(standard_move) => {
                let thing_moved = self.query_square(standard_move.from_square);
                self.set_square(standard_move.from_square, Self::ContentsRep::build_contents(None));
                self.set_square(standard_move.to_square, thing_moved);
            },
            ChessMove::PromotionMove(promotion_move) => {
                let promoted_content = match self.query_square(promotion_move.from_square).get_contents() {
                    None => None,
                    Some(piece) => {
                        let mut new_piece = piece;
                        new_piece.set_piece_type(promotion_move.promotion_choice.get_piece_type());
                        Some(new_piece)
                    }
                };
                self.set_square(promotion_move.from_square, Self::ContentsRep::build_contents(None));
                self.set_square(promotion_move.to_square, Self::ContentsRep::build_contents(promoted_content));
            },
            ChessMove::EnPassantMove(ep_move) => {
                let thing_moved = self.query_square(ep_move.from_square);
                self.set_square(ep_move.from_square, Self::ContentsRep::build_contents(None));
                self.set_square(ep_move.taken_square, Self::ContentsRep::build_contents(None));
                self.set_square(ep_move.to_square, thing_moved);
            },
            ChessMove::CastlingMove(castling_move) => {
                let rook_moved = self.query_square(castling_move.rook_from);
                let king_moved = self.query_square(castling_move.king_from);
                self.set_square(castling_move.rook_from, Self::ContentsRep::build_contents(None));
                self.set_square(castling_move.king_from, Self::ContentsRep::build_contents(None));
                self.set_square(castling_move.rook_to, rook_moved);
                self.set_square(castling_move.king_to, king_moved);
            }
        }
    }
}

// Set and get method: types implementing this have a ply counter. 
// It should be logically equivalent to the provided one, as usual. 
pub(crate) trait PlyCounting {
    fn get_ply_count(&self) -> i8;
    fn set_ply_count(&mut self, ply_count: i8) -> ();
    #[inline(always)]
    fn reset_ply_counter(&mut self) {
        self.set_ply_count(0)
    }
    #[inline(always)]
    fn increment_ply(&mut self) {
        let new_count = self.get_ply_count().max(49) + 1;
        self.set_ply_count(new_count)
    }
    #[inline(always)]
    fn time_up(&self) -> bool {
        self.get_ply_count() == 50
    }
}

// Similar to the above. The longest possible chess game assuming 
// strict 50mr enforcement falls in the i16 range. 
pub(crate) trait MoveCounting {
    fn get_move_count(&self) -> i16;
    fn set_move_count(&mut self, move_count: i16) -> ();
    #[inline(always)]
    fn increment_move(&mut self) {
        self.set_move_count(self.get_move_count() + 1)
    }
}

// The big deal trait: this contains all the data necessary to extract a FEN. 
// Supports variants with weird castling rules like 960. Types optimized for 
// standard play can have the relevant set methods round to the nearest valid 
// option for implementation purposes, but should probably throw an error if 
// you try to construct an invalid position and avoid using set methods directly. 
pub(crate) trait FENnec: HasBoard + Colored + PlyCounting + MoveCounting + Sized + Copy {
    fn get_castling(&self, color: EnumColor) -> [Option<CastlingMove<Self::PositionRep>>; 2];
    fn set_castling(&mut self, color: EnumColor, new_rules: [Option<CastlingMove<Self::PositionRep>>; 2]) -> ();
    #[inline(always)]
    fn remove_castling(&mut self, moved_square: Self::PositionRep) -> () {
        let mut new_rules = self.get_castling(self.get_color());
        for i in [0, 1] {
            match new_rules[i] {
                None => {},
                Some(castling_move) => {
                    if castling_move.king_from == moved_square || castling_move.rook_from == moved_square {
                        new_rules[i] = None;
                    }
                }
            }
        }
        self.set_castling(self.get_color(), new_rules);
    }
    #[inline(always)]
    fn remove_enemy_castling(&mut self, removed_square: Self::PositionRep) -> () {
        let mut new_rules = self.get_castling(self.get_opposite_color());
        for i in [0, 1] {
            match new_rules[i] {
                None => {},
                Some(castling_move) => {
                    if castling_move.rook_from == removed_square {
                        new_rules[i] = None;
                    }
                }
            }
        }
        self.set_castling(self.get_opposite_color(), new_rules);
    }
    fn get_w_king_square(&self) -> Self::PositionRep;
    fn set_w_king_square(&mut self, square: Self::PositionRep) -> ();
    fn get_b_king_square(&self) -> Self::PositionRep;
    fn set_b_king_square(&mut self, square: Self::PositionRep) -> ();
    fn try_get_ep_square(&self) -> Option<(Self::PositionRep, Self::PositionRep)>;
    fn set_ep_square(&mut self, value: Option<(Self::PositionRep, Self::PositionRep)>) -> ();

    #[inline(always)]
    fn mover_in_check(&self) -> bool {
        let relevant_king_square = match self.get_color() {
            EnumColor::White => self.get_w_king_square(),
            EnumColor::Black => self.get_b_king_square(),
        };
        self.sees_obvious_attack(self.get_color(), relevant_king_square)
    }
    #[inline(always)]
    fn non_mover_in_check(&self) -> bool {
        let relevant_king_square = match self.get_color() {
            EnumColor::White => self.get_b_king_square(),
            EnumColor::Black => self.get_w_king_square(),
        };
        // let debug_help = self.sees_obvious_attack(self.get_opposite_color(), relevant_king_square);
        self.sees_obvious_attack(self.get_opposite_color(), relevant_king_square)
    }
    #[inline(always)]
    fn is_stalemate(&self) -> bool {
        self.get_legal_proper_moves().len() == 0 && !self.mover_in_check()
    }
    #[inline(always)]
    fn is_checkmate(&self) -> bool {
        self.get_legal_proper_moves().len() == 0 && self.mover_in_check()
    }

    fn is_pinned(&self, square: Self::PositionRep) -> bool {
        let king_square = match self.get_color() {
            EnumColor::White => self.get_w_king_square(),
            EnumColor::Black => self.get_b_king_square(),
        };
        match king_square.try_get_ray_to(square) {
            None => false,
            Some(ray) => {
                let ray_piece_type = match ray.direction.0 == SmallOffset::Stay || ray.direction.1 == SmallOffset::Stay {
                    true => EnumPiecesUncolored::Rook,
                    false => EnumPiecesUncolored::Bishop,
                };
                for threatening_square in ray {
                    match self.query_square(threatening_square).get_contents() {
                        None => {},
                        Some(piece) => {
                            if piece.get_color() == self.get_opposite_color() && (piece.get_piece_type() == EnumPiecesUncolored::Queen || piece.get_piece_type() == ray_piece_type) {
                                return true
                            }
                            if threatening_square != square {
                                break
                            }
                        },
                    }
                }
                false
            },
        }
    }
    fn pinning_square(&self, square: Self::PositionRep) -> Option<Self::PositionRep> {
        let king_square = match self.get_color() {
            EnumColor::White => self.get_w_king_square(),
            EnumColor::Black => self.get_b_king_square(),
        };
        match king_square.try_get_ray_to(square) {
            None => None,
            Some(ray) => {
                let ray_piece_type = match ray.direction.0 == SmallOffset::Stay || ray.direction.1 == SmallOffset::Stay {
                    true => EnumPiecesUncolored::Rook,
                    false => EnumPiecesUncolored::Bishop,
                };
                for threatening_square in ray {
                    match self.query_square(threatening_square).get_contents() {
                        None => {},
                        Some(piece) => {
                            if piece.get_color() == self.get_opposite_color() && (piece.get_piece_type() == EnumPiecesUncolored::Queen || piece.get_piece_type() == ray_piece_type) {
                                return Some(threatening_square)
                            }
                            if threatening_square != square {
                                break
                            }
                        },
                    }
                }
                None
            },
        }
    }
    fn check_remaining_legality(&self, possible_move: Self::MoveRep) -> bool {
        match possible_move.get_move() {
            ChessMove::CastlingMove(proposed_move) => {
                if self.mover_in_check() {
                    return false
                };
                match proposed_move.king_from.try_get_ray_to(proposed_move.king_to) {
                    // This could concievably happen in some 960-like variant if the king castles onto its own current square.
                    // Otherwise this should be impossible.
                    None => true,
                    Some(ray) => {
                        for passed_over_square in ray {
                            if self.sees_obvious_attack(self.get_color(), passed_over_square) {
                                return false
                            };
                            if passed_over_square.get_file() == proposed_move.king_to.get_file() {
                                break;
                            }
                        }
                        true
                    }
                }
            },
            _ => {return !self.after_move(possible_move).non_mover_in_check()},
        }
    }
    fn is_forward_progress(&self, possible_move: Self::MoveRep) -> bool {
        match possible_move.get_move() {
            ChessMove::StandardMove(unwrapped_move) => {
                match self.query_square(unwrapped_move.to_square).get_contents() {
                    None => {
                        match self.query_square(unwrapped_move.from_square).get_contents() {
                            None => panic!(),
                            Some(piece) => piece.get_piece_type() == EnumPiecesUncolored::Pawn,
                        }
                    },
                    Some(_piece) => true,
                }
            },
            ChessMove::NullMove => false,
            _ => true,
        }
    }

    fn get_likely_pawn_moves(&self, square: Self::PositionRep) -> Vec<Self::MoveRep> {
        let mut pawn_moves = Vec::new();

        let (double_move_rank, promotion_rank, pawn_move_dir) = match self.get_color() {
            EnumColor::White => (EnumRank::Two, EnumRank::Eight, SmallOffset::PlusOne),
            EnumColor::Black => (EnumRank::Seven, EnumRank::One, SmallOffset::MinusOne),
        };

        // Capture moves.
        for file_offset in [SmallOffset::MinusOne, SmallOffset::PlusOne] {
            match square.try_get_offset_square(pawn_move_dir, file_offset) {
                None => {},
                Some(attacked_square) => {
                    match self.query_square(attacked_square).get_contents() {
                        None => {},
                        Some(attacked_piece) => {
                            if attacked_piece.get_color() == self.get_opposite_color() {
                                if attacked_square.get_rank() == promotion_rank {
                                    for promotion_option in [
                                        EnumPiecesUncolored::Queen,
                                        EnumPiecesUncolored::Knight,
                                        EnumPiecesUncolored::Rook,
                                        EnumPiecesUncolored::Bishop,
                                    ] {
                                        pawn_moves.push(
                                            Self::MoveRep::build_move(
                                                ChessMove::PromotionMove(
                                                    PromotionMove {
                                                        from_square: square, 
                                                        to_square: attacked_square, 
                                                        promotion_choice: <Self::ContentsRep as Contentsy>::Content::build_piece(
                                                            self.get_color(), 
                                                            promotion_option)})))
                                    }
                                } else {
                                    pawn_moves.push(
                                        Self::MoveRep::build_move(
                                            ChessMove::StandardMove(
                                                StandardMove {
                                                    from_square: square, 
                                                    to_square: attacked_square})))
                                }
                            }
                        }
                    }
                }
            }
        }

        // Forward moves.
        match square.try_get_offset_square(pawn_move_dir, SmallOffset::Stay) {
            None => panic!(),
            Some(forward_square) => {
                match self.query_square(forward_square).get_contents() {
                    None => {
                        if forward_square.get_rank() == promotion_rank {
                            for promotion_option in [
                                        EnumPiecesUncolored::Queen,
                                        EnumPiecesUncolored::Knight,
                                        EnumPiecesUncolored::Rook,
                                        EnumPiecesUncolored::Bishop,
                                    ] {
                                        pawn_moves.push(
                                            Self::MoveRep::build_move(
                                                ChessMove::PromotionMove(
                                                    PromotionMove {
                                                        from_square: square, 
                                                        to_square: forward_square, 
                                                        promotion_choice: <Self::ContentsRep as Contentsy>::Content::build_piece(
                                                            self.get_color(), 
                                                            promotion_option)})))
                                    }
                        } else {
                            pawn_moves.push(
                                Self::MoveRep::build_move(
                                    ChessMove::StandardMove(
                                        StandardMove { 
                                            from_square: square, 
                                            to_square: forward_square })));
                        }
                        if square.get_rank() == double_move_rank {
                            match forward_square.try_get_offset_square(pawn_move_dir, SmallOffset::Stay) {
                                None => panic!(),
                                Some(double_forward_square) => {
                                    match self.query_square(double_forward_square).get_contents() {
                                        None => {
                                            pawn_moves.push(
                                                Self::MoveRep::build_move(
                                                    ChessMove::StandardMove(
                                                        StandardMove { 
                                                            from_square: square, 
                                                            to_square: double_forward_square })));
                                        },
                                        Some(_piece) => {},
                                    }
                                }
                            }
                        }
                    },
                    Some(_piece) => {},
                }
            }
        }

        pawn_moves
    }
    fn get_likely_knight_moves(&self, square: Self::PositionRep) -> Vec<Self::MoveRep> {
        let mut knight_moves = Vec::new();

        for to_square in square.get_knight_offset_squares() {
            match self.query_square(to_square).get_contents() {
                None => {
                    knight_moves.push(
                        Self::MoveRep::build_move(
                            ChessMove::StandardMove(
                                StandardMove { 
                                    from_square: square, 
                                    to_square: to_square })))
                },
                Some(piece) => {
                    if piece.get_color() == self.get_opposite_color() {
                        knight_moves.push(
                            Self::MoveRep::build_move(
                                ChessMove::StandardMove(
                                    StandardMove { 
                                        from_square: square, 
                                        to_square: to_square })))
                    }
                }
            }
        }

        knight_moves
    }
    fn get_likely_bishop_moves(&self, square: Self::PositionRep) -> Vec<Self::MoveRep> {
        let mut bishop_moves = Vec::new();

        for ray in square.get_bishop_rays() {
            for attacked_square in ray {
                match self.query_square(attacked_square).get_contents() {
                    None => {
                        bishop_moves.push(
                            Self::MoveRep::build_move(
                                ChessMove::StandardMove(
                                    StandardMove { 
                                        from_square: square, 
                                        to_square: attacked_square })))
                    },
                    Some(piece) => {
                        if piece.get_color() == self.get_opposite_color() {
                            bishop_moves.push(
                                Self::MoveRep::build_move(
                                    ChessMove::StandardMove(
                                        StandardMove { 
                                            from_square: square, 
                                            to_square: attacked_square })))
                        };
                        break;
                    },
                }
            }
        }

        bishop_moves
    }
    fn get_likely_rook_moves(&self, square: Self::PositionRep) -> Vec<Self::MoveRep> {
        let mut rook_moves = Vec::new();

        for ray in square.get_rook_rays() {
            for attacked_square in ray {
                match self.query_square(attacked_square).get_contents() {
                    None => {
                        rook_moves.push(
                            Self::MoveRep::build_move(
                                ChessMove::StandardMove(
                                    StandardMove { 
                                        from_square: square, 
                                        to_square: attacked_square })))
                    },
                    Some(piece) => {
                        if piece.get_color() == self.get_opposite_color() {
                            rook_moves.push(
                                Self::MoveRep::build_move(
                                    ChessMove::StandardMove(
                                        StandardMove { 
                                            from_square: square, 
                                            to_square: attacked_square })))
                        };
                        break;
                    }
                }
            }
        }

        rook_moves
    }
    fn get_likely_queen_moves(&self, square: Self::PositionRep) -> Vec<Self::MoveRep> {
        let mut queen_moves = Vec::new();

        for ray in square.get_queen_rays() {
            for attacked_square in ray {
                match self.query_square(attacked_square).get_contents() {
                    None => {
                        queen_moves.push(
                            Self::MoveRep::build_move(
                                ChessMove::StandardMove(
                                    StandardMove { 
                                        from_square: square, 
                                        to_square: attacked_square })))
                    },
                    Some(piece) => {
                        if piece.get_color() == self.get_opposite_color() {
                            queen_moves.push(
                                Self::MoveRep::build_move(
                                    ChessMove::StandardMove(
                                        StandardMove { 
                                            from_square: square, 
                                            to_square: attacked_square })))
                        };
                        break;
                    }
                }
            }
        }

        queen_moves
    }
    fn get_likely_king_moves(&self, square: Self::PositionRep) -> Vec<Self::MoveRep> {
        let mut king_moves = Vec::new();

        for to_square in square.get_king_offset_squares() {
            match self.query_square(to_square).get_contents() {
                None => {
                    king_moves.push(
                        Self::MoveRep::build_move(
                            ChessMove::StandardMove(
                                StandardMove { 
                                    from_square: square, 
                                    to_square: to_square })))
                },
                Some(piece) => {
                    if piece.get_color() == self.get_opposite_color() {
                        king_moves.push(
                            Self::MoveRep::build_move(
                                ChessMove::StandardMove(
                                    StandardMove { 
                                        from_square: square, 
                                        to_square: to_square })))
                    }
                }
            }
        }

        king_moves
    }
    fn get_likely_ep_moves(&self) -> Vec<Self::MoveRep> {
        match self.try_get_ep_square() {
            None => Vec::new(),
            Some((taken_square, ep_square)) => {
                let mut ep_captures = Vec::new();

                let back_capturing_dir = match self.get_color() {
                    EnumColor::White => SmallOffset::MinusOne,
                    EnumColor::Black => SmallOffset::PlusOne,
                };

                for file_offset in [SmallOffset::MinusOne, SmallOffset::PlusOne] {
                    match ep_square.try_get_offset_square(back_capturing_dir, file_offset) {
                        None => {},
                        Some(threat_square) => {
                            match self.query_square(threat_square).get_contents() {
                                None => {},
                                Some(piece) => {
                                    if piece.get_color() == self.get_color() && piece.get_piece_type() == EnumPiecesUncolored::Pawn {
                                        ep_captures.push(
                                            Self::MoveRep::build_move(
                                                ChessMove::EnPassantMove(
                                                    EnPassantMove { 
                                                        from_square: threat_square, 
                                                        taken_square: taken_square, 
                                                        to_square: ep_square })))
                                    }
                                },
                            }
                        },
                    }
                }

                ep_captures
            },
        }
    }
    fn get_likely_castling_moves(&self) -> Vec<Self::MoveRep> {
        let mut castling_moves = Vec::new();
        for possible_castling_move in self.get_castling(self.get_color()) {
            match possible_castling_move {
                None => {},
                Some(proposed_castling_move) => {
                    match proposed_castling_move.king_from.try_get_ray_to(proposed_castling_move.rook_from) {
                        None => panic!(),
                        Some(ray) => {
                            for square_passed_over in ray {
                                if square_passed_over == proposed_castling_move.rook_from {
                                    castling_moves.push(Self::MoveRep::build_move(ChessMove::CastlingMove(proposed_castling_move)));
                                    break;
                                }
                                match self.query_square(square_passed_over).get_contents() {
                                    None => {},
                                    Some(_piece) => break,
                                }
                            }
                        }
                    }
                },
            }
        }
        castling_moves
    }
    fn get_pseudo_legal_proper_moves(&self) -> Vec<Self::MoveRep> {
        let mut probable_moves = Vec::new();
        for square in Self::CANONICAL_ARRAY {
            match self.query_square(square).get_contents() {
                None => {},
                Some(piece) => {
                    if piece.get_color() == self.get_color() {
                        match piece.get_piece_type() {
                            EnumPiecesUncolored::Pawn => {
                                probable_moves.append(&mut self.get_likely_pawn_moves(square))
                            },
                            EnumPiecesUncolored::Knight => {
                                probable_moves.append(&mut self.get_likely_knight_moves(square))
                            },
                            EnumPiecesUncolored::Bishop => {
                                probable_moves.append(&mut self.get_likely_bishop_moves(square))
                            },
                            EnumPiecesUncolored::Rook => {
                                probable_moves.append(&mut self.get_likely_rook_moves(square))
                            },
                            EnumPiecesUncolored::Queen => {
                                probable_moves.append(&mut self.get_likely_queen_moves(square))
                            },
                            EnumPiecesUncolored::King => {
                                probable_moves.append(&mut self.get_likely_king_moves(square))
                            }
                        }
                    }
                },
            }
        }
        probable_moves.append(&mut self.get_likely_castling_moves());
        probable_moves.append(&mut self.get_likely_ep_moves());
        probable_moves
    }
    
    #[inline(always)]
    fn get_legal_proper_moves(&self) -> Vec<Self::MoveRep> {
        let mut legal_moves = Vec::new();
        for possible_move in self.get_pseudo_legal_proper_moves() {
            if self.check_remaining_legality(possible_move) {
                legal_moves.push(possible_move)
            }
        }
        legal_moves
    }

    #[inline(always)]
    fn after_null_move(&self) -> Self {
        let mut position_after = *self;
        position_after.do_null_move();
        position_after
    }
    #[inline(always)]
    fn do_null_move(&mut self) -> () {
        self.increment_ply();
        if self.get_color() == EnumColor::Black {
            self.increment_move()
        }
        self.set_ep_square(None);
        self.set_color(self.get_opposite_color());
    }
    #[inline(always)]
    fn after_move(&self, possible_move: Self::MoveRep) -> Self {
        let mut position_after = *self;
        position_after.make_move(possible_move);
        position_after
    }
    fn make_move(&mut self, possible_move: Self::MoveRep) -> () {
        if self.is_forward_progress(possible_move) {
            self.reset_ply_counter()
        } else {
            self.increment_ply()
        }
        let mut update_king_square = None;
        match possible_move.get_move() {
            ChessMove::StandardMove(updating_move) => {
                self.remove_castling(updating_move.from_square);
                self.remove_enemy_castling(updating_move.to_square);
                match self.query_square(updating_move.from_square).get_contents() {
                    None => {},
                    Some(piece) => {
                        if updating_move.from_square.rank_gap(&updating_move.to_square).abs() == 2 && piece.get_piece_type() == EnumPiecesUncolored::Pawn {
                            let midpoint_square = match self.get_color() {
                                EnumColor::White => Self::PositionRep::build_square(EnumRank::Three, updating_move.from_square.get_file()),
                                EnumColor::Black => Self::PositionRep::build_square(EnumRank::Six, updating_move.from_square.get_file()),
                            };
                            self.set_ep_square(Some((updating_move.to_square, midpoint_square)))
                        } else {
                            self.set_ep_square(None)
                        }
                        if piece.get_piece_type() == EnumPiecesUncolored::King {
                            update_king_square = Some(updating_move.to_square)
                        }
                    }
                }
            },
            ChessMove::CastlingMove(castling_move) => {
                self.set_ep_square(None);
                self.set_castling(self.get_color(), [None, None]);
                update_king_square = Some(castling_move.king_to);
            },
            ChessMove::PromotionMove(promotion_move) => {
                self.set_ep_square(None);
                self.remove_enemy_castling(promotion_move.to_square);
            }
            _ => self.set_ep_square(None),
        }
        self.frozen_make_move(possible_move);
        match update_king_square {
            None => {},
            Some(new_square) => {
                match self.get_color() {
                    EnumColor::White => self.set_w_king_square(new_square),
                    EnumColor::Black => self.set_b_king_square(new_square),
                }
            },
        }
        self.set_color(self.get_opposite_color());
    }
}
