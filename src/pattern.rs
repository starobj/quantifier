use std::{
    fmt::Debug, ops::{Index, Range, RangeTo}, rc::Rc, slice::Iter
};
use tracing_rc::rc::{Gc, GcVisitor, Trace};

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

pub type DynPatternTerminal<T> = dyn Fn(&T) -> bool;

#[derive(Clone)]
pub enum PatternTerminal<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    /**
    A static terminal symbol is a symbol that:
    - does not change
    - is limited to an exact value
    - is of type `T`
    - is required to implement: `Clone + Debug + PartialEq<T>`
     */
    Static(T),

    /**
    A dynamic terminal symbol is a symbol that:
    - represents a ran
    - is limited to an exact value
    - is of type `T`
    - is required to implement: `Clone + Debug + PartialEq<T>`
     */
    Dyn(&'static DynPatternTerminal<T>),
}

impl<T> PartialEq for PatternTerminal<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Static(lhs), Self::Static(rhs)) => lhs == rhs,
            (Self::Dyn(lhs), Self::Dyn(rhs)) => {
                // Compare the two function's pointers to determine equality.
                Rc::ptr_eq(
                    &Rc::new(lhs),
                    &Rc::new(rhs)
                )
            },
            _ => false,
        }
    }
}

impl<T> Trace for PatternTerminal<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    fn visit_children(&self, _visitor: &mut GcVisitor) {}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PatternGroupType {
    All,
    Any,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternGroup<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    group_type: PatternGroupType,
    children: Vec<Gc<Pattern<T>>>,
}

impl<T> PatternGroup<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    pub fn new(group_type: PatternGroupType, children: Vec<Gc<Pattern<T>>>) -> PatternGroup<T> {
        PatternGroup {
            group_type,
            children,
        }
    }

    pub fn get_child_count(&self) -> usize {
        self.children.len()
    }

    pub fn get_children(&self) -> Iter<'_, Gc<Pattern<T>>> {
        self.children.iter()
    }

    pub fn get_children_iter(&self) -> std::vec::IntoIter<Gc<Pattern<T>>> {
        self.children.clone().into_iter()
    }

    pub fn get_deep_count(&self) -> usize {
        let children = self.get_children();

        let mut child_count = 0;

        for child in children {
            match &child.borrow().symbol {
                PatternSymbol::Group(group) => {
                    child_count += group.borrow().get_child_count();
                },
                PatternSymbol::Terminal(_terminal) => {
                    child_count += 1;
                },
            }
        }

        return child_count;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternGroupIterator<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    index: usize,
    group: Gc<PatternGroup<T>>,
    stack: Vec<(usize, Gc<PatternGroup<T>>)>,
}

impl<T> PatternGroupIterator<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    pub fn is_empty(&self) -> bool {
        self.stack.len() < 1
    }
    pub fn peek(&mut self) -> Option<Gc<PatternGroup<T>>> {
        if self.is_empty() {
            return None;
        }

        let (_index, group) = self.stack.last().unwrap();

        return Some(group.clone());
    }

    pub fn pop(&mut self) -> Option<Gc<PatternGroup<T>>> {
        if self.is_empty() {
            return None;
        }

        let (index, group) = self.stack.pop().unwrap();

        self.index = index;
        self.group = group.clone();

        return Some(group);
    }

    pub fn push(&mut self, group: &Gc<PatternGroup<T>>) {
        self.stack.push((self.index, self.group.clone()));

        self.index = 0;
        self.group = group.clone();
    }

    pub fn seek(&mut self, index: usize) {
        self.index = index;
    }

    pub fn seek_relative(&mut self, offset: usize) {
        self.index += offset;
    }
}

impl<T> Iterator for PatternGroupIterator<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    type Item = (Gc<Pattern<T>>, Gc<PatternTerminal<T>>);

    fn next(&mut self) -> Option<Self::Item> {
        let group = self.group.clone();

        let group_ref = group.borrow();

        // If the stack is empty:
        if self.is_empty() {
            if self.index >= group_ref.get_child_count() {
                self.seek(usize::MAX);

                return None;
            }

            let pattern = &group_ref.children[self.index];

            let pattern_ref = pattern.borrow();

            match &pattern_ref.symbol {
                PatternSymbol::Group(subgroup) => {
                    self.push(subgroup);

                    return self.next();
                },
                PatternSymbol::Terminal(terminal) => {
                    self.seek_relative(1);

                    return Some((pattern.clone(), terminal.clone()));
                },
            }
        }
        // Otherwise, if the stack is not empty:
        else {
            if self.index >= group_ref.get_child_count() {
                self.pop();

                return self.next();
            }

            let pattern = group_ref.children[self.index].clone();

            match &pattern.borrow().symbol {
                PatternSymbol::Group(group) => {
                    self.push(group);

                    return self.next();
                },
                PatternSymbol::Terminal(terminal) => {
                    return Some((pattern.clone(), terminal.clone()));
                }
            }
        }
    }
}

