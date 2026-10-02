CREATE TABLE playbook_document (id INTEGER PRIMARY KEY, schema_id TEXT NOT NULL, authored_id TEXT NOT NULL, version TEXT NOT NULL, title TEXT);
CREATE TABLE playbook_narrative_child (id INTEGER PRIMARY KEY REFERENCES playbook_document(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL);
CREATE TABLE playbook_flow_child (id INTEGER PRIMARY KEY REFERENCES playbook_document(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL);
