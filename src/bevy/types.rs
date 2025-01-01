use crate::{Arbitrary, Result, Unstructured};

use bevy::{math::IVec2, prelude::Entity};
// use core::num::NonZero;
use rand::{thread_rng, Rng};

impl<'a> Arbitrary<'a> for Entity {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let low = u32::arbitrary(u)?;
        // Max value for high: 0x7FFF_FFFF (2147483647)
        let high = thread_rng().gen_range(1..10_000);

        let bits = pack_into_u64(low, high);

        Ok(Entity::from_bits(bits))
    }
}

/// Ripped from Bevy
const fn pack_into_u64(low: u32, high: u32) -> u64 {
    ((high as u64) << u32::BITS) | (low as u64)
}

impl<'a> Arbitrary<'a> for IVec2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let x = i32::arbitrary(u)?;
        let y = i32::arbitrary(u)?;
        Ok(IVec2::new(x, y))
    }
}
