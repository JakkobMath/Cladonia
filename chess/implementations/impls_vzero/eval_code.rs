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
const MG_PAWN_B: [i16; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
     90,  93,  90,  87,  87,  90,  93,  90,
     28,  70,  18,   5,   5,  18,  70,  28,
    -29, -31, -33, -40, -40, -33, -31, -29,
    -37, -30, -47,   2,   2, -47, -30, -37,
    -30, -47, -24, -20, -20, -24, -47, -30,
    -22, -20, -20, -53, -53, -20, -20, -22,
      0,   0,   0,   0,   0,   0,   0,   0,
];

const EG_PAWN_B: [i16; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
     95, 103,  98,  88,  88,  98, 103,  95,
     38,  78,  28,   8,   8,  28,  78,  38,
    -19, -25, -23, -30, -30, -23, -25, -19,
    -27, -47, -42, -37, -37, -42, -47, -27,
    -32, -37, -39, -40, -40, -39, -37, -32,
    -32, -32, -32, -42, -42, -32, -32, -32,
      0,   0,   0,   0,   0,   0,   0,   0,   
];

const MG_KNIGHT_B: [i16; 64] = [
    -40, -32, -23, -23, -23, -23, -32, -40,
    -32,  -6,   4,   9,   9,   4,  -6, -32,
    -10,   8,  17,  23,  23,  17,   8, -10,
     -4,  13,  32,  34,  34,  32,  13,  -4,
     -4,  14,  29,  32,  32,  29,  14,  -4,
     -7,   9,  24,  30,  30,  24,   9,  -7,
    -32,  -6,   8,  12,  12,   8,  -6, -32,
    -40, -13, -13, -13, -13, -13, -13, -40,
];

const EG_KNIGHT_B: [i16; 64] = [
    -42, -34, -9, -6, -6, -9, -34, -42,
    -34,  -8,  6, 11, 11,  6,  -8, -34,
     -9,   6, 24, 28, 28, 24,   6,  -9,
     -6,  11, 28, 36, 36, 28,  11,  -6,
     -6,  11, 28, 36, 36, 28,  11,  -6,
     -9,   6, 21, 28, 28, 21,   6,  -9,
    -34,  -8,  6, 10, 10,  6,  -8, -34,
    -42, -34, -9, -6, -6, -9, -34, -42,   
];

const MG_BISHOP_B: [i16; 64] = [
    -30, -20, -12, -10, -10, -12, -20, -30,
    -20,   2,  10,  10,  10,  10,   2, -20,
    -12,  10,  10,  10,  10,  10,  10, -12,
    -10,  12,  19,  21,  21,  19,  12, -10,
    -10,  12,  20,  22,  22,  20,  12, -10,
    -12,  12,  18,  20,  20,  18,  12, -12,
    -15,  23,  15,  12,  12,  15,  23, -15,
    -30, -20, -10, -15, -15, -10, -20, -30,
];

const EG_BISHOP_B: [i16; 64] = [
    -2,  0, -1, -1, -1, -1,  0, -2,
     0, -1,  0, -1, -1,  0, -1,  0,
    -1,  0,  0,  1,  1,  0,  0, -1,
    -1, -1,  1,  6,  6,  1, -1, -1,
    -1, -1,  2,  6,  6,  2, -1, -1,
    -1,  0,  0,  2,  2,  0,  0, -1,
     0, -1,  0, -1, -1,  0, -1,  0,
    -2,  0, -1, -1, -1, -1,  0, -2,   
];

const MG_ROOK_B: [i16; 64] = [
    -1, -1, -1, -1, -1, -1, -1, -1,
    11, 11, 11, 11, 11, 11, 11, 11,
    -2, -4, -4, -4, -4, -4, -4, -2,
    -4, -4, -4, -4, -4, -4, -4, -4,
    -4, -4, -4, -4, -4, -4, -4, -4,
    -3, -4, -4, -4, -4, -4, -4, -3,
    -1, -2, -3, -3, -3, -3, -2, -1,
     3, -1, 11, 17, 17, 11, -1,  3,   
];

const EG_ROOK_B: [i16; 64] = [
     4,  7, 17, -3, -3, 17,  7,  4,
     9, -3, -3, -3, -3, -3, -3,  9,
    14, -3, -3, -3, -3, -3, -3, 14,
    -3, -3, -2, -3, -3, -2, -3, -3,
    -1, -3, -3, -3, -3, -3, -3, -1,
     5, -2, -3, -3, -3, -3, -2,  5,
     1, -1, -2, -2, -2, -2, -1,  1,
    -2, -1,  2, -1, -1,  2, -1, -2,
];

const MG_QUEEN_B: [i16; 64] = [
    -20, -11, -8, -8, -8, -8, -11, -20,
    -10,   2,  3,  2,  2,  3,   2, -10,
     -5,   4, -2, -2, -2, -2,   4,  -5,
     -4,   5, -1,  2,  2, -1,   5,  -4,
     -2,   6,  1,  4,  4,  1,   6,  -2,
     -1,   7,  3,  3,  3,  3,   7,  -1,
     -1,  10, 13, 12, 12, 13,  10,  -1,
    -10,  -1,  4,  5,  5,  4,  -1, -10,
];

