// Movegen and perft code. 

use crate::chess::{abstracts::helper_traits::*, implementations::*};
use super::board_rep::*;

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