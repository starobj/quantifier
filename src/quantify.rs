use crate::pattern::*;
use crate::quantifier::*;

use std::rc::Rc;
use std::marker::PhantomData;
use std::{ops::{Index, Range, RangeTo}};
use std::fmt::Debug;

pub struct PatternNode<'pattern, T>
where
    Self: 'pattern + Clone + Iterator<Item = T>,
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    value: Option<T>,
    children: Option<Rc<PatternNode<'pattern, T>>>,
    phantom: PhantomData<&'pattern T>,
}

impl<'pattern, T> PatternNode<'pattern, T>
where
    Self: 'pattern + Clone + Iterator<Item = T>,
    T: Clone + Debug + PartialEq<&'pattern T> + 'pattern,
{
    pub fn new(value: Option<T>, children: Option<Rc<PatternNode<'pattern, T>>>) -> PatternNode<'pattern, T> {
        PatternNode {
            value,
            children,
            phantom: PhantomData {}
        }
    }
}

pub trait Quantify<'collection, 'pattern, T, Item, Pattern>
where
    Self: 'collection + Clone + Index<usize, Output = T> + Index<Range<usize>, Output = [T]> + Index<RangeTo<usize>, Output = [T]> + IntoIterator,
    T: Clone + Debug + PartialEq + Sized + 'collection,
    Item: Clone + Debug + PartialEq<&'collection T> + 'pattern,
{
    // --- Pattern Management ---

    type Pattern: 'pattern + Clone + Iterator<Item = Item>;

    // --- Static Methods ---

    fn calculate_length(quantify: &Self) -> usize {
        quantify.clone().into_iter().count()
    }

    fn calculate_pattern_length(pattern: &Self::Pattern) -> usize {
        pattern.clone().count()
    }

    // --- Matching Logic ---

    fn try_match(
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
        slice: &'collection [T],
    ) -> bool {
        let slice_len = slice.len();

        match quantifier {
            Quantifier::One => {
                if Self::calculate_pattern_length(pattern) != slice.len() {
                    return false;
                }

                let mut pattern_clone = pattern.clone();

                for item in slice {
                    if let Some(pattern_item) = pattern_clone.next() {
                        if pattern_item != item {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                true
            },
            Quantifier::ExactCount(n) => {
                let pattern_len = Self::calculate_pattern_length(pattern);

                if slice_len % pattern_len != 0 || slice_len / pattern_len != *n {
                    return false;
                }

                for i in 0..*n {
                    let sub_slice = &slice[i * pattern_len..(i + 1) * pattern_len];

                    if !Self::try_match(pattern, &Quantifier::One, sub_slice) {
                        return false;
                    }
                }

                true
            },
            Quantifier::ZeroOrOne => {
                if slice_len < 1 {
                    return true;
                }

                Self::try_match(pattern, &Quantifier::One, slice)
            },
            _ => false,
        }
    }

    // --- Matching ---

    fn first_match(
        &'collection self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Option<PatternMatch<'collection, T>> {
        self.matches_pattern(pattern, quantifier).first().cloned()
    }

    fn is_match(&'collection self, pattern: &'pattern Self::Pattern, quantifier: &Quantifier) -> bool {
        !self.matches_pattern(pattern, quantifier).is_empty()
    }

    fn last_match(
        &'collection self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Option<PatternMatch<'collection, T>> {
        self.matches_pattern(pattern, quantifier).last().cloned()
    }

    /**
    Return a vector containing each slice that matches all of the quantified patterns.
     */
    fn  matches_all(
        &'collection self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<PatternMatch<'collection, T>> {
        let mut matches: Vec<PatternMatch<'collection, T>> = vec![];

        let self_len = Self::calculate_length(self);

        // Loop from 0 to the length of self:
        for i in 0..self_len {
            // Get the range of the next slice.
            let range = i..self_len;

            // Get the next slice to try matching.
            let slice = &self[i..self_len];

            let mut is_match = true;

            // For each pattern:
            for pattern in patterns {
                // If the slice does not match the pattern:
                if !Self::try_match(pattern, quantifier, slice) {
                    // Remove the match.
                    is_match = false;

                    // A match was found; try matching the next slice.
                    // To do so, break the loop.
                    break;
                }
            }

            if is_match {
                // Add the match.
                matches.push(PatternMatch::new(range.clone(), slice));
            }
        }

        matches
    }

    /**
    Return a vector containing each slice that matches any of the quantified patterns.
     */
    fn  matches_any(
        &'collection self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<PatternMatch<'collection, T>> {
        let mut matches = vec![];

        let self_len = Self::calculate_length(self);

        // Loop from 0 to the length of self:
        for i in 0..self_len {
            // Get the range of the next slice.
            let range = i..self_len;

            // Get the next slice to try matching.
            let slice = &self[i..self_len];

            // For each pattern:
            for pattern in patterns {
                // If the slice matches the pattern:
                if Self::try_match(pattern, quantifier, slice) {
                    // Add the match.
                    matches.push(PatternMatch::new(range.clone(), slice));

                    // A match was found; try matching the next slice.
                    // To do so, break the loop.
                    break;
                }
            }
        }

        matches
    }

    /**
    Return a vector containing each slice that doesn't match any of the quantified patterns.
     */
    fn  matches_any_not(
        &'collection self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<PatternMatch<'collection, T>> {
        let mut matches = vec![];
        // let pattern_len = Self::calculate_pattern_length(pattern);
        let self_len = Self::calculate_length(self);

        // Loop from 0 to the length of self:
        for i in 0..self_len {
            // Get the range of the next slice.
            let range = i..self_len;

            // Get the next slice to try matching.
            let slice = &self[i..self_len];

            // For each pattern:
            for pattern in patterns {
                // If the slice matches the pattern:
                if !Self::try_match(pattern, quantifier, slice) {
                    // Add the match.
                    matches.push(PatternMatch::new(range.clone(), slice));

                    // A match was found; try matching the next slice.
                    // To do so, break the loop.
                    break;
                }
            }
        }

        matches
    }

    /**
    Return a vector containing each slice that matches none of the quantified patterns.
     */
    fn  matches_none(
        &'collection self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<PatternMatch<'collection, T>> {
        let mut matches = vec![];
        // let pattern_len = Self::calculate_pattern_length(pattern);
        let self_len = Self::calculate_length(self);

        // Loop from 0 to the length of self:
        for i in 0..self_len {
            // Get the range of the next slice.
            let range = i..self_len;

            // Get the next slice to try matching.
            let slice = &self[range.clone()];

            let mut is_match = true;

            // For each pattern:
            for pattern in patterns {
                // If the slice matches the pattern:
                if Self::try_match(pattern, quantifier, slice) {
                    // Remove the match.
                    is_match = false;

                    // A match was found; try matching the next slice.
                    // To do so, break the loop.
                    break;
                }
            }

            if is_match {
                // Add the match.
                matches.push(PatternMatch::new(range.clone(), slice));
            }
        }

        matches
    }

    /**
    Return a vector containing each slice that matches the quantified pattern.
     */
    fn  matches_pattern(
        &'collection self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Vec<PatternMatch<'collection, T>> {
        let mut matches = vec![];

        let self_len = Self::calculate_length(self);

        let mut pc = pattern.clone();
        println!("Pattern:");
        for _ in 0..Self::calculate_pattern_length(pattern) {
            println!("- {:?}", pc.next().unwrap());
        }

        for i in 0..=self_len {
            for j in i..=self_len {
                let range = i..j;

                let slice = if i == 0 { &self[..j] } else { &self[range.clone()] };

                println!("SLICE: {:?} ({})", i..j, j - i);
                println!("{:?}", slice);

                if Self::try_match(pattern, quantifier, slice) {
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

    /**
    Return a vector containing each slice that does not  match the quantified pattern.
     */
    fn  matches_pattern_not(
        &'collection self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Vec<PatternMatch<'collection, T>> {
        let mut matches = vec![];

        let self_len = Self::calculate_length(self);

        let mut pc = pattern.clone();
        println!("Pattern:");
        for _ in 0..Self::calculate_pattern_length(pattern) {
            println!("- {:?}", pc.next().unwrap());
        }

        for i in 0..=self_len {
            for j in i..=self_len {
                let range = i..j;

                let slice = if i == 0 { &self[..j] } else { &self[range.clone()] };

                println!("SLICE: {:?} ({})", i..j, j - i);
                println!("{:?}", slice);

                if !Self::try_match(pattern, quantifier, slice) {
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

    // --- Quantification ---

    fn quantify(&'collection self, _pattern: &'pattern Self::Pattern) -> Quantifier {
        todo!()
    }
}
