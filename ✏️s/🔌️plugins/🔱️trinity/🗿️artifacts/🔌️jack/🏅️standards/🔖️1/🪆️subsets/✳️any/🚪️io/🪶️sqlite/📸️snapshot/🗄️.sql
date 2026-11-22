CREATE TABLE jack_document (
 id INTEGER PRIMARY KEY,
 schema TEXT NOT NULL,
 name TEXT NOT NULL,
 manifest_id TEXT NOT NULL,
 root_node_id TEXT,
 query TEXT NOT NULL
);
CREATE TABLE jack_camera (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES jack_document(id),
 x REAL,
 y REAL,
 zoom REAL,
 x_ieee754_bits INTEGER NOT NULL,
 x_numeric_class TEXT NOT NULL,
 y_ieee754_bits INTEGER NOT NULL,
 y_numeric_class TEXT NOT NULL,
 zoom_ieee754_bits INTEGER NOT NULL,
 zoom_numeric_class TEXT NOT NULL
);
CREATE TABLE jack_content_child (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES jack_document(id),
 child_id TEXT NOT NULL,
 artifact_id TEXT NOT NULL,
 artifact_kind TEXT NOT NULL,
 standard TEXT NOT NULL,
 subset TEXT NOT NULL
);
