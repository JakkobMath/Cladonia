
// This is just a draft for now.

// HEAVY emphasis that I'm drafting things here. Very likely to be restructured into something more legible.

pub(crate) mod eval_abstracts {

    // Different search methods demand different traits from the evaluation functions provided to 
    // them. This is intended to be a place to put those traits for now. 

    use std::ops::Neg;

    use super::searches::*;

    // Required for any eval

    pub(crate) trait Evaluates {
        type GamestateRep: PseudolegalGeneratingGamestate;
        fn get_evaluation(pos_in: &Self::GamestateRep) -> Self;
    }

    // Todo: figure out whether it's more elegant to treat evaluators that have multiple levels 
    // differently here, or just build it into the searches. Probably that's something to build 
    // into the search. 

    // Traits to capture compatibility with different searches

    pub(crate) trait NegamaxCompatible: Ord + Neg + Evaluates {}

    pub(crate) trait ABCompatible: Ord + Neg + Evaluates {
        type WindowParams: Default;
        type Window: Neg;
        fn window_about(&self, params: Self::WindowParams) -> Self::Window;
        fn in_window(&self, window: Self::Window) -> bool;
        fn widen(window: Self::Window) -> Self::Window; // In case all continuations get pruned. 
        // Maybe extend to widen_up and widen_down later. 
    }

    // Traits to affect what happens to the generated moves

    pub(crate) trait IncrementallyUpdatingEvaluator: Evaluates {
        fn update_eval(&self, pos_in: &Self::GamestateRep, move_in: &<Self::GamestateRep as UpdatesOnMove>::MoveRep) -> Self;
    }
}

/*

Planning out the structure of the module:

A search should take an eval as a parameter. Evals should be treated as (potentially) polymorphic 
over a family of gamestate representation types. An eval provides some functionality to the search, 
and the search may decide what functionality needs to be provided. For instance, an AB search needs 
evals to produce an output of some type T, such that T admits the calculation of minima and maxima, 
admits the construction of windows about itself, and admits the querying of windows. An eval may 
also demand properties from its expected input type, such as a NNUE eval demanding that the 
gamestate type keep track of an accumulator and provde functionality for initializing it from a 
startpos. 

Once it has its eval, a search should specify a gamestate tree structure and methods for extending 
it. An AB search might keep some tree/digraph structure and provide functionality for iterative 
deepening. That deepening step could involve a quiescence search, which would require the input 
eval to require the gamestate representations to implement an is_quiet function. 

I really, really hope that inlining combines this all more.

*/

/*

The chess module exists largely to provide the FENnec trait and some methods associated to it. If 
we want to allow the possibility of running other games through the same search code, we'll 
probably need to generalize FENnec into some larger Searchable trait...

Let BasicSearchable be the trait encoding the minimum functionality common whenever there's a 
triple of compatible gamestate/evaluator/basic search method. For example, the basic search 
methods might include BFS-negamax, the basically indistinguishable but longer BFS-minimax, AB, 
true MCTS, UCT-MCTS, PUCT, et cetera. Also in the search code, create traits like 
AdmitsQuiescenceSearch, AdmitsWDLRescaling, AdmitsTranspositionTables, AdmitsOpeningBook, 
AdmitsEndgameTablebase, AdmitsMoveOrdering, AdmitsNullMovePruning, AdmitsFutilityPruning, 
AdmitsKillerMoves, AdmitsDepthReductions, and the like. Now implemet the Searchable trait inside 
each game module by calling the BasicSearchable methods and modifying the results or inputs or 
whatever using the optimizations' methods. 

*/

pub(crate) mod searches {

    // To hold the code for Negamax, AB, etc until I possibly rearrange things. 
    // Also search abstracts.

    // Traits for gamestate representaiton types. 

    pub(crate) trait UpdatesOnMove: Copy {
        type MoveRep;

        fn make_move(&mut self, legal_move: Self::MoveRep) -> ();
        fn after_move(&self, legal_move: Self::MoveRep) -> Self;
    }

    pub(crate) trait BasicGamestate: UpdatesOnMove {
        fn get_legal_moves(&self) -> Vec<Self::MoveRep>;
    }

    pub(crate) trait PseudolegalGeneratingGamestate: UpdatesOnMove {
        fn get_pseudolegal_moves(&self) -> Vec<Self::MoveRep>;
        fn check_remaining_legality(&self, pseudolegal_move: Self::MoveRep) -> bool;
    }

}
