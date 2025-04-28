use std::thread::current;

//use crate::card::abilities::Ability;
use crate::card::card_owner::CardOwner;
use crate::card::components::*;
use bevy::prelude::*;

pub fn resolve_combat(
    mut cards_in_combat: Query<(Entity, &mut Card), With<InCombat>>,
    mut commands: Commands,
) {
    let mut player_cards: Vec<Entity> = Vec::new();
    let mut opponent_cards: Vec<Entity> = Vec::new();

    for (entity, card) in cards_in_combat.iter() {
        match card.owned_by {
            CardOwner::Player => player_cards.push(entity),
            CardOwner::Opponent => opponent_cards.push(entity),
        }
    }

    let mut current_card_index = 0;
    while !player_cards.is_empty() && !opponent_cards.is_empty() {
        if current_card_index >= player_cards.len() || current_card_index >= opponent_cards.len() {
            current_card_index = 0;
        }

        println!("Current Index: {}", current_card_index);
        println!("Total Player Cards: {}", player_cards.len());
        println!("Total Opponent Cards: {}", opponent_cards.len());
        let pc_entity = player_cards[current_card_index];
        let oc_entity = opponent_cards[current_card_index];

        let mut pc_attack = 0;
        let mut oc_attack = 0;

        if let Ok(player_card) = cards_in_combat.get(pc_entity) {
            pc_attack = player_card.1.get_attack();
        }

        if let Ok(opponent_card) = cards_in_combat.get(oc_entity) {
            oc_attack = opponent_card.1.get_attack();
        }

        if let Ok(mut player_card) = cards_in_combat.get_mut(pc_entity) {
            player_card.1.apply_damage(oc_attack);
            println!("Attacking player card");
        }

        if let Ok(mut opponent_card) = cards_in_combat.get_mut(oc_entity) {
            opponent_card.1.apply_damage(pc_attack);
            println!("Attacking opponent card");
        }

        let mut player_dead = false;
        let mut opponent_dead = false;

        if let Ok(player_card) = cards_in_combat.get(pc_entity) {
            if player_card.1.health.current <= 0 {
                commands.entity(pc_entity).despawn();
                player_dead = true;
                println!("Removing player card");
            }
        }

        if let Ok(opponent_card) = cards_in_combat.get(oc_entity) {
            if opponent_card.1.health.current <= 0 {
                commands.entity(oc_entity).despawn();
                opponent_dead = true;
                println!("Removing opponent card");
            }
        }

        let mut removed = false;
        if player_dead {
            player_cards.remove(current_card_index);
            removed = true;
        }
        if opponent_dead {
            opponent_cards.remove(current_card_index);
            removed = true;
        }

        if !removed {
            println!("Increasing card index");
            current_card_index += 1;
        }
    }
    println!("Combat Resolved!");
    if !player_cards.is_empty() && opponent_cards.is_empty(){
        println!("Player wins!");
    } else if !opponent_cards.is_empty() && player_cards.is_empty(){
        println!("Player wins!");
    } else {
        println!("Draw!");
    }
}
