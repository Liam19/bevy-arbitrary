use crate::{Arbitrary, Result, Unstructured};

use bevy::prelude::Entity;

impl<'a> Arbitrary<'a> for bevy::prelude::Entity {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let bits = u64::arbitrary(u)?;
        Ok(Entity::from_bits(bits))
    }
}
