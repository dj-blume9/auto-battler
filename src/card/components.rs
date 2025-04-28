use bevy::prelude::{Bundle,Transform};
use bevy::prelude::Component;
use super::abilities::Ability;
use super::card_owner::CardOwner;

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    //pub transform: Transform,
   }

#[derive(Component)]
pub struct Card {
    pub name: String,
    pub health: Health,
    pub attack: Attack,
    pub speed: Speed,
    pub ability: AbilityComponent,
    pub owned_by: CardOwner,
}
#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub original: u32,
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
pub struct AbilityComponent {
    pub ability: Ability,
}

#[derive(Component)]
pub struct InCombat;

impl Card {
    pub fn apply_damage(&mut self, damage: u32) {
       self.health.current = self.health.current.saturating_sub(damage); 
    }

    pub fn get_attack(&self) -> u32 {
        self.attack.current
    }
}
