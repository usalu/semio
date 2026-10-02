CREATE TABLE procedure_document (id INTEGER PRIMARY KEY, schema_id TEXT NOT NULL);
CREATE TABLE procedure_flow_child (id INTEGER PRIMARY KEY REFERENCES procedure_document(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL);
CREATE TABLE procedure_text_child (id INTEGER PRIMARY KEY REFERENCES procedure_document(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL);
