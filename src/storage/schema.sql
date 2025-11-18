-- Enable foriegn keys and WAL mode for better concurrency 

PRAGMA foriegn_keys = ON;
PRAGMA journal_mode = ON;

-- Entities table
CREATE TABLE IF NOT EXISTS entities (
    id TEXT PRIMARY KEY,                -- UUID
    type TEXT NOT NULL,                 -- person, org, object, concept
    name TEXT NOT NULL,
    properties TEXT NOT NULL,           -- JSON object
    first_seen INTEGER NOT NULL,        -- unix timestamp
    last_updated INTEGER NOT NULL,
    metadata TEXT NOT NULL
);

CREATE INDEX idx_entities_type ON entities(type);
CREATE INDEX idx_entities_name ON entities(name);
CREATE INDEX idx_entities_type_name ON entities(type, name);

-- Actions table
CREATE TABLE IF NOT EXISTS actions (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL,
    actor_entity_id TEXT NOT NULL,
    object_entity_id TEXT,
    timestamp INTEGER NOT NULL,
    properties TEXT NOT NULL,
    FOREIGN KEY (actor_entity_id) REFERENCES entities(id),
    FOREIGN KEY (object_entity_id) REFERENCES entities(id)
);

-- Topics table
CREATE TABLE IF NOT EXISTS topics (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    related_topics TEXT NOT NULL,             -- JSON array
    importance_score REAL NOT NULL
);

-- Relationships table
CREATE TABLE IF NOT EXISTS relationships (
    id TEXT PRIMARY KEY,
    from_entity_id TEXT NOT NULL,
    relationship_type TEXT NOT NULL,    -- works_at, owns, parent_of
    to_entity_id TEXT NOT NULL,
    strength REAL NOT NULL,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (from_entity_id) REFERENCES entities(id),
    FOREIGN KEY (to_entity_id) REFERENCES entities(id)
);

-- Knowledge entries (master table)
CREATE TABLE IF NOT EXISTS knowledge_entries (
    id TEXT PRIMARY KEY,
    source_id TEXT NOT NULL,
    entities TEXT NOT NULL,
    actions TEXT NOT NULL,
    topics TEXT NOT NULL,
    raw_input TEXT NOT NULL,
    llm_extracted TEXT NOT NULL,
    confidence REAL NOT NULL,
    timestamp INTEGER NOT NULL,
    indexed INTEGER NOT NULL DEFAULT 0
);
