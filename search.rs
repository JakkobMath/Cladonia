
// This is just a draft for now.

pub(crate) mod eval_abstracts {
    use std::ops::Neg;
    pub(crate) trait NegamaxCompatible: Ord + Neg {}
    pub(crate) trait ABCompatible: Ord + Neg {
        type WindowParams: Default;
        type Window: Neg;
        fn window_about(&self, params: Self::WindowParams) -> Self::Window;
        fn in_window(&self, window: Self::Window) -> bool;
        fn widen(window: Self::Window) -> Self::Window; // in case all continuations get pruned
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