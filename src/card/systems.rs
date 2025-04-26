use crate::card::card_owner::CardOwner;

use super::{abilities::Ability, components::*};
use bevy::prelude::*;

pub fn spawn_card(commands: &mut Commands, name: &str, owned: bool) {
    commands.spawn(CardBundle {
        name: CardName(name.into()),
        attack: Attack {
            current: 2,
            original: 10,
        },
        health: Health {
            current: 10,
            max: 10,
        },
        speed: Speed {
            current: 10,
            original: 10,
        },
        ability: AbilityComponent {
            ability: Ability::SingleTargetDamage,
        },
        owned_by: OwnedBy { owner: if owned { CardOwner::Player} else { CardOwner::Opponent}}
    }).insert(InCombat);
}

pub fn read_card(query: Query<&Attack, With<Health>>) {
    for attack in query.iter() {
        println!("{}", attack.current);
    }
}

pub fn spawn_hand(mut commands: Commands) {
    for (i, card) in (1..4).enumerate() {
        let card_name = format!("Card {}", card.to_string());
        spawn_card(&mut commands, &card_name, true);
    }
}

pub fn spawn_enemy_hand(mut commands: Commands) {
    for (i, card) in (1..4).enumerate() {
        let card_name = format!("Card {}", card.to_string());
        spawn_card(&mut commands, &card_name, false);
    }
}
