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

#[derive(Clone, Debug, PartialEq)]
pub struct PatternTerminal<'pattern, T>
where
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    value: T,
    _phantom: PhantomData<&'pattern T>
}

impl<'pattern, T> PatternTerminal<'pattern, T>
where
    Self: 'pattern,
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    pub fn new(value: T) -> PatternTerminal<'pattern, T> {
        PatternTerminal { value, _phantom: PhantomData {} }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternGroup<'pattern, T>
where
    Self: 'pattern,
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    children: Vec<Rc<PatternSymbol<'pattern, T>>>,
    _phantom: PhantomData<&'pattern T>,
}

impl<'pattern, T> PatternGroup<'pattern, T>
where
    Self: 'pattern,
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    pub fn new(children: Vec<Rc<PatternSymbol<'pattern, T>>>) -> PatternGroup<'pattern, T> {
        PatternGroup {
            children,
            _phantom: PhantomData {},
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatternSymbol<'pattern, T>
where
    Self: 'pattern,
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    Terminal(Rc<PatternTerminal<'pattern, T>>),
    Group(Rc<PatternGroup<'pattern, T>>),
}

pub struct Pattern<'pattern, T>
where
    Self: 'pattern,
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    /**
    The symbol to match.
     */
    symbol: PatternSymbol<'pattern, T>,

    /**
    The quantifier to used to match the symbol.
     */
    quantifier: Quantifier,

    /**
    Whether or not the pattern is a negative search.
    This can be thought of as similar to the logical not operator (`!` in Rust, `^` in Regex).

    If `false`, the pattern will be matched normally.
    If `true`, then the pattern will be considered as matched only if the symbol isn't matched.
     */
    is_negative: bool,

    /**
    Whether or not the pattern should capture matches (i.e. add matches to the result).
    This can be thought of as similar to capturing and non-capturing groups in Regex.

    If `false`, the pattern will not capture matches.
    If `true`, then the pattern will capture matches.
     */
    is_capturing: bool,
}

impl<'pattern, T> Pattern<'pattern, T>
where
    Self: 'pattern,
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    pub fn new(symbol: PatternSymbol<'pattern, T>, quantifier: Quantifier, is_negative: bool, is_capturing: bool) -> Rc<Pattern<'pattern, T>> {
        Rc::new(
            Pattern {
                symbol,
                quantifier,
                is_negative,
                is_capturing,
            }
        )
    }

    pub fn new_group(children: Vec<Rc<PatternSymbol<'pattern, T>>>, quantifier: Quantifier, is_negative: bool, is_capturing: bool) -> Rc<Pattern<'pattern, T>> {
        Self::new(
            PatternSymbol::Group(
                Rc::new(
                    PatternGroup::new(children)
                )
            ),
            quantifier,
            is_negative,
            is_capturing
        )
    }

    pub fn new_terminal(value: T, quantifier: Quantifier, is_negative: bool, is_capturing: bool) -> Rc<Pattern<'pattern, T>> {
        Self::new(
            PatternSymbol::Terminal(
                Rc::new(
                    PatternTerminal::new(value)
                )
            ),
            quantifier,
            is_negative,
            is_capturing
        )
    }

}