const EG_QUEEN_B: [i16; 64] = [
    -5, -2, -1, -1, -1, -1, -2, -5,
    -2,  2,  3,  2,  2,  3,  2, -2,
    -1,  3,  0,  0,  0,  0,  3, -1,
    -1,  2,  0,  1,  1,  0,  2, -1,
    -1,  2,  0,  1,  1,  0,  2, -1,
    -1,  3,  0,  0,  0,  0,  3, -1,
    -2,  2,  3,  2,  2,  3,  2, -2,
    -5, -2, -1, -1, -1, -1, -2, -5,
];

const MG_KING_B: [i16; 64] = [
    -34, -24, -12,  -9,  -9, -12, -24, -34,
    -24, -13, -13, -13, -13, -13, -13, -24,
    -13, -13, -13, -13, -13, -13, -13, -13,
    -13, -13, -13, -13, -13, -13, -13, -13,
    -13, -13, -13, -13, -13, -13, -13, -13,
     -4,  -5, -13, -13, -13, -13,  -5,  -4,
     21,  16,  -6,  -6,  -6,  -6,  16,  21,
     46,  79,  37,  18,  18,  37,  79,  46,
];

const EG_KING_B: [i16; 64] = [
    -63, -30, -18, -15, -15, -18, -30,  -63,  
    -30,   6,  22,  20,  20,  22,   6,  -30,  
    -18,  22,  29,  30,  30,  29,  22,  -18,  
    -15,  20,  30,  31,  31,  30,  20,  -15,  
    -15,  19,  28,  31,  31,  28,  19,  -15,  
    -18,  13,  24,  27,  27,  24,  13,  -18,  
    -30,   0,  11,  15,  15,  11,   0,  -30,  
    -63, -30, -18, -15, -15, -18, -30,  -63,  

];

const PESTO_MG_COMBINED_B: [[i16; 64]; 6] = [
    MG_PAWN_B, 
    MG_KNIGHT_B, 
    MG_BISHOP_B, 
    MG_ROOK_B, 
    MG_QUEEN_B, 
    MG_KING_B, 
];

const PESTO_EG_COMBINED_B: [[i16; 64]; 6] = [
    EG_PAWN_B, 
    EG_KNIGHT_B, 
    EG_BISHOP_B, 
    EG_ROOK_B, 
    EG_QUEEN_B, 
    EG_KING_B, 
];

const GAME_PHASE_ADDER: [i32; 6] = [1, 3, 3, 5, 9, 0];

const DEFAULT_MG_VALUES: [i32; 6] = [100, 303, 305, 500, 900, 0];
const DEFAULT_EG_VALUES: [i32; 6] = [105, 295, 310, 520, 940, 0];

pub(crate) fn hce_stm(position: &UnwrappedFen) -> i32 {
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
                } as i32 + DEFAULT_MG_VALUES[piece_number];
                let eg_piece_square_value = match piece.get_color() {
                    EnumColor::White => PESTO_EG_COMBINED_B[piece_number][vertical_flip_index(square_index) as usize],
                    EnumColor::Black => PESTO_EG_COMBINED_B[piece_number][square_index as usize],
                } as i32 + DEFAULT_EG_VALUES[piece_number];
                let stm_multiplier = match position.get_color() == piece.get_color() {
                    true => 1,
                    false => -1,
                };
                mg_value += mg_piece_square_value * stm_multiplier;
                eg_value += eg_piece_square_value * stm_multiplier;
                game_phase += GAME_PHASE_ADDER[piece_number];
            },
        }
    }
    
    // Endpoint evals set up, now taper them to get the output eval. 

    let mg_multiplier = game_phase.min(28);
    let eg_multiplier = 28i32 - mg_multiplier;

    (mg_value * mg_multiplier + eg_value * eg_multiplier) / 28
}

// TODO: implement Evaluator etc for that evaluation. 

