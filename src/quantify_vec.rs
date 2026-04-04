use std::ops::{Index, Range};
use std::fmt::Debug;
use crate::quantify::*;

impl<'a, 'pattern, T, Item, Pattern> Quantify<'a, 'pattern, T, Item, Pattern> for Vec<T>
where
    Self: 'a + Clone + Index<usize, Output = T> + Index<Range<usize>, Output = [T]> + IntoIterator,
    T: Clone + Debug + PartialEq + Sized + 'a,
    Item: Clone + Debug + PartialEq<&'a T> + 'pattern,
    Pattern: 'pattern + Debug + Clone + Iterator<Item = Item>,
{
    type Pattern = Pattern;
}
