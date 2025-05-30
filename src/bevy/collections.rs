use crate::{Arbitrary, Result, Unstructured};
use bevy::platform::collections::{HashMap, HashSet};
use std::hash::Hash;

impl<'a, K, V> Arbitrary<'a> for HashMap<K, V>
where
    K: Arbitrary<'a> + Eq + Hash,
    V: Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.arbitrary_iter()?.collect()
    }

    fn arbitrary_take_rest(u: Unstructured<'a>) -> Result<Self> {
        u.arbitrary_take_rest_iter()?.collect()
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl<'a, A> Arbitrary<'a> for HashSet<A>
where
    A: Arbitrary<'a> + Eq + Hash,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.arbitrary_iter()?.collect()
    }

    fn arbitrary_take_rest(u: Unstructured<'a>) -> Result<Self> {
        u.arbitrary_take_rest_iter()?.collect()
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (0, None)
    }
}
