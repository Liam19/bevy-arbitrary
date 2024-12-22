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

// #[derive(Clone, Copy)]
// #[repr(C, align(8))]
// pub struct EntityShadow {
//     // Do not reorder the fields here. The ordering is explicitly used by repr(C)
//     // to make this struct equivalent to a u64.
//     #[cfg(target_endian = "little")]
//     index: u32,
//     generation: NonZero<u32>,
//     #[cfg(target_endian = "big")]
//     index: u32,
// }

// impl EntityShadow {
//     #[inline(always)]
//     pub const fn to_bits(self) -> u64 {
//         pack_into_u64(self.index, self.generation.get())
//     }
// }

// impl<'a> Arbitrary<'a> for EntityShadow {
//     fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
//         let index = u32::arbitrary(u)?;
//         let generation = u.int_in_range(1..=10_000)?;
//         let generation = NonZero::new(generation).unwrap();

//         let entity = EntityShadow { index, generation };

//         Ok(Entity::from_bits(bits))
//     }
// }

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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rand::*;

//     #[test]
//     fn test_entity() {
//         for _ in 0..1000 {
//             let estimated_size = core::mem::size_of::<u64>() * 1; // Estimate based on type size.
//             let mut rng = thread_rng();
//             let mut raw = vec![0u8; estimated_size];
//             rng.fill_bytes(&mut raw);

//             let mut unstructured = Unstructured::new(&raw);

//             let entity = <Entity as Arbitrary>::arbitrary(&mut unstructured)
//                 .expect("Failed to generate random instance");

//             println!("{entity}");
//         }
//     }
// }
