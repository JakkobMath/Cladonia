
// The structure here is as follows: There is a module for chess (I'm open to adding similar modules 
// for other games later) containing code that encodes the rules of chess. The module is divided into 
// a few parts. The abstracts submodule (AND SPECIFICALLY THAT MODULE) extends the ``no magic numbers" 
// principle to ``no magic types." Everything in it should be implemented at the trait level, using 
// traits to encode the condition that compliant types either contain or are equivalent to various 
// unoptimized helper types which correspond directly with the rules of the game as you'd explain them 
// to a human. This allows code like movegen and legality checking to be implemented polymorphically 
// over representations, including representations that support variants like 960 or double 960. 
// Keeping the abstracts section abstract is one of the goals of Cladonia, with the hope being that 
// this will keep things as flexible, modular, and extensible as possible. All optimizations improving 
// the default methods for specific types should take place in the second ``implementations" submodule. 
// I'd like to stress that the implementations submodule specifies the types actually used by the code- 
// everything with the helper types should be inlined out of existence, and mostly serve to make the 
// code more straightforward for me to read and write by allowing pattern matching et cetera. 
// Eventually the chess module will also contain chess-specific code for use by the search, such as 
// evaluations and move ordering. I would like to write the search to be polymorphic over games if 
// possible, and allow games to implement any game-specific optimizations for the search to plug into 
// itself and make use of. I would also like the search to be as polymorphic over evaluation spaces as 
// possible, perhaps only requiring them to implement get_minimum and negation (swap sides) functions 
// but including further optimizations when types also implement, say, the minimal window-querying 
// functionality for alpha-beta to function. 

pub(crate) mod chess {

    // This module deals only with chess. In particular, it provides a trait for types which contain 
    // gamestate information, provides basic pseudolegal movegen code and legality checking code for 
    // types implementing said trait, and gives a particular implementation of that trait together 
    // with a FEN parser. Future work should add gamestate types implementing FENnec which are more 
    // amenable to the kinds of computations Cladonia will be doing and override FENnec methods to 
    // make movegen actually fast. For example, I'll probably use this default movegen to generate 
    // magic bitboards at some point. 

    pub(crate) mod abstracts {

        // This module deals with the abstract rules of chess at the trait level and implements basic 
        // (read: slow) code for things like pseudolegal movegen, legality checking, etc for types 
        // implementing those traits. That code is intended largely to let me experiment quickly with 
        // different implementations of things like gamestate representations without having to 
        // implement and debug movegen et cetera every time. Most of this code should be de facto dead 
        // for normal use once better alternatives exist. Note that this code is also trying to be as 
        // generally applicable as possible. It should, for instance, be 960 (^2) compatible if I did 
        // everything right. "No magic numbers" starts getting turned into "no magic types" here- the 
        // only non-custom types are usize for arrays, i8 to hold gaps between ranks/files and the 
        // 50mr counter, and i16 to hold the move counter (to the best of my recollection). 

        pub(crate)  mod helper_types {

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
        }

        pub(crate) mod helper_consts {

            // This module contains certain constants found in the rules of chess that 
            // are convenient to not have to write down all the time. Pawn directions 
            // would also fit in here, but I put them directly in the movegen code 
            // instead. 

            #![allow(dead_code)]
            use super::helper_types::*;

            pub(crate) const PROPER_KING_OFFSETS: [(SmallOffset, SmallOffset); 8] = [
                (SmallOffset::Stay, SmallOffset::MinusOne),
                (SmallOffset::PlusOne, SmallOffset::MinusOne),
                (SmallOffset::PlusOne, SmallOffset::Stay),
                (SmallOffset::PlusOne, SmallOffset::PlusOne),
                (SmallOffset::Stay, SmallOffset::PlusOne),
                (SmallOffset::MinusOne, SmallOffset::PlusOne),
                (SmallOffset::MinusOne, SmallOffset::Stay),
                (SmallOffset::MinusOne, SmallOffset::MinusOne),
            ];

