
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

pub(crate) mod helper_types;

pub(crate) mod helper_consts;

pub(crate) mod helper_traits;

pub(crate) mod default_implementations;
