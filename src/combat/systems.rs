use crate::card::abilities::Ability;
use crate::card::card_owner::CardOwner;
use crate::card::components::*;
use bevy::prelude::*;

pub fn resolve_combat(
    mut combat_cards: Query<(Entity, &Attack, &mut Health, &AbilityComponent, &OwnedBy)>,
    mut commands: Commands,
) {
    let mut player_cards: Vec<Entity> = Vec::new();
    let mut enemy_cards: Vec<Entity> = Vec::new();

    for (entity, _attack, mut _health, _ability, owned_by) in combat_cards.iter() {
        match owned_by.owner {
            CardOwner::Player => player_cards.push(entity),
            CardOwner::Opponent => enemy_cards.push(entity),
        }
    }

    println!("---Player Cards---");
    for player_entity in player_cards.iter() {
        if let Ok((_entity, attack, health, _ability, _owned_by)) = combat_cards.get(*player_entity)
        {
            println!(
                "Player Attack: {}, Health: {}",
                attack.current, health.current
            );
        }
    }

    println!("---Enemy Cards---");
    for enemy_entity in enemy_cards.iter() {
        if let Ok((_entity, attack, health, _ability, _owned_by)) = combat_cards.get(*enemy_entity)
        {
            println!(
                "Enemy Attack: {}, Health: {}",
                attack.current, health.current
            );
        }
    }

    let mut current_card_index = 0;
    while !player_cards.is_empty() && !enemy_cards.is_empty() {
        // Reset index if we go past available cards
        if current_card_index >= player_cards.len() || current_card_index >= enemy_cards.len() {
            current_card_index = 0;
        }

        let player_entity = player_cards[current_card_index];
        let opponent_entity = enemy_cards[current_card_index];

        if let Ok([mut player_card, mut enemy_card]) =
            combat_cards.get_many_mut([player_entity, opponent_entity])
        {
            let (pc_entity, pc_attack, mut pc_health, _pc_ability, _pc_owned_by) = player_card;
            let (ec_entity, ec_attack, mut ec_health, _ec_ability, _ec_owned_by) = enemy_card;

            println!(
                "Card {} is attacking enemy {}",
                pc_entity.index(),
                ec_entity.index()
            );

            ec_health.current = ec_health.current.saturating_sub(pc_attack.current);
            pc_health.current = pc_health.current.saturating_sub(ec_attack.current);

            println!(
                "Card Health: {} Enemy Card Health: {}",
                pc_health.current, ec_health.current
            );

            // Track if they die
            let mut player_dead = false;
            let mut enemy_dead = false;

            if ec_health.current == 0 {
                commands.entity(ec_entity).despawn();
                enemy_dead = true;
            }
            if pc_health.current == 0 {
                commands.entity(pc_entity).despawn();
                player_dead = true;
            }

            // Remove dead from Vecs
            if enemy_dead {
                enemy_cards.remove(current_card_index);
            }
            if player_dead {
                player_cards.remove(current_card_index);
            } else {
                // Only move to next card if attacker survived
                current_card_index += 1;
            }
        } else {
            // If for some reason we cannot get the cards (maybe despawned?), move to next
            current_card_index += 1;
        }
    }
}