            pub(crate) const KNIGHT_OFFSETS: [(SmallOffset, SmallOffset); 8] = [
                (SmallOffset::MinusOne, SmallOffset::PlusTwo),
                (SmallOffset::PlusOne, SmallOffset::PlusTwo),
                (SmallOffset::PlusTwo, SmallOffset::PlusOne),
                (SmallOffset::PlusTwo, SmallOffset::MinusOne),
                (SmallOffset::PlusOne, SmallOffset::MinusTwo),
                (SmallOffset::MinusOne, SmallOffset::MinusTwo),
                (SmallOffset::MinusTwo, SmallOffset::MinusOne),
                (SmallOffset::MinusTwo, SmallOffset::PlusOne),
            ];

            pub(crate) const BISHOP_DIRECTIONS: [(SmallOffset, SmallOffset); 4] = [
                (SmallOffset::PlusOne, SmallOffset::MinusOne),
                (SmallOffset::PlusOne, SmallOffset::PlusOne),
                (SmallOffset::MinusOne, SmallOffset::PlusOne),
                (SmallOffset::MinusOne, SmallOffset::MinusOne),
            ];

            pub(crate) const ROOK_DIRECTIONS:[(SmallOffset, SmallOffset); 4] = [
                (SmallOffset::Stay, SmallOffset::MinusOne),
                (SmallOffset::PlusOne, SmallOffset::Stay),
                (SmallOffset::Stay, SmallOffset::PlusOne),
                (SmallOffset::MinusOne, SmallOffset::Stay),
            ];

            pub(crate) const QUEEN_DIRECTIONS: [(SmallOffset, SmallOffset); 8] = PROPER_KING_OFFSETS;
        }

        pub(crate) mod helper_traits {

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
                fn rank_gap(&self, other_rank: &Self) -> i8 {
                    self.get_rank().rank_gap(&other_rank.get_rank())
                }
            }

            // Similar to the above. 
            pub(crate) trait Filed: Sized + Copy + PartialOrd {
                fn get_file(&self) -> EnumFile;
                fn set_file(&mut self, file: EnumFile) -> ();
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
                fn set_square(&mut self, rank: EnumRank, file: EnumFile) -> () {
                    self.set_file(file);
                    self.set_rank(rank);
                }
                fn build_square(rank: EnumRank, file: EnumFile) -> Self;
                fn try_get_offset_square(&self, rank_offset: SmallOffset, file_offset: SmallOffset) -> Option<Self> {
                    match self.rank_shift(rank_offset) {
                        None => None,
                        Some(new_square) => new_square.file_shift(file_offset),
                    }
                }
                fn generate_ray(&self, rank_offset: SmallOffset, file_offset: SmallOffset) -> Ray<Self, (SmallOffset, SmallOffset)> {
                    Ray {
                        curr_pos: *self,
                        direction: (rank_offset, file_offset),
                    }
                }
                fn get_offset(&self, other_square: Self) -> (i8, i8) {
                    (self.rank_gap(&other_square), self.file_gap(&other_square))
                }
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
                fn reset_ply_counter(&mut self) {
                    self.set_ply_count(0)
                }
                fn increment_ply(&mut self) {
                    let new_count = self.get_ply_count().max(49) + 1;
                    self.set_ply_count(new_count)
                }
                fn time_up(&self) -> bool {
                    self.get_ply_count() == 50
                }
            }

            // Similar to the above. The longest possible chess game assuming 
            // strict 50mr enforcement falls in the i16 range. 
            pub(crate) trait MoveCounting {
                fn get_move_count(&self) -> i16;
                fn set_move_count(&mut self, move_count: i16) -> ();
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

