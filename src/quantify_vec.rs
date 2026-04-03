use std::ops::{Index, Range};

use crate::quantify::*;

impl<'a, 'pattern, T, Item, Pattern> Quantify<'a, 'pattern, T, Item, Pattern> for Vec<T>
where
    Self: 'a + Clone + Index<usize, Output = T> + Index<Range<usize>, Output = [T]> + IntoIterator,
    T: PartialEq + Sized + 'a,
    Item: PartialEq<&'a T> + 'pattern,
    Pattern: 'pattern + Clone + Iterator<Item = Item>,
{
    type Pattern = Pattern;
}