impl<T> IntoIterator for PatternGroup<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    type Item = (Gc<Pattern<T>>, Gc<PatternTerminal<T>>);

    type IntoIter = PatternGroupIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        PatternGroupIterator {
            index: 0,
            group: Gc::new(self),
            stack: vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternIterator<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    pattern: Gc<Pattern<T>>,
    end_of_stream: bool,
    group_iter: Option<PatternGroupIterator<T>>,
}

impl<T> Iterator for PatternIterator<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    type Item = (Gc<Pattern<T>>, Gc<PatternTerminal<T>>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_of_stream {
            return None;
        }

        let pattern = &self.pattern.clone();

        let pattern_ref = pattern.borrow();

        if let Some(group_iter) = self.group_iter.as_mut() {
            let child = group_iter.next();

            if child.is_none() {
                self.end_of_stream = true;

                return None;
            }

            return Some(child.unwrap());
        }

        match &pattern_ref.symbol {
            PatternSymbol::Group(group) => {
                self.group_iter = Some(group.borrow().clone().into_iter());

                return self.next();
            },
            PatternSymbol::Terminal(terminal) => {
                self.end_of_stream = true;

                return Some((self.pattern.clone(), terminal.clone()));
            },
        }
    }
}

impl<T> IntoIterator for Pattern<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    type Item = (Gc<Pattern<T>>, Gc<PatternTerminal<T>>);

    type IntoIter = PatternIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        PatternIterator {
            pattern: Gc::new(self),
            end_of_stream: false,
            group_iter: None,
        }
    }
}