                fn mover_in_check(&self) -> bool {
                    let relevant_king_square = match self.get_color() {
                        EnumColor::White => self.get_w_king_square(),
                        EnumColor::Black => self.get_b_king_square(),
                    };
                    self.sees_obvious_attack(self.get_color(), relevant_king_square)
                }
                fn non_mover_in_check(&self) -> bool {
                    let relevant_king_square = match self.get_color() {
                        EnumColor::White => self.get_b_king_square(),
                        EnumColor::Black => self.get_w_king_square(),
                    };
                    // let debug_help = self.sees_obvious_attack(self.get_opposite_color(), relevant_king_square);
                    self.sees_obvious_attack(self.get_opposite_color(), relevant_king_square)
                }
                fn is_stalemate(&self) -> bool {
                    self.get_legal_proper_moves().len() == 1 && !self.mover_in_check()
                }
                fn is_checkmate(&self) -> bool {
                    self.get_legal_proper_moves().len() == 0
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
                
                fn get_legal_proper_moves(&self) -> Vec<Self::MoveRep> {
                    let mut legal_moves = Vec::new();
                    for possible_move in self.get_pseudo_legal_proper_moves() {
                        if self.check_remaining_legality(possible_move) {
                            legal_moves.push(possible_move)
                        }
                    }
                    legal_moves
                }

                fn after_null_move(&self) -> Self {
                    let mut position_after = *self;
                    position_after.do_null_move();
                    position_after
                }
                fn do_null_move(&mut self) -> () {
                    self.increment_ply();
                    if self.get_color() == EnumColor::Black {
                        self.increment_move()
                    }
                    self.set_ep_square(None);
                    self.set_color(self.get_opposite_color());
                }
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
        }

        pub(crate) mod default_implementations {

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
        }
    }

    pub(crate) mod implementations {

        // This is where actual implementations of the traits in the above module go. 
        // The types here allow you to actually store chess-related data and call 
        // chess-related functions, and later impls_version submodules are expected 
        // to become progressively better at this. 

        use super::abstracts::{helper_traits::*, helper_types::*};

