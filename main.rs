pub(crate) mod chess {
    pub(crate) mod abstracts {
        pub(crate)  mod helper_types {
            #![allow(dead_code)]
            use super::helper_traits::*;

            #[derive(Clone, Copy, PartialEq, Eq, Debug)]
            pub(crate) enum EnumColor {
                White,
                Black,
            }

            #[derive(Clone, Copy, PartialEq, Eq, Debug)]
            pub(crate) enum EnumPiecesUncolored {
                Pawn,
                Knight,
                Bishop,
                Rook,
                Queen,
                King,
            }

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

            #[derive(Clone, Copy, PartialEq, Eq, Debug)]
            pub(crate) enum SmallOffset {
                MinusTwo,
                MinusOne,
                Stay,
                PlusOne,
                PlusTwo,
            }

            #[derive(Debug)]
            pub(crate) struct Ray<Pos, Dir> {
                pub(crate) curr_pos: Pos,
                pub(crate) direction: Dir,
            }

            #[derive(Clone, Copy, Debug)]
            pub(crate) struct StandardMove<PositionRep: Squarey> {
                pub(crate) from_square: PositionRep,
                pub(crate) to_square: PositionRep,
            }

            #[derive(Clone, Copy, Debug)]
            pub(crate) struct EnPassantMove<PositionRep: Squarey> {
                pub(crate) from_square: PositionRep,
                pub(crate) taken_square: PositionRep,
                pub(crate) to_square: PositionRep,
            }

            #[derive(Clone, Copy, Debug)]
            pub(crate) struct CastlingMove<PositionRep: Squarey> {
                pub(crate) king_from: PositionRep,
                pub(crate) rook_from: PositionRep,
                pub(crate) king_to: PositionRep,
                pub(crate) rook_to: PositionRep,
            }

            #[derive(Clone, Copy, Debug)]
            pub(crate) struct PromotionMove<PositionRep: Squarey, PieceRep: Piecey> {
                pub(crate) from_square: PositionRep,
                pub(crate) to_square: PositionRep,
                pub(crate) promotion_choice: PieceRep,
            }

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
            use super::{helper_types::*, helper_consts::*};

            pub(crate) trait Colored {
                fn get_color(&self) -> EnumColor;
                fn set_color(&mut self, color: EnumColor) -> ();
                fn get_opposite_color(&self) -> EnumColor {
                    // EnumColor overrides this method to actually do something. 
                    // This is just helpful shorthand.
                    self.get_color().get_opposite_color() 
                }
            }

            pub(crate) trait Piecey: Colored + Sized + Copy {
                fn get_piece_type(&self) -> EnumPiecesUncolored;
                fn set_piece_type(&mut self, piece_type: EnumPiecesUncolored) -> () {
                    *self = Self::build_piece(self.get_color(), piece_type)
                }
                fn build_piece(piece_color: EnumColor, piece_type: EnumPiecesUncolored) -> Self;
            }

            pub(crate) trait Contentsy: Sized + Copy {
                type Content: Piecey;
                fn get_contents(&self) -> Option<Self::Content>;
                fn set_contents(&mut self, contents: Option<Self::Content>) -> () {
                    *self = Self::build_contents(contents)
                }
                fn build_contents(contents: Option<Self::Content>) -> Self;
            }

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
                fn rank_gap(&self, other_rank: &Self) -> i8 {
                    self.get_rank().rank_gap(&other_rank.get_rank())
                }
            }

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
                fn file_gap(&self, other_file: &Self) -> i8 {
                    self.get_file().file_gap(&other_file.get_file())
                }
            }

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
                        match self.try_get_offset_square(direction.0, direction.1) {
                            None => {},
                            Some(offset_square) => rays.push(Ray {curr_pos: offset_square, direction: direction})
                        }
                    }
                    rays
                }
                fn get_rook_rays(&self) -> Vec<Ray<Self, (SmallOffset, SmallOffset)>>{
                    let mut rays = Vec::new();
                    for direction in ROOK_DIRECTIONS {
                        match self.try_get_offset_square(direction.0, direction.1) {
                            None => {},
                            Some(offset_square) => rays.push(Ray {curr_pos: offset_square, direction: direction})
                        }
                    }
                    rays
                }
                fn get_queen_rays(&self) -> Vec<Ray<Self, (SmallOffset, SmallOffset)>>{
                    let mut rays = Vec::new();
                    for direction in QUEEN_DIRECTIONS {
                        match self.try_get_offset_square(direction.0, direction.1) {
                            None => {},
                            Some(offset_square) => rays.push(Ray {curr_pos: offset_square, direction: direction})
                        }
                    }
                    rays
                }
            }

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

            pub(crate) trait HasBoard: Sized + Copy {
                type PositionRep: Squarey;
                type ContentsRep: Contentsy;
                type MoveRep: Movey<Self::PositionRep, <Self::ContentsRep as Contentsy>::Content>;

                const CANONICAL_ARRAY: [Self::PositionRep; 64];

                fn query_square(&self, square: Self::PositionRep) -> Self::ContentsRep;
                fn set_square(&mut self, square: Self::PositionRep, new_contents: Self::ContentsRep) -> ();
                fn sees_obvious_attack(&self, defending_color: EnumColor, square: Self::PositionRep) -> bool {

                    let opponent_pawn_move_dir = match defending_color {
                        EnumColor::White => SmallOffset::MinusOne,
                        EnumColor::Black => SmallOffset::PlusOne,
                    };

                    for file_movement in [SmallOffset::MinusOne, SmallOffset::PlusOne] {
                        match square.try_get_offset_square(opponent_pawn_move_dir, file_movement) {
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

                    for knight_square in square.get_king_offset_squares() {
                        match self.query_square(knight_square).get_contents() {
                            None => {},
                            Some(piece) => {
                                if piece.get_color() != defending_color && piece.get_piece_type() == EnumPiecesUncolored::Knight {
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

                // fn get_raw_knight_moves(&self, square: Self::PositionRep) -> Vec<Self::MoveRep> {
                //     let mut knight_moves = Vec::new();
                //     for knight_offset in KNIGHT_OFFSETS {
                //         match square.try_get_offset_square(knight_offset.0, knight_offset.1) {
                //             None => {},
                //             Some(new_square) => knight_moves.push(
                //                 Self::MoveRep::build_move(
                //                     ChessMove::StandardMove(
                //                         StandardMove {
                //                             from_square: square,
                //                             to_square: new_square,})))
                //         }
                //     }
                //     knight_moves
                // }
                // fn get_raw_king_moves(&self, square: Self::PositionRep) -> Vec<Self::MoveRep> {
                //     let mut king_moves = Vec::new();
                //     for king_offset in PROPER_KING_OFFSETS {
                //         match square.try_get_offset_square(king_offset.0, king_offset.1) {
                //             None => {},
                //             Some(new_square) => king_moves.push(
                //                 Self::MoveRep::build_move(
                //                     ChessMove::StandardMove(
                //                         StandardMove {
                //                             from_square: square,
                //                             to_square: new_square,})))
                //         }
                //     }
                //     king_moves
                // }
                
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

            pub(crate) trait MoveCounting {
                fn get_move_count(&self) -> i16;
                fn set_move_count(&mut self, move_count: i16) -> ();
                fn increment_move(&mut self) {
                    self.set_move_count(self.get_move_count() + 1)
                }
            }

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
                    self.sees_obvious_attack(self.get_opposite_color(), relevant_king_square)
                }
                fn is_stalemate(&self) -> bool {
                    self.get_legal_improper_moves().len() == 1 && !self.mover_in_check()
                }
                fn is_checkmate(&self) -> bool {
                    self.get_legal_improper_moves().len() == 0
                }

                fn is_pinned(&self, square: Self::PositionRep) -> bool {
                    let king_square = match self.get_color() {
                        EnumColor::White => self.get_w_king_square(),
                        EnumColor::Black => self.get_b_king_square(),
                    };
                    match square.try_get_ray_away(king_square) {
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
                                            return true;
                                        }
                                        break;
                                    }
                                }
                            }
                            false
                        }
                    }
                }
                fn check_remaining_legality(&self, possible_move: Self::MoveRep) -> bool {
                    match possible_move.get_move() {
                        ChessMove::NullMove => true,
                        ChessMove::StandardMove(proposed_move) => {
                            if self.is_pinned(proposed_move.from_square) {
                                return false
                            } else if !self.mover_in_check() {
                                return true
                            } else {
                                return !self.after_move(possible_move).non_mover_in_check()
                            }
                        },
                        ChessMove::PromotionMove(proposed_move) => {
                            if self.is_pinned(proposed_move.from_square) {
                                return false
                            } else if !self.mover_in_check() {
                                return true
                            } else {
                                return !self.after_move(possible_move).non_mover_in_check()
                            }
                        },
                        ChessMove::EnPassantMove(proposed_move) => {
                            // The captured piece also disappears, but we don't have to check whether it's pinned because if it was then
                            // moving it into place the previous ply would have blocked an attack on the now-moving king. This entails
                            // that said king would have been in check while it was the opponent's turn, which is illegal. The only kind
                            // of attack that would be blocked by both the pre-moved opponent pawn and the now-moved pawn is a rook-style
                            // ray along a file... which will now be blocked by the pawn that did en passant.
                            if self.is_pinned(proposed_move.from_square) {
                                return false
                            } else if !self.mover_in_check() {
                                return true
                            } else {
                                return !self.after_move(possible_move).non_mover_in_check()
                            }
                        },
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
                                    pawn_moves.push(
                                        Self::MoveRep::build_move(
                                            ChessMove::StandardMove(
                                                StandardMove { 
                                                    from_square: square, 
                                                    to_square: forward_square })));
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
                                }
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
                fn get_pseudo_legal_improper_moves(&self) -> Vec<Self::MoveRep> {
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
                
                fn get_legal_improper_moves(&self) -> Vec<Self::MoveRep> {
                    let mut legal_moves = Vec::new();
                    for possible_move in self.get_pseudo_legal_improper_moves() {
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
                    match possible_move.get_move() {
                        ChessMove::StandardMove(updating_move) => {
                            self.remove_castling(updating_move.from_square);
                            match self.query_square(updating_move.from_square).get_contents() {
                                None => {},
                                Some(piece) => {
                                    if updating_move.from_square.rank_gap(&updating_move.to_square).abs() == 2 && piece.get_piece_type() == EnumPiecesUncolored::Pawn {
                                        let midpoint_square = match self.get_color() {
                                            EnumColor::White => Self::PositionRep::build_square(EnumRank::Six, updating_move.from_square.get_file()),
                                            EnumColor::Black => Self::PositionRep::build_square(EnumRank::Three, updating_move.from_square.get_file()),
                                        };
                                        self.set_ep_square(Some((updating_move.to_square, midpoint_square)))
                                    }
                                    if piece.get_piece_type() == EnumPiecesUncolored::King {
                                        match piece.get_color() {
                                            EnumColor::White => self.set_w_king_square(updating_move.to_square),
                                            EnumColor::Black => self.set_b_king_square(updating_move.to_square),
                                        }
                                    }
                                }
                            }
                        },
                        ChessMove::CastlingMove(castling_move) => {
                            self.set_ep_square(None);
                            self.set_castling(self.get_color(), [None, None]);
                            match self.get_color() {
                                EnumColor::White => self.set_w_king_square(castling_move.king_to),
                                EnumColor::Black => self.set_b_king_square(castling_move.king_to),
                            }
                        },
                        _ => self.set_ep_square(None),
                    }
                    self.frozen_make_move(possible_move);
                    self.set_color(self.get_opposite_color());
                }
            }
        }

        pub(crate) mod default_implementations {
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
        use super::abstracts::{helper_traits::*, helper_types::*};

        pub(crate) mod impls_v0 {
            use super::*;
            // Colored for i8.
            // Pairing 0 and 1 together, 2 and 3, and so forth. Last bit is color info.
            impl Colored for i8 {
                fn get_color(&self) -> crate::chess::abstracts::helper_types::EnumColor {
                    match self % 2 {
                        0 => EnumColor::White,
                        _ => EnumColor::Black,
                    }
                }
                fn set_color(&mut self, color: EnumColor) -> () {
                    *self = 2 * (*self / 2) + match color {
                        EnumColor::White => 0,
                        EnumColor::Black => 1,
                    }
                }
            }

            // Piecey for i8.
            impl Piecey for i8 {
                fn get_piece_type(&self) -> EnumPiecesUncolored {
                    match self / 2 {
                        0 => EnumPiecesUncolored::Pawn,
                        1 => EnumPiecesUncolored::Knight,
                        2 => EnumPiecesUncolored::Bishop,
                        3 => EnumPiecesUncolored::Rook,
                        4 => EnumPiecesUncolored::Queen,
                        _ => EnumPiecesUncolored::King,
                    }
                }
                fn build_piece(piece_color: EnumColor, piece_type: EnumPiecesUncolored) -> Self {
                    2 * match piece_type {
                        EnumPiecesUncolored::Pawn => 0,
                        EnumPiecesUncolored::Knight => 1,
                        EnumPiecesUncolored::Bishop => 2,
                        EnumPiecesUncolored::Rook => 3,
                        EnumPiecesUncolored::Queen => 4,
                        EnumPiecesUncolored::King => 5,
                    } + match piece_color {
                        EnumColor::White => 0,
                        EnumColor::Black => 1,
                    }
                }
            }

            // Contentsy for i8.
            impl Contentsy for i8 {
                type Content = i8;
                fn get_contents(&self) -> Option<Self::Content> {
                    match *self < 0 {
                        true => None,
                        false => Some(*self),
                    }
                }
                fn build_contents(contents: Option<Self::Content>) -> Self {
                    match contents {
                        None => -1,
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
                    //  6, 0, -1, -1, -1, -1, 1,  7, 
                    //  2, 0, -1, -1, -1, -1, 1,  3, 
                    //  4, 0, -1, -1, -1, -1, 1,  5, 
                    //  8, 0, -1, -1, -1, -1, 1,  9, 
                    // 10, 0, -1, -1, -1, -1, 1, 11, 
                    //  4, 0, -1, -1, -1, -1, 1,  5, 
                    //  2, 0, -1, -1, -1, -1, 1,  3, 
                    //  6, 0, -1, -1, -1, -1, 1,  7, 
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
        }
    }
}

fn main() {
    use chess::{abstracts::helper_traits::*, implementations::impls_v0::*};
    println!("Start position: {:?}", STARTPOS);
    println!("Valid starting moves: {:?}", STARTPOS.get_legal_improper_moves().len());
}