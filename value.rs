// pub(crate) trait Scorey: Sized + Copy + Ord + Neg {
//     fn proven_score(score: i8, ) -> Self;
// }
#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum OutStyleScore {
    MatingIn(i8),
    MatedIn(i8),
    Centipawn(i8),
}

impl PartialOrd for OutStyleScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            OutStyleScore::MatingIn(n) => {
                match other {
                    OutStyleScore::MatingIn(k) => k.partial_cmp(n),
                    _ => Some(std::cmp::Ordering::Greater),
                }
            },
            OutStyleScore::MatedIn(n) => {
                match other {
                    OutStyleScore::MatedIn(k) => n.partial_cmp(k),
                    _ => Some(std::cmp::Ordering::Less),
                }
            },
            OutStyleScore::Centipawn(n) => {
                match other {
                    OutStyleScore::MatingIn(_) => Some(std::cmp::Ordering::Less),
                    OutStyleScore::MatedIn(_) => Some(std::cmp::Ordering::Greater),
                    OutStyleScore::Centipawn(k) => n.partial_cmp(k),
                }
            },
        }
    }
}

impl Ord for OutStyleScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            OutStyleScore::MatingIn(n) => {
                match other {
                    OutStyleScore::MatingIn(k) => k.cmp(n),
                    _ => std::cmp::Ordering::Greater,
                }
            },
            OutStyleScore::MatedIn(n) => {
                match other {
                    OutStyleScore::MatedIn(k) => n.cmp(k),
                    _ => std::cmp::Ordering::Less,
                }
            },
            OutStyleScore::Centipawn(n) => {
                match other {
                    OutStyleScore::MatingIn(_) => std::cmp::Ordering::Less,
                    OutStyleScore::MatedIn(_) => std::cmp::Ordering::Greater,
                    OutStyleScore::Centipawn(k) => n.cmp(k),
                }
            },
        }
    }
}

// #[derive(Clone, Copy, PartialEq, Eq, Debug)]
// pub(crate) struct 

// #[derive(Clone, Copy, PartialEq, Eq, Debug)]
// pub(crate) enum InternalScore {}