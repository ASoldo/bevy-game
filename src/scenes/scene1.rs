use bevy::prelude::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SceneData {
    pub entities: Vec<EntityData>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityData {
    pub name: String,
    pub position: Vec2,
}

pub fn generate_scene_json() -> String {
    let scene = SceneData {
        entities: vec![
            EntityData {
                name: "Entity1".to_string(),
                position: Vec2::new(0.0, 0.0),
            },
            EntityData {
                name: "Entity2".to_string(),
                position: Vec2::new(10.0, 0.0),
            },
        ],
    };

    serde_json::to_string(&scene).expect("Failed to serialize scene")
}
