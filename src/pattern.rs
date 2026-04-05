use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::Range,
    rc::Rc,
    slice::Iter,
};

use crate::quantifier::*;

pub fn build_patterns<'r, T>(patterns: &'r [Vec<T>]) -> Vec<Iter<'r, T>> {
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

pub struct PatternNode<'pattern, T>
where
    Self: 'pattern + Clone + Iterator<Item = T>,
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    children: Option<Vec<Rc<PatternNode<'pattern, T>>>>,
    is_negative: bool,
    is_non_capturing: bool,
    quantifier: Quantifier,
    value: Option<T>,
    phantom: PhantomData<&'pattern T>,
}

impl<'pattern, T> PatternNode<'pattern, T>
where
    Self: 'pattern + Clone + Iterator<Item = T>,
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    pub fn new(value: Option<T>, children: Option<Vec<Rc<PatternNode<'pattern, T>>>>, quantifier: Quantifier, is_negative: bool, is_non_capturing: bool) -> PatternNode<'pattern, T> {
        PatternNode {
            children,
            is_negative,
            is_non_capturing,
            quantifier,
            value,
            phantom: PhantomData {},
        }
    }
}
