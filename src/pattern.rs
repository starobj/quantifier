use std::{fmt::Debug, ops::Range, slice::Iter};

pub fn build_patterns<'r>(patterns: &'r [Vec<i32>]) -> Vec<Iter<'r, i32>> {
    patterns.iter().map(|x| x.iter()).collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternMatch<'collection, T>
where
    Self: 'collection,
    T: Clone + Debug + PartialEq + Sized + 'collection,
{
    range: Range<usize>,
    slice: &'collection [T],
}

impl<'collection, T> PatternMatch<'collection, T>
where
    Self: 'collection,
    T: Clone + Debug + PartialEq + Sized + 'collection,
{
    pub fn new(range: Range<usize>, slice: &'collection [T]) -> PatternMatch<'collection, T> {
        PatternMatch { range: range.clone(), slice }
    }
}
