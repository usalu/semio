CREATE TABLE dag_document (
 id INTEGER PRIMARY KEY,
 schema TEXT NOT NULL
);
CREATE TABLE dag_content_child (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES dag_document(id),
 child_id TEXT NOT NULL,
 artifact_id TEXT NOT NULL,
 artifact_kind TEXT NOT NULL,
 standard TEXT NOT NULL,
 subset TEXT NOT NULL
);
