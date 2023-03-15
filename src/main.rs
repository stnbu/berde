use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Eq, PartialEq, Debug, Serialize, Deserialize, Deref)]
pub struct Element(pub u64);

#[derive(Resource, Default)]
pub struct SerializedWorld(pub Vec<Vec<u8>>);

pub type SerializableElement<'a> = (&'a Element,);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SerializedWorld::default())
        .add_startup_system(spawn_masses)
        .add_system(serialize_world)
        .add_system(reload_world)
        .add_system(bevy::window::close_on_esc)
        .run();
}

// systems

pub fn serialize_world(
    keys: Res<Input<KeyCode>>,
    query: Query<SerializableElement>,
    mut serialized_world: ResMut<SerializedWorld>,
) {
    if keys.just_released(KeyCode::Down) {
        serialized_world.0 = vec![];
        for row in &query {
            match bincode::serialize(&row) {
                Ok(bin) => {
                    serialized_world.0.push(bin);
                }
                Err(err) => {
                    error!("Failed to serialize row: {err}");
                    serialized_world.0 = vec![];
                    return;
                }
            }
        }
        info!(
            "Sizes of {} serialized elements: {:?}",
            serialized_world.0.len(),
            serialized_world
                .0
                .iter()
                .map(|b| b.len())
                .collect::<Vec<_>>()
        )
    }
}

pub fn reload_world(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    drop_query: Query<Entity, With<Element>>,
    serialized_world: Res<SerializedWorld>,
) {
    if keys.just_released(KeyCode::Up) {
        drop_query.for_each(|e| commands.entity(e).despawn_recursive());
        for bin in serialized_world.0.iter() {
            dbg!(bin);
            // match bincode::deserialize::<SerializableElement>(bin) {
            //     Ok(row) => {
            //         info!("->> {row:?}");
            //     }
            //     Err(err) => {
            //         error!("xx");
            //     }
            // }
        }
    }
}

//

fn spawn_masses(mut commands: Commands) {
    commands
        .spawn(TransformBundle::default())
        .insert(Element(0));
}
