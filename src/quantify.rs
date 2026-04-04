use crate::quantifier::*;

use std::{ops::{Index, Range, RangeTo}, slice::Iter};
use std::fmt::Debug;

pub fn build_patterns<'r>(patterns: &'r [Vec<i32>]) -> Vec<Iter<'r, i32>> {
    patterns.iter().map(|x| x.iter()).collect()
}

pub trait Quantify<'collection, 'pattern, T, Item, Pattern>
where
    Self: 'collection + Clone + Index<usize, Output = T> + Index<Range<usize>, Output = [T]> + Index<RangeTo<usize>, Output = [T]> + IntoIterator,
    T: Debug + PartialEq + Sized + 'collection,
    Item: Debug + PartialEq<&'collection T> + 'pattern,
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
        &'collection self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Option<&'collection [T]> {
        self.matches_pattern(pattern, quantifier).first().copied()
    }

    fn is_match(&'collection self, pattern: &'pattern Self::Pattern, quantifier: &Quantifier) -> bool {
        !self.matches_pattern(pattern, quantifier).is_empty()
    }

    fn last_match(
        &'collection self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Option<&'collection [T]> {
        self.matches_pattern(pattern, quantifier).last().copied()
    }

    /**
    Return a vector containing each slice that matches all of the quantified patterns.
     */
    fn  matches_all(
        &'collection self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<&'collection [T]> {
        let mut matches = vec![];

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
        &'collection self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<&'collection [T]> {
        let mut matches = vec![];

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
        &'collection self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<&'collection [T]> {
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
        &'collection self,
        patterns: &'pattern Vec<Self::Pattern>,
        quantifier: &Quantifier,
    ) -> Vec<&'collection [T]> {
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
        &'collection self,
        pattern: &'pattern Self::Pattern,
        quantifier: &Quantifier,
    ) -> Vec<&'collection [T]> {
        let mut matches = vec![];

        let self_len = Self::calculate_length(self);

        let mut pc = pattern.clone();
        println!("Pattern:");
        for _ in 0..Self::calculate_pattern_length(pattern) {
            println!("- {:?}", pc.next().unwrap());
        }

        for i in 0..=self_len {
            for j in i..=self_len {
                let slice = if i == 0 { &self[..j] } else { &self[i..j] };

                println!("SLICE: {:?} ({})", i..j, j - i);
                println!("{:?}", slice);

                if self.try_match(pattern, quantifier, slice) {
                    println!("Match!");
                    matches.push(slice);
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
    ) -> Vec<&'collection [T]> {
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

    fn quantify(&'collection self, _pattern: &'pattern Self::Pattern) -> Quantifier {
        todo!()
    }

    // --- Matching Logic ---

    fn try_match(
        &'collection self,
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

                    if !self.try_match(pattern, &Quantifier::One, sub_slice) {
                        return false;
                    }
                }

                true
            },
            Quantifier::ZeroOrOne => {
                if slice_len < 1 {
                    return true;
                }

                self.try_match(pattern, &Quantifier::One, slice)
            },
            _ => false,
        }
    }
}
