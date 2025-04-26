use bevy::prelude::Bundle;
use bevy::prelude::Component;
use super::abilities::Ability;
use super::card_owner::CardOwner;
use bevy::prelude::Mut;

#[derive(Bundle)]
pub struct CardBundle {
    pub name: CardName,
    pub attack: Attack,
    pub health: Health,
    pub speed: Speed,
    pub ability: AbilityComponent,
    pub owned_by: OwnedBy,
}

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

#[derive(Component)]
pub struct Attack {
    pub current: u32,
    pub original: u32,
}

#[derive(Component)]
pub struct Speed {
    pub current: u32,
    pub original: u32,
}

#[derive(Component)]
pub struct CardName(pub String);

#[derive(Component)]
pub struct AbilityComponent {
    pub ability: Ability,
}

#[derive(Component)]
pub struct InCombat;

#[derive(Component)]
pub struct OwnedBy {
    pub owner: CardOwner,
}

pub fn apply_damage(mut health: Mut<Health>, amount: u32) -> bool {
    health.current = health.current.saturating_sub(amount);
    health.current == 0
}
