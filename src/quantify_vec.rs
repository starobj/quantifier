use std::{fmt::Display, ops::{Index, Range}};

use crate::quantify::*;

impl<'a, 'pattern, P, T, Item> Quantify<'a, 'pattern, P, T, Item> for Vec<T>
where
    Self: 'a + Clone + Index<usize, Output = T> + Index<Range<usize>, Output = [T]> + IntoIterator,
    T: Display + PartialEq + Sized + 'a,
    Item: Display + PartialEq<&'a T> + 'pattern,
    P: 'pattern + Clone + Iterator<Item = Item>,
{}
