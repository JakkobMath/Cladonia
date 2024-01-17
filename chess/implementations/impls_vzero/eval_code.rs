// Some very basic search/evaluator code. Probably this file gets broken up 
// into multiple modules and then multiple files. 

// MVVLVA (most valuable victim, least valuable attacker) is used to get a 
// basic move ordering to work for an AB search. 

use crate::chess::abstracts::{helper_types::*, helper_traits::*};
use super::board_rep::*;

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

// Pretty much the most naive evaluation short of literally just guessing. 
// material difference in centipawns, bishop = 3.1. Not even any king
// safety stuff yet, just get any kind of search working first. 
#[allow(dead_code)]
fn naive_evaluation_stm(position: &UnwrappedFen) -> i16 {
    let mut sum = 0;
    for square_index in 0..64 {
        match position.board[square_index].get_contents() {
            None => {},
            Some(piece) => {
                sum += get_piece_value(piece.get_piece_type()) * match piece.get_color() == position.get_color() {
                    false => -1,
                    true => 1,
                }
            },
        }
    }
    sum
}

// TODO: implement Evaluator etc for that evaluation. 

// :crabgrab: took this from https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function on 
// advice of the chess programming discord. Will make my own later, only using these values as 
// a stand-in so I can properly iterate on the search design. I plan to produce no training data 
// from these tables. 

// Indexing differently <=> swapping black and white, makes this easier to copy. 
const PESTO_MG_PAWN_B: [i16; 64] = [
    0,   0,   0,   0,   0,   0,  0,   0, 
    98, 134,  61,  95,  68, 126, 34, -11, 
    -6,   7,  26,  31,  65,  56, 25, -20, 
    -14,  13,   6,  21,  23,  12, 17, -23, 
    -27,  -2,  -5,  12,  17,   6, 10, -25, 
    -26,  -4,  -4, -10,   3,   3, 33, -12, 
    -35,  -1, -20, -23, -15,  24, 38, -22, 
    0,   0,   0,   0,   0,   0,  0,   0, 
];

const PESTO_EG_PAWN_B: [i16; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0, 
    178, 173, 158, 134, 147, 132, 165, 187, 
    94, 100,  85,  67,  56,  53,  82,  84, 
    32,  24,  13,   5,  -2,   4,  17,  17, 
    13,   9,  -3,  -7,  -7,  -8,   3,  -1, 
    4,   7,  -6,   1,   0,  -5,  -1,  -8, 
    13,   8,   8,  10,  13,   0,   2,  -7, 
    0,   0,   0,   0,   0,   0,   0,   0, 
];

const PESTO_MG_KNIGHT_B: [i16; 64] = [
    -167, -89, -34, -49,  61, -97, -15, -107, 
    -73, -41,  72,  36,  23,  62,   7,  -17, 
    -47,  60,  37,  65,  84, 129,  73,   44, 
    -9,  17,  19,  53,  37,  69,  18,   22, 
    -13,   4,  16,  13,  28,  19,  21,   -8, 
    -23,  -9,  12,  10,  19,  17,  25,  -16, 
    -29, -53, -12,  -3,  -1,  18, -14,  -19, 
    -105, -21, -58, -33, -17, -28, -19,  -23, 
];

const PESTO_EG_KNIGHT_B: [i16; 64] = [
    -58, -38, -13, -28, -31, -27, -63, -99, 
    -25,  -8, -25,  -2,  -9, -25, -24, -52, 
    -24, -20,  10,   9,  -1,  -9, -19, -41, 
    -17,   3,  22,  22,  22,  11,   8, -18, 
    -18,  -6,  16,  25,  16,  17,   4, -18, 
    -23,  -3,  -1,  15,  10,  -3, -20, -22, 
    -42, -20, -10,  -5,  -2, -20, -23, -44, 
    -29, -51, -23, -15, -22, -18, -50, -64, 
];

const PESTO_MG_BISHOP_B: [i16; 64] = [
    -29,   4, -82, -37, -25, -42,   7,  -8, 
    -26,  16, -18, -13,  30,  59,  18, -47, 
    -16,  37,  43,  40,  35,  50,  37,  -2, 
    -4,   5,  19,  50,  37,  37,   7,  -2, 
    -6,  13,  13,  26,  34,  12,  10,   4, 
    0,  15,  15,  15,  14,  27,  18,  10, 
    4,  15,  16,   0,   7,  21,  33,   1, 
    -33,  -3, -14, -21, -13, -12, -39, -21, 
];

const PESTO_EG_BISHOP_B: [i16; 64] = [
    -14, -21, -11,  -8, -7,  -9, -17, -24, 
    -8,  -4,   7, -12, -3, -13,  -4, -14, 
    2,  -8,   0,  -1, -2,   6,   0,   4, 
    -3,   9,  12,   9, 14,  10,   3,   2, 
    -6,   3,  13,  19,  7,  10,  -3,  -9, 
    -12,  -3,   8,  10, 13,   3,  -7, -15, 
    -14, -18,  -7,  -1,  4,  -9, -15, -27, 
    -23,  -9, -23,  -5, -9, -16,  -5, -17, 
];

