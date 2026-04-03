use std::ops::{Index, Range};

use crate::quantify::*;

impl<'a, 'pattern, P, T, Item> Quantify<'a, 'pattern, P, T, Item> for Vec<T>
where
    Self: 'a + Clone + Index<usize, Output = T> + Index<Range<usize>, Output = [T]> + IntoIterator,
    T: PartialEq + Sized + 'a,
    Item: PartialEq<&'a T> + 'pattern,
    P: 'pattern + Clone + Iterator<Item = Item>,
{}
