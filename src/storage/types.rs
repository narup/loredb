use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub entity_type: String,
    pub name: String,
    pub properties: serde_json::Value,
    pub first_seen: i64,
    pub last_updated: i64,
    pub metadata: serde_json::Value,
}

impl Entity {
    pub fn new(
        id: String,
        entity_type: String,
        name: String,
        properties: serde_json::Value,
        metadata: serde_json::Value,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id,
            entity_type,
            name,
            properties,
            first_seen: now,
            last_updated: now,
            metadata,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub action_type: String,
    pub actor_entity_id: String,
    pub object_entity_id: String,
    pub timestamp: i64,
    pub properties: serde_json::Value,
}

impl Action {
    pub fn new(
        id: String,
        action_type: String,
        actor_entity_id: String,
        object_entity_id: String,
        properties: serde_json::Value,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id,
            action_type,
            actor_entity_id,
            object_entity_id,
            timestamp: now,
            properties,
        }
    }
}