const PESTO_MG_ROOK_B: [i16; 64] = [
    32,  42,  32,  51, 63,  9,  31,  43, 
    27,  32,  58,  62, 80, 67,  26,  44, 
    -5,  19,  26,  36, 17, 45,  61,  16, 
-24, -11,   7,  26, 24, 35,  -8, -20, 
-36, -26, -12,  -1,  9, -7,   6, -23, 
-45, -25, -16, -17,  3,  0,  -5, -33, 
-44, -16, -20,  -9, -1, 11,  -6, -71, 
-19, -13,   1,  17, 16,  7, -37, -26, 
];

const PESTO_EG_ROOK_B: [i16; 64] = [
    13, 10, 18, 15, 12,  12,   8,   5, 
    11, 13, 13, 11, -3,   3,   8,   3, 
    7,  7,  7,  5,  4,  -3,  -5,  -3, 
    4,  3, 13,  1,  2,   1,  -1,   2, 
    3,  5,  8,  4, -5,  -6,  -8, -11, 
    -4,  0, -5, -1, -7, -12,  -8, -16, 
    -6, -6,  0,  2, -9,  -9, -11,  -3, 
    -9,  2,  3, -1, -5, -13,   4, -20, 
];

const PESTO_MG_QUEEN_B: [i16; 64] = [
    -28,   0,  29,  12,  59,  44,  43,  45, 
    -24, -39,  -5,   1, -16,  57,  28,  54, 
    -13, -17,   7,   8,  29,  56,  47,  57, 
    -27, -27, -16, -16,  -1,  17,  -2,   1, 
    -9, -26,  -9, -10,  -2,  -4,   3,  -3, 
    -14,   2, -11,  -2,  -5,   2,  14,   5, 
    -35,  -8,  11,   2,   8,  15,  -3,   1, 
    -1, -18,  -9,  10, -15, -25, -31, -50, 
];

const PESTO_EG_QUEEN_B: [i16; 64] = [
    -9,  22,  22,  27,  27,  19,  10,  20, 
-17,  20,  32,  41,  58,  25,  30,   0, 
-20,   6,   9,  49,  47,  35,  19,   9, 
    3,  22,  24,  45,  57,  40,  57,  36, 
-18,  28,  19,  47,  31,  34,  39,  23, 
-16, -27,  15,   6,   9,  17,  10,   5, 
-22, -23, -30, -16, -16, -23, -36, -32, 
-33, -28, -22, -43,  -5, -32, -20, -41, 
];

const PESTO_MG_KING_B: [i16; 64] = [
    -65,  23,  16, -15, -56, -34,   2,  13, 
    29,  -1, -20,  -7,  -8,  -4, -38, -29, 
    -9,  24,   2, -16, -20,   6,  22, -22, 
    -17, -20, -12, -27, -30, -25, -14, -36, 
    -49,  -1, -27, -39, -46, -44, -33, -51, 
    -14, -14, -22, -46, -44, -30, -15, -27, 
    1,   7,  -8, -64, -43, -16,   9,   8, 
    -15,  36,  12, -54,   8, -28,  24,  14, 
];

const PESTO_EG_KING_B: [i16; 64] = [
    -74, -35, -18, -18, -11,  15,   4, -17, 
    -12,  17,  14,  17,  17,  38,  23,  11, 
    10,  17,  23,  15,  20,  45,  44,  13, 
    -8,  22,  24,  27,  26,  33,  26,   3, 
    -18,  -4,  21,  24,  27,  23,   9, -11, 
    -19,  -3,  11,  21,  23,  16,   7,  -9, 
    -27, -11,   4,  13,  14,   4,  -5, -17, 
    -53, -34, -21, -11, -28, -14, -24, -43, 
];

const PESTO_MG_COMBINED_B: [[i16; 64]; 6] = [
    PESTO_MG_PAWN_B, 
    PESTO_MG_KNIGHT_B, 
    PESTO_MG_BISHOP_B, 
    PESTO_MG_ROOK_B, 
    PESTO_MG_QUEEN_B, 
    PESTO_MG_KING_B, 
];

const PESTO_EG_COMBINED_B: [[i16; 64]; 6] = [
    PESTO_EG_PAWN_B, 
    PESTO_EG_KNIGHT_B, 
    PESTO_EG_BISHOP_B, 
    PESTO_EG_ROOK_B, 
    PESTO_EG_QUEEN_B, 
    PESTO_EG_KING_B, 
];

const PESTO_GAME_PHASE_ADDER: [i32; 6] = [0, 1, 1, 2, 4, 0];

const PESTO_DEFAULT_MG_VALUES: [i32; 6] = [82, 337, 365, 477, 1025, 0];
const PESTO_DEFAULT_EG_VALUES: [i32; 6] = [94, 281, 297, 512,  936, 0];

