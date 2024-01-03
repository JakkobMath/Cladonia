
// This is just a draft for now.

pub(crate) mod eval_abstracts {
    use std::ops::Neg;
    pub(crate) trait NegamaxCompatible: Ord + Neg {}
    pub(crate) trait ABCompatible: Ord + Neg {
        type WindowParams: Default;
        type Window: Neg;
        fn window_about(&self, params: Self::WindowParams) -> Self::Window;
        fn in_window(&self, window: Self::Window) -> bool;
    }
}
