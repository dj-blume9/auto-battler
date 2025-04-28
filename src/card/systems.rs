use crate::card::card_owner::CardOwner;

use super::{abilities::Ability, components::*};
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_card(commands: &mut Commands, name: &str, owned: bool) {
    let mut rand = rand::rng();

    commands
        .spawn(CardBundle {
            card: Card {
                name: name.into(),
                health: Health {
                    current: rand.random_range(20..50),
                    original: 10,
                    max: 10,
                },
                attack: Attack {
                    current: rand.random_range(1..10),
                    original: 10,
                },
                speed: Speed {
                    current: 10,
                    original: 10,
                },
                ability: AbilityComponent {
                    ability: Ability::SingleTargetDamage,
                },
                owned_by: if owned {
                    CardOwner::Player
                } else {
                    CardOwner::Opponent
                },
            },
        })
        .insert(InCombat);
}

pub fn spawn_hand(mut commands: Commands) {
    for card in (1..4).enumerate() {
        let card_name = format!("Card {}", card.1.to_string());
        spawn_card(&mut commands, &card_name, true);
    }
}

pub fn spawn_enemy_hand(mut commands: Commands) {
    for card in (1..4).enumerate() {
        let card_name = format!("Card {}", card.1.to_string());
        spawn_card(&mut commands, &card_name, false);
    }
}
