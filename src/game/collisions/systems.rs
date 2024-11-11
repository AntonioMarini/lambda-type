use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, ContactForceEvent};

pub fn check_collisions_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
){

    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}