impl<T> Trace for PatternGroup<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    fn visit_children(&self, visitor: &mut GcVisitor) {
        for child in &self.children[..] {
            child.visit_children(visitor);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatternSymbol<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    Terminal(Gc<PatternTerminal<T>>),
    Group(Gc<PatternGroup<T>>),
}

impl<T> Trace for PatternSymbol<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    fn visit_children(&self, visitor: &mut GcVisitor) {
        match self {
            Self::Group(group) => {
                group.visit_children(visitor);
            },
            Self::Terminal(terminal) => {
                terminal.visit_children(visitor);
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    /**
    The symbol to match.
     */
    symbol: PatternSymbol<T>,

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

trait MatchPattern<'collection, T>:
    'collection
    + Clone
    + Index<usize, Output = T>
    + Index<Range<usize>, Output = [T]>
    + Index<RangeTo<usize>, Output = [T]>
    + IntoIterator
where
    T: 'collection + Clone + Debug + PartialEq + Sized,
{}

impl<T> Pattern<T>
where
    T: 'static + Clone + Debug + PartialEq<T>,
{
    fn collection_len<'collection, Collection>(collection: &Collection) -> usize
    where
        Collection: MatchPattern<'collection, T>
    {
        collection.clone().into_iter().count()
    }

    pub fn new(symbol: PatternSymbol<T>, quantifier: Quantifier, is_negative: bool, is_capturing: bool) -> Gc<Pattern<T>> {
        Gc::new(
            Pattern {
                symbol,
                quantifier,
                is_negative,
                is_capturing,
            }
        )
    }

    pub fn new_group(group_type: PatternGroupType, children: Vec<Gc<Pattern<T>>>, quantifier: Quantifier, is_negative: bool, is_capturing: bool) -> Gc<Pattern<T>> {
        Self::new(
            PatternSymbol::Group(
                Gc::new(
                    PatternGroup::new(group_type, children)
                )
            ),
            quantifier,
            is_negative,
            is_capturing
        )
    }

    pub fn new_literal(value: T, quantifier: Quantifier, is_negative: bool, is_capturing: bool) -> Gc<Pattern<T>> {
        Self::new(
            PatternSymbol::Terminal(
                Gc::new(
                    PatternTerminal::Static(value)
                )
            ),
            quantifier,
            is_negative,
            is_capturing
        )
    }

    pub fn new_dynamic(value: T, quantifier: Quantifier, is_negative: bool, is_capturing: bool) -> Gc<Pattern<T>> {
        Self::new(
            PatternSymbol::Terminal(
                Gc::new(
                    PatternTerminal::Static(value)
                )
            ),
            quantifier,
            is_negative,
            is_capturing
        )
    }

    /**
    Return a vector containing each slice that matches the quantified pattern.
     */
    fn  matches<'collection, Collection>(
        &self,
        collection: &'collection Collection,
    ) -> Vec<PatternMatch<'collection, T>>
    where
        Collection: MatchPattern<'collection, T>
    {
        let mut matches: Vec<PatternMatch<'collection, T>> = vec![];

        let collection_length = Self::collection_len(collection);

        for i in 0..=collection_length {
            for j in i..=collection_length {
                let range = i..j;

                let slice = if i == 0 { &collection[..j] } else { &collection[range.clone()] };

                println!("SLICE: {:?} ({})", i..j, j - i);
                println!("{:?}", slice);

                if self.is_match(slice) {
                    println!("Match!");
                    matches.push(PatternMatch::new(range.clone(), slice));
                }
                else {
                    println!("Not match!");
                }
            }
        }

        matches
    }

    fn is_match(
        &self,
        slice: &[T],
    ) -> bool {
        let slice_len = slice.len();

        match &self.symbol {
            PatternSymbol::Group(group) => {
                // let mut group_ref = group.borrow();
                // let mut children = group_ref.get_children();

                let mut children = group.borrow().get_children_iter();

                match &self.quantifier {
                    Quantifier::One => {
                        for item in slice {
                            if let Some(pattern) = children.next() {
                                let pattern_ref = pattern.borrow();

                                if self.is_negative {
                                    if pattern_ref.is_match(&vec![item.clone()][..]) {
                                        return false;
                                    }
                                }
                                else {
                                    if !pattern_ref.is_match(&vec![item.clone()][..]) {
                                        return false;
                                    }
                                }
                            }
                            else {
                                return false;
                            }
                        }

                        return true;
                    },
                    Quantifier::ExactCount(n) => {
                        todo!()
                    },
                    Quantifier::Range(range) => {
                        todo!()
                    },
                    Quantifier::OneOrMore
                    | Quantifier::ZeroOrMore
                    | Quantifier::ZeroOrOne => {
                        todo!()
                    },
                }
            },
            PatternSymbol::Terminal(terminal) => {
                match &self.quantifier {
                    Quantifier::One => {
                        if slice.len() < 1 || slice.len() > 1 {
                            return false;
                        }

                        let mut terminal_ref = terminal.borrow();

                        let actual = &slice[0];

                        match terminal_ref.clone() {
                            PatternTerminal::Dyn(dyn_pattern_terminal) => {
                                if self.is_negative {
                                    return !dyn_pattern_terminal(actual);
                                }
                                else {
                                    return dyn_pattern_terminal(actual);
                                }
                            },
                            PatternTerminal::Static(expected) => {
                                if self.is_negative {
                                    return *actual != expected;
                                }
                                else {
                                    return *actual == expected;
                                }
                            },
                        }
                    },

                    Quantifier::ExactCount(n) => {
                        if slice.len() < 1 || slice.len() > *n {
                            return false;
                        }

                        let mut subpattern = self.clone();

                        subpattern.quantifier = Quantifier::One;

                        let subslice_length = slice_len / n;

                        for i in 0..*n {
                            let subslice_start = i * subslice_length;
                            let subslice_end = subslice_start + subslice_length;
                            let subslice = &slice[subslice_start..subslice_end];

                            if self.is_negative {
                                if subpattern.is_match(subslice) {
                                    return false;
                                }
                            }
                            else {
                                if !subpattern.is_match(subslice) {
                                    return false;
                                }
                            }
                        }

                        return true;
                    },
                    Quantifier::ZeroOrOne => todo!(),
                    Quantifier::ZeroOrMore => todo!(),
                    Quantifier::OneOrMore => todo!(),
                    Quantifier::Range(range) => todo!(),
                }
            },
        }
    }
}

impl<T> Trace for Pattern<T>
where
    T: Clone + Debug + PartialEq + Sized,
{
    fn visit_children(&self, visitor: &mut GcVisitor) {
        match self.symbol.clone() {
            PatternSymbol::Group(group) => {
                group.visit_children(visitor);
            },
            PatternSymbol::Terminal(terminal) => {
                terminal.visit_children(visitor);
            },
        }
    }
}
