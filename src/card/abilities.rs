
use bevy::prelude::*;

#[derive(Clone)]
pub enum Ability {
    SingleTargetDamage,
    AoeDamage(u32, u32), // (damage, radius)
    Heal(u32),
}
