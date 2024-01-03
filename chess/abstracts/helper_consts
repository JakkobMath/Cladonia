
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