        pub(crate) mod impls_v0 {

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
        }
    }
}

pub(crate) mod search {
    pub(crate) mod eval_abstracts {
        use std::ops::Neg;
        pub(crate) trait NegamaxCompatible: Ord + Neg {}
        pub(crate) trait ABCompatible: Ord + Neg {
            type WindowParams: Default;
            type Window: Neg;
            fn window_about(&self, params: Self::WindowParams) -> Self::Window;
            fn in_window(&self, window: Self::Window) -> bool;
        }
    }
}


fn main() {
    use chess::{abstracts::{helper_traits::*, helper_types::*}, implementations::impls_v0::*};

    let trying_startpos_perft = true;
    let first_test = false;
    let second_test = false;
    let third_test = false;
    let fourth_test = false;

    let testing_fen_builder = false;

    let kiwipete_string = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string();
    let trying_kiwipete_perft = true;
    let first_test_pos_two = false;
    let second_test_pos_two = false;
    let third_test_pos_two = false;

    let pos_3_string = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string();
    let trying_pos_3_perft = true;

    let pos_4_string = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string();
    let trying_pos_4_perft = true;

    let pos_5_string = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string();
    let trying_pos_5_perft = true;

    let pos_6_string = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".to_string();
    let trying_pos_6_perft = true;

    if trying_startpos_perft {
        println!("Perft from STARTPOS:");
        let (total_num, sub_perfts) = depth_n_better_perft(STARTPOS, 5);
        println!("Total: {}", total_num);
        for (move_made, successors_num) in sub_perfts {
            println!("{0} - {1}", move_made, successors_num)
        }
    
        // Initial problem was that en passant squares were being created on the wrong side of the board. 
        // Thus A2A4 could be followed up by B7A6, taking on the now-empty A2 square.
    } else if first_test {
        let mut curr_pos = STARTPOS;
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::A), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Four, EnumFile::A),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::A), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Six, EnumFile::A),
                }
            )
        );
        println!("Perft from offender:");
        let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
        println!("Total: {}", total_num);
        for (move_made, successors_num) in sub_perfts {
            println!("{0} - {1}", move_made, successors_num)
        }

        // Problem was that en passant squares were sticking around for more than one ply, so after A2A4 and A7A6, 
        // white could capture their own pawn via en passant per B2A3. The ep-resetting thing was missing from 
        // the StandardMove branch.
    } else if second_test {
        let mut curr_pos = STARTPOS;
        println!("Startpos initialized");
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::One, EnumFile::B), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::C),
                }
            )
        );
        println!("First move made, B1C3");
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::E), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Six, EnumFile::E),
                }
            )
        );
        println!("Second move made, E7E6");
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::C), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::D),
                }
            )
        );
        println!("Third move made, C3D5");
        println!("Perft from this, the offending position:");
        let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
        println!("Total: {}", total_num);
        for (move_made, successors_num) in sub_perfts {
            println!("{0} - {1}", move_made, successors_num)
        }
        println!("Legality check: {}", curr_pos.check_remaining_legality(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Eight, EnumFile::E), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::E),
                }
            )
        ));
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Eight, EnumFile::E), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::E),
                }
            )
        );
        println!("Fourth (problem) move made.");
        println!("Black king square: {}", standardize(curr_pos.get_b_king_square()).to_string())
        // The extra problem move is E8E7, which would be the black king going into check.

        // The reason for this problem was that I was checking for attacking knights at a 
        // king's move away from the target square, not a knight's move away.
    } else if third_test {
        let mut curr_pos = STARTPOS;
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::C), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::C),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::B), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::B),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::One, EnumFile::D), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Four, EnumFile::A),
                }
            )
        );
        println!("Perft from offender:");
        let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
        println!("Total: {}", total_num);
        for (move_made, successors_num) in sub_perfts {
            println!("{0} - {1}", move_made, successors_num)
        }
        
        // In this position, the pawn on B5 flasely claims to have no legal moves. Presumably this is a bug both: 
        // 1) with pin-checking logic, where the pin-checker misses the D7 pawn,
        // 2) with pin-checking logic, where pieces are falsely disallowed from taking the attackers that put them into pins.

        // Fixed: stopped trying to do fancy stuff with pins, the logic was hard to work out. 
        // That's an optimization for later, and should be tested in case cache witchcraft makes it slow.
        // Perft now correct to depth 4.
    } else if fourth_test {
        let mut curr_pos = STARTPOS;
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::D), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::D),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::B), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::B),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::One, EnumFile::E), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::D),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::B), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Four, EnumFile::B),
                }
            )
        );
        println!("Perft from offender:");
        let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
        println!("Total: {}", total_num);
        for (move_made, successors_num) in sub_perfts {
            println!("{0} - {1}", move_made, successors_num)
        }

        // The problem is D2C3 in this position.

        // The function sees_obvious_attack was checking for opponent pawns a *forward* opponent-pawn-attacking-move away
        // from the square being checked. It now checks for opponent pawns *backwards* from the square being queried, so
        // that detected pawns will actually be threatening the relevant square rather than one two squares behind it.

        // Perft totals are now correct to depth 5. I haven't checked each of the 20 start move totals, but the first several
        // at least should still be correct from when they were checked to find the line in this test.
    } else if testing_fen_builder {
        println!("Hopefully the parser correctly interprets the startpos fen... Here it is: \n{:?}", interpret_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()));
        println!("Startpos for comparison: \nOk({:?})", STARTPOS);
    } 
    if trying_kiwipete_perft {
        println!("Perft from Kiwipete:");
        let try_kiwipete_pos = interpret_fen(kiwipete_string);
        match try_kiwipete_pos {
            Err(some_error) => println!("Error with parsing Kiwipete: {}", some_error),
            Ok(kiwipete) => {
                let (total_num, sub_perfts) = depth_n_better_perft(kiwipete, 4);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }
            }
        }
    } else if first_test_pos_two {
        println!("Perft from Kiwipete:");
        let try_kiwipete_pos = interpret_fen(kiwipete_string);
        match try_kiwipete_pos {
            Err(some_error) => println!("Error with parsing Kiwipete: {}", some_error),
            Ok(kiwipete) => {
                let mut curr_pos = kiwipete;
                println!("Kiwipete initialized.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::One, EnumFile::A), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::One, EnumFile::B),
                        }
                    )
                );
                println!("First move made, A1B1.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::H), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::G),
                        }
                    )
                );
                println!("Second move made, H3G2.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::E), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Six, EnumFile::C),
                        }
                    )
                );
                println!("Third move made, E5C6.");
                println!("Perft from offender:");
                let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }

                println!("Checking whether the problem is with legality checks: {}", !curr_pos.check_remaining_legality(ChessMove::PromotionMove(PromotionMove {
                    from_square: i8::build_square(EnumRank::Two, EnumFile::G),
                    to_square: i8::build_square(EnumRank::One, EnumFile::G),
                    promotion_choice: i8::build_piece(EnumColor::Black, EnumPiecesUncolored::Knight)
                })));

                for constructed_move in curr_pos.get_pseudo_legal_proper_moves() {
                    println!("Detected move {}", constructed_move);
                }

                // The G2G1 promotion moves are not detected.
                // ... But G2G1 (NOT promotion) IS constructed by pseudolegal movegen.
            },
        }
    } else if second_test_pos_two {
        println!("Perft from Kiwipete:");
        let try_kiwipete_pos = interpret_fen(kiwipete_string);
        match try_kiwipete_pos {
            Err(some_error) => println!("Error with parsing Kiwipete: {}", some_error),
            Ok(kiwipete) => {
                let mut curr_pos = kiwipete;
                println!("Kiwipete initialized.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::E), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Six, EnumFile::G),
                        }
                    )
                );
                println!("First move made, E5G6.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::H), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::G),
                        }
                    )
                );
                println!("Second move made, H3G2.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Six, EnumFile::G), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Eight, EnumFile::H),
                        }
                    )
                );
                println!("Third move made, G6H8.");
                println!("Perft from offender:");
                let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }

                // Problem is that Cladonia thinks that black can legally castle kingside in this position.
                // The knight has taken the rook, so this should be impossible, but castling removal doesn't
                // trigger on the taken squares as-is. 

                // Added in castling removal from the target (``to") squares of standard and promotion moves.
            }
        }
    } else if third_test_pos_two {
        println!("Perft from Kiwipete:");
        let try_kiwipete_pos = interpret_fen(kiwipete_string);
        match try_kiwipete_pos {
            Err(some_error) => println!("Error with parsing Kiwipete: {}", some_error),
            Ok(kiwipete) => {
                let mut curr_pos = kiwipete;
                println!("Kiwipete initialized.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::E), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::F),
                        }
                    )
                );
                println!("First move made, E5F7.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::H), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::G),
                        }
                    )
                );
                println!("Second move made, H3G2.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::F), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Eight, EnumFile::H),
                        }
                    )
                );
                println!("Second move made, F7H8.");
                println!("Perft from offender:");
                let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }

                // Problem is that Cladonia thinks that black can legally castle kingside in this position. Again.
                // I thought I just fixed this?

                // Oh. I removed the castling for the wrong color. Here comes another helper function.

                // Fixed. Kiwipete perft total is now correct to depth four. Guess I just didn't check whether I'd actually 
                // fixed the offending position with the last change.
            }
        }
    }
    if trying_pos_3_perft {
        println!("Perft from Position 3:");
        let try_get_pos = interpret_fen(pos_3_string);
        match try_get_pos {
            Err(some_error) => println!("Error with parsing Position 3: {}", some_error),
            Ok(pos_3) => {
                let (total_num, sub_perfts) = depth_n_better_perft(pos_3, 4);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }
            }
        }

        // Initially didn't parse because castling rules handler didn't know that '-' 
        // in the castling spot should be interpreted as ``no castling."

        // After fixing the parser, perft is correct to depth 4 right off the bat.
    }
    if trying_pos_4_perft {
        println!("Perft from Position 4:");
        let try_get_pos = interpret_fen(pos_4_string);
        match try_get_pos {
            Err(some_error) => println!("Error with parsing Position 4: {}", some_error),
            Ok(pos_4) => {
                let (total_num, sub_perfts) = depth_n_better_perft(pos_4, 4);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }
            }
        }

        // Correct from the get-go to depth 4.
    }
    if trying_pos_5_perft {
        println!("Perft from Position 5:");
        let try_get_pos = interpret_fen(pos_5_string);
        match try_get_pos {
            Err(some_error) => println!("Error with parsing Position 5: {}", some_error),
            Ok(pos_5) => {
                let (total_num, sub_perfts) = depth_n_better_perft(pos_5, 4);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }
            }
        }

        // No issues at depth 4.
    }
    if trying_pos_6_perft {
        println!("Perft from Position 6:");
        let try_get_pos = interpret_fen(pos_6_string);
        match try_get_pos {
            Err(some_error) => println!("Error with parsing Position 6: {}", some_error),
            Ok(pos_6) => {
                let (total_num, sub_perfts) = depth_n_better_perft(pos_6, 4);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }
            }
        }

        // No issues at depth 4 :).
    }

    // Perft for all tested positions gives correct totals up to depth 4, and startpos is correct to depth 5. 
}
