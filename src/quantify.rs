use crate::quantifier::*;

use std::{fmt::Display, ops::{Index, Range}};

pub trait Quantify<'a, 'pattern, P, T, Item>
where
    Self: 'a + Clone + Index<usize, Output = T> + Index<Range<usize>, Output = [T]> + IntoIterator,
    T: Display + PartialEq + Sized + 'a,
    Item: Display + PartialEq<&'a T> + 'pattern,
    P: 'pattern + Clone + Iterator<Item = Item>,
{
    // --- Pattern Management ---

    fn calculate_length(quantify: &Self) -> usize {
        quantify.clone().into_iter().count()
    }

    fn calculate_pattern_length(pattern: &P) -> usize {
        pattern.clone().count()
    }

    // --- Matching ---

    fn first_match(
        &'a self,
        pattern: &'pattern P,
        quantifier: &'a Quantifier,
    ) -> Option<&'a [T]> {
        self.matches(pattern, quantifier).first().copied()
    }

    fn is_match(&'a self, pattern: &'pattern P, quantifier: &'a Quantifier) -> bool {
        !self.matches(pattern, quantifier).is_empty()
    }

    fn last_match(
        &'a self,
        pattern: &'pattern P,
        quantifier: &'a Quantifier,
    ) -> Option<&'a [T]> {
        self.matches(pattern, quantifier).last().copied()
    }

    fn  matches(
        &'a self,
        pattern: &'pattern P,
        quantifier: &'a Quantifier,
    ) -> Vec<&'a [T]> {
        let mut matches = vec![];
        // let pattern_len = Self::calculate_pattern_length(pattern);
        let self_len = Self::calculate_length(self);

        for i in 0..self_len {
            let slice = &self[i..self_len];
            if self.try_match(pattern, quantifier, slice) {
                matches.push(slice);
            }
        }

        matches
    }

    // --- Quantification ---

    fn quantify(&'a self, _pattern: &'pattern P) -> Quantifier {
        todo!()
    }

    // --- Matching Logic ---

    fn try_match(
        &'a self,
        pattern: &'pattern P,
        quantifier: &'a Quantifier,
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
            }
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
            }
            _ => false,
        }
    }
}
