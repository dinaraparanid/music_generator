use std::{
    iter::{Skip, Zip},
    slice::Iter,
};

pub mod genetic;
pub mod melody_type;
pub mod midi;
pub mod notes;

trait WithNextIterable<'a, T, I: Iterator>: IntoIterator {
    fn to_iter(&'a self) -> I;

    #[inline]
    fn with_next(&'a self) -> Zip<Skip<I>, I>
    where
        Self: Sized,
    {
        self.to_iter().skip(1).zip(self.to_iter())
    }
}

impl<'a, T> WithNextIterable<'a, T, Iter<'a, T>> for Vec<T> {
    #[inline]
    fn to_iter(&'a self) -> Iter<'a, T> {
        self.iter()
    }
}