fn pesto_evalutation_stm(position: &UnwrappedFen) -> i32 {
    let mut mg_value = 0;
    let mut eg_value = 0;
    let mut game_phase = 0;

    for square_index in 0..64i8 {
        match position.board[square_index as usize].get_contents() {
            None => {},
            Some(piece) => {
                let piece_number = match piece.get_piece_type() {
                    EnumPiecesUncolored::Pawn => 0,
                    EnumPiecesUncolored::Knight => 1, 
                    EnumPiecesUncolored::Bishop => 2, 
                    EnumPiecesUncolored::Rook => 3, 
                    EnumPiecesUncolored::Queen => 4,
                    EnumPiecesUncolored::King => 5, 
                };
                let mg_piece_square_value = match piece.get_color() {
                    EnumColor::White => PESTO_MG_COMBINED_B[piece_number][vertical_flip_index(square_index) as usize],
                    EnumColor::Black => PESTO_MG_COMBINED_B[piece_number][square_index as usize],
                } as i32 + PESTO_DEFAULT_MG_VALUES[piece_number];
                let eg_piece_square_value = match piece.get_color() {
                    EnumColor::White => PESTO_EG_COMBINED_B[piece_number][vertical_flip_index(square_index) as usize],
                    EnumColor::Black => PESTO_EG_COMBINED_B[piece_number][square_index as usize],
                } as i32 + PESTO_DEFAULT_EG_VALUES[piece_number];
                let stm_multiplier = match position.get_color() == piece.get_color() {
                    true => 1,
                    false => -1,
                };
                mg_value += mg_piece_square_value * stm_multiplier;
                eg_value += eg_piece_square_value * stm_multiplier;
                game_phase += PESTO_GAME_PHASE_ADDER[piece_number];
            },
        }
    }
    
    // Endpoint evals set up, now taper them to get the output eval. 

    let mg_multiplier = game_phase.min(24);
    let eg_multiplier = 24i32 - mg_multiplier;

    (mg_value * mg_multiplier + eg_value * eg_multiplier) / 24
}

// TODO: implement Evaluator etc for that evaluation. 

pub(crate) fn mvv_lva_sort(position: &UnwrappedFen, moves_list: &mut Vec<<UnwrappedFen as HasBoard>::MoveRep>) {
    moves_list.sort_by(|move_to_make, other_move| mvv_lva_score(position, *move_to_make).cmp(&mvv_lva_score(position, *other_move)));
    moves_list.retain_mut(|possible_move| position.check_remaining_legality(*possible_move));

    // Introducing this temporary fix for performance reasons.
    moves_list.truncate(12)
}

// Higher score -> put move earlier. 
// Not doing anything fancy yet- basically just raw naive MVV-LVA with some special cases. 
// Might add in a forward movement bonus later or something. ... maybe just adding it now. 
pub(crate) fn mvv_lva_score(position: &UnwrappedFen, move_to_make: <UnwrappedFen as HasBoard>::MoveRep) -> i16 {
    match move_to_make {
        ChessMove::NullMove => 0,
        ChessMove::StandardMove(some_standard_move) => {
            20 * match position.query_square(some_standard_move.to_square).get_contents() {
                None => 0,
                Some(piece) => get_piece_value(piece.get_piece_type()),
            } - match position.query_square(some_standard_move.from_square).get_contents() {
                None => 0,
                Some(piece) => get_piece_value(piece.get_piece_type()),
            } + (some_standard_move.to_square.rank_gap(&some_standard_move.from_square) as i16) * match position.get_color() {
                EnumColor::White => 1,
                EnumColor::Black => -1,
            }
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

fn negamax_evaluate(position: &UnwrappedFen, depth: i8) -> i32 {
    match depth <= 0 {
        true => pesto_evalutation_stm(position),
        false => {
            let mut valid_moves = position.get_pseudo_legal_proper_moves();
            mvv_lva_sort(position, &mut valid_moves);

            let mut score_thus_far = i32::MIN + 1;

            for hopeful_move in valid_moves {
                let successor_position = position.after_move(hopeful_move);
                let successor_estimated_value = -negamax_evaluate(&successor_position, depth-1);

                // I probably shouldn't be trying to be fancy, but I kind of want to punish 
                // the evaluation if there are move options of similar but slightly lower 
                // evaluation. Doing too much work to avoid overflows anywhere. Good thing 
                // this gets replaced later. Partly I'm also hoping this functions to fill 
                // some of the large gaps we have in the space of possible outputs. 
                score_thus_far = score_thus_far.max(successor_estimated_value);
            }

            score_thus_far
        }
    }
}

#[allow(dead_code)]
pub(crate) fn negamax_best_move(position: &UnwrappedFen, depth: i8) -> (<UnwrappedFen as HasBoard>::MoveRep, i32) {
    let valid_moves = position.get_legal_proper_moves();
    
    let mut best_move = ChessMove::NullMove;
    let mut best_eval = i32::MIN;

    for hopeful_move in valid_moves {
        let successor_position = position.after_move(hopeful_move);

        match best_move {
            ChessMove::NullMove => {
                best_move = hopeful_move;
                best_eval = negamax_evaluate(&successor_position, depth - 1)
            },
            _ => {
                match negamax_evaluate(&successor_position, depth - 1) > best_eval {
                    true => {
                        best_move = hopeful_move;
                        best_eval = negamax_evaluate(&successor_position, depth - 1)
                    },
                    false => {},
                }
            }
        }
    }

    (best_move, best_eval)
}