pub(crate) fn mvv_lva_sort(position: &UnwrappedFen, moves_list: &mut Vec<<UnwrappedFen as HasBoard>::MoveRep>) {
    moves_list.sort_by(|move_to_make, other_move| mvv_lva_score(position, *move_to_make).cmp(&mvv_lva_score(position, *other_move)));
    moves_list.retain_mut(|possible_move| position.check_remaining_legality(*possible_move));

    // Introducing this temporary fix for performance reasons. Obviously this gets deleted later :) (TODO)
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
        true => hce_stm(position),
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

fn get_sorted_plp_moves(position: &UnwrappedFen) -> Vec<ChessMove<i8,i8>> {
    let mut valid_moves = position.get_pseudo_legal_proper_moves();
    mvv_lva_sort(position, &mut valid_moves);
    return valid_moves;
}

#[allow(dead_code)]
pub(crate) fn ab_best_move(position: &UnwrappedFen, depth: i8, alpha: i32, beta: i32, wiggle_room: i32, force: bool) -> Option<(<UnwrappedFen as HasBoard>::MoveRep, i32)> {

    // Use an alpha-beta search and the hand-"crafted" evaluation to determine the value of a position. 
    // Includes the option to force a search to return a value; searches are allowed to fail by default. 
    // Searches that are forced to return a value still attempt to rely on pruning in their successors. 
    // Not yet end-of-game aware. May produce strange results in positions where mate is unavoidable. 

    // Alpha is the ambient minimum value we (from our perspective) are willing to accept. 
    // Beta is the ambient minimum value the opponent (from their perspective) is willing to accept. 

    if depth <= 0 {
        return Some((ChessMove::NullMove, hce_stm(position)));
    } else {
        let valid_moves = get_sorted_plp_moves(position);

        let mut best_move = ChessMove::NullMove;
        let mut own_alpha = alpha;
        let mut own_beta = beta;
        let mut curr_best = i32::MIN + 1;

        let mut own_wiggle_room = wiggle_room.max(7).min(150);

        let mut blocked = true;
        let mut force_successors = false;

        // TODO: remove. This is for debugging. 
        let mut blocked_iterations = 0i8;

        // Run until either (1, if forced to return a value) there is a value to return, or 
        // (2, if not forced) every child has been examined. 
        while blocked {

            // We want to tighten up the wiggle room as we go along so we don't have too high of a branching factor. 
            // If it gets too small and the search can't succeed enough it'll be doubled. Code inside the AB search 
            // function prevents successors from having an effective wiggle below (currently) 7... which should be 
            // extracted to a global constant or something, probably. 
            let successor_wiggle_room = (3 * (own_wiggle_room / 4)).max(7 * (wiggle_room / 4)) - wiggle_room;

            for hopeful_move in &valid_moves {
                let successor_position = position.after_move(*hopeful_move);

                // Will be replaced by TT later. With iterative deepening, this will let us make use of more accurate 
                // results for initial filtering. Also, before this function can be trusted, it NEEDS qsearch to make 
                // sure all lines with trades aren't pruned because of the captures. 
                let static_move_evaluation = -hce_stm(&successor_position); 
    
                match (static_move_evaluation >= own_alpha.max(i32::MIN + own_wiggle_room + 1) - own_wiggle_room && static_move_evaluation <= own_wiggle_room - own_beta.max(i32::MIN + own_wiggle_room + 1)) || force {

                    // If the move looks bad at first glance and we haven't gotten desparate for a follow-up, ignore it. 
                    // This probably turns into a depth reduction later instead of full pruning. 
                    false => {},

                    // If the move looks okay, continue with recursive AB search 
                    // to determine its value (hopefully) more accurately. 
                    true => {
                        match ab_best_move(&successor_position, depth-1, own_beta, own_alpha, successor_wiggle_room, force_successors) {
                            None => {
                                // If the search failed from the child node, ignore the continuation unless this 
                                // node needs to return a continuation and has previously failed to do so. 
                            },
                            Some((_follow_up, opponent_value)) => {
                                let searched_move_value = -opponent_value;

                                let refined_wiggle = own_wiggle_room / (depth as i32);

                                // We'll only consider this move further if it's good or if we have to. 
                                match searched_move_value >= own_alpha.max(i32::MIN + refined_wiggle + 1) - refined_wiggle || (force && blocked) { 
                                    true => {
                                        if searched_move_value > curr_best {
                                            curr_best = searched_move_value;
                                            best_move = *hopeful_move;

                                            // We are free to exit the loop: we have got a move and evaluation to return. 
                                            blocked = false;
                                        }

                                        // Update the worse we can expect and the best the opponent 
                                        // can expect should the game pass through this node. 
                                        own_alpha = own_alpha.max(searched_move_value.max(i32::MIN + refined_wiggle + 1) - refined_wiggle);
                                        own_beta = own_beta.min(opponent_value.min(i32::MAX - refined_wiggle - 1) + refined_wiggle);

                                        // If the opponent would reject this continuation from the previous node 
                                        // based on this move being too good of a response, we don't need to 
                                        // continue searching more options. 
                                        if own_beta < beta && !(force && blocked) {
                                            break;
                                        }
                                    }
                                    false => {}
                                }
                            }
                        }
                    },
                }
            }

            // Every move has been checked. If no continuations have been 
            // found but an answer is forced, we may need to re-search. 
            blocked = blocked && force;

            // On a re-search, be more generous with the bounds. 
            own_wiggle_room *= 2;

            // If we're already allowing for a very large error (>1.5 pawns), 
            // force successor positions to return answers on the next loop. 
            // If the parent position isn't forced, a next loop won't happen. 
            force_successors = own_wiggle_room > 150;

            
            // TODO: this is for debugging. Remove. 
            blocked_iterations += 1;
            if blocked_iterations > 12 {
                // Trigger debug thing!

                blocked_iterations += 12; // Fix breakpoint please?
                if blocked_iterations == 32 {
                    println!("Stop sending me unread variable errors please.")
                }
                break;
            }
        }



        return match !best_move.is_proper() && !force {
            true => None,
            false => Some((best_move, curr_best)),
        }
    }
}