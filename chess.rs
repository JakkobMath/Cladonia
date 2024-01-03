
// This module deals only with chess. In particular, it provides a trait for types which contain 
// gamestate information, provides basic pseudolegal movegen code and legality checking code for 
// types implementing said trait, and gives a particular implementation of that trait together 
// with a FEN parser. Future work should add gamestate types implementing FENnec which are more 
// amenable to the kinds of computations Cladonia will be doing and override FENnec methods to 
// make movegen actually fast. For example, I'll probably use this default movegen to generate 
// magic bitboards at some point. 

pub(crate) mod abstracts;

pub(crate) mod implementations;
