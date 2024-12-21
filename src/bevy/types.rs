use crate::{Arbitrary, Result, Unstructured};

use bevy::{math::IVec2, prelude::Entity};

impl<'a> Arbitrary<'a> for Entity {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let bits = u64::arbitrary(u)?;

        Ok(Entity::from_bits(bits))
    }
}

impl<'a> Arbitrary<'a> for IVec2 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let x = i32::arbitrary(u)?;
        let y = i32::arbitrary(u)?;
        Ok(IVec2::new(x, y))
    }
}
