use crate::quantifier::*;

use std::{ops::{Index, Range}, slice::Iter};

// pub struct QuantifiedClass<'a, 'pattern, T, Item, Pattern, Q>
// where
//     T: PartialEq + Sized + 'a,
//     Item: PartialEq<&'a T> + 'pattern,
//     Q: Quantify<'a, 'pattern, T, Item, Pattern>,
// {
//     quantifier: Quantifier,
//     patterns: Vec<Q>,
// }

pub fn build_patterns<'r>(patterns: &'r [Vec<i32>]) -> Vec<Iter<'r, i32>> {
    patterns.iter().map(|x| x.iter()).collect()
}

pub trait Quantify<'a, 'pattern, T, Item, Pattern>
where
    Self: 'a + Clone + Index<usize, Output = T> + Index<Range<usize>, Output = [T]> + IntoIterator,
    T: PartialEq + Sized + 'a,
    Item: PartialEq<&'a T> + 'pattern,
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

    // --- Matching ---

    fn first_match(
        &'a self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Option<&'a [T]> {
        self.matches_pattern(pattern, quantifier).first().copied()
    }

    fn is_match(&'a self, pattern: &'pattern Self::Pattern, quantifier: &Quantifier) -> bool {
        !self.matches_pattern(pattern, quantifier).is_empty()
    }

    fn last_match(
        &'a self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Option<&'a [T]> {
        self.matches_pattern(pattern, quantifier).last().copied()
    }

    /**
    Return a vector containing each slice that matches all of the quantified patterns.
     */
    fn  matches_all(
        &'a self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<&'a [T]> {
        let mut matches = vec![];
        // let pattern_len = Self::calculate_pattern_length(pattern);
        let self_len = Self::calculate_length(self);

        // Loop from 0 to the length of self:
        for i in 0..self_len {
            // Get the next slice to try matching.
            let slice = &self[i..self_len];

            let mut is_match = true;

            // For each pattern:
            for pattern in patterns {
                // If the slice does not match the pattern:
                if !self.try_match(pattern, quantifier, slice) {
                    // Remove the match.
                    is_match = false;

                    // A match was found; try matching the next slice.
                    // To do so, break the loop.
                    break;
                }
            }

            if is_match {
                // Add the match.
                matches.push(slice);
            }
        }

        matches
    }

    /**
    Return a vector containing each slice that matches any of the quantified patterns.
     */
    fn  matches_any(
        &'a self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<&'a [T]> {
        let mut matches = vec![];
        // let pattern_len = Self::calculate_pattern_length(pattern);
        let self_len = Self::calculate_length(self);

        // Loop from 0 to the length of self:
        for i in 0..self_len {
            // Get the next slice to try matching.
            let slice = &self[i..self_len];

            // For each pattern:
            for pattern in patterns {
                // If the slice matches the pattern:
                if self.try_match(pattern, quantifier, slice) {
                    // Add the match.
                    matches.push(slice);

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
        &'a self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<&'a [T]> {
        let mut matches = vec![];
        // let pattern_len = Self::calculate_pattern_length(pattern);
        let self_len = Self::calculate_length(self);

        // Loop from 0 to the length of self:
        for i in 0..self_len {
            // Get the next slice to try matching.
            let slice = &self[i..self_len];

            // For each pattern:
            for pattern in patterns {
                // If the slice matches the pattern:
                if !self.try_match(pattern, quantifier, slice) {
                    // Add the match.
                    matches.push(slice);

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
        &'a self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<&'a [T]> {
        let mut matches = vec![];
        // let pattern_len = Self::calculate_pattern_length(pattern);
        let self_len = Self::calculate_length(self);

        // Loop from 0 to the length of self:
        for i in 0..self_len {
            // Get the next slice to try matching.
            let slice = &self[i..self_len];

            let mut is_match = true;

            // For each pattern:
            for pattern in patterns {
                // If the slice matches the pattern:
                if self.try_match(pattern, quantifier, slice) {
                    // Remove the match.
                    is_match = false;

                    // A match was found; try matching the next slice.
                    // To do so, break the loop.
                    break;
                }
            }

            if is_match {
                // Add the match.
                matches.push(slice);
            }
        }

        matches
    }

    /**
    Return a vector containing each slice that matches the quantified pattern.
     */
    fn  matches_pattern(
        &'a self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Vec<&'a [T]> {
        let mut matches = vec![];

        let self_len = Self::calculate_length(self);

        for i in 0..self_len {
            let slice = &self[i..self_len];

            if self.try_match(pattern, quantifier, slice) {
                matches.push(slice);
            }
        }

        matches
    }

    /**
    Return a vector containing each slice that does not  match the quantified pattern.
     */
    fn  matches_pattern_not(
        &'a self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Vec<&'a [T]> {
        let mut matches = vec![];

        let self_len = Self::calculate_length(self);

        for i in 0..self_len {
            let slice = &self[i..self_len];

            if !self.try_match(pattern, quantifier, slice) {
                matches.push(slice);
            }
        }

        matches
    }

    // --- Quantification ---

    fn quantify(&'a self, _pattern: &'pattern Self::Pattern) -> Quantifier {
        todo!()
    }

    // --- Matching Logic ---

    fn try_match(
        &'a self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
        slice: &'a [T],
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

                    if !self.try_match(pattern, &Quantifier::One, sub_slice) {
                        return false;
                    }
                }

                true
            },
            Quantifier::ZeroOrOne => true,
            _ => false,
        }
    }
}
