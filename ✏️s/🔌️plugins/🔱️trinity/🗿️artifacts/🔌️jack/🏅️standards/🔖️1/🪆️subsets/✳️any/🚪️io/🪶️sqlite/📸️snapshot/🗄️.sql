CREATE TABLE jack_document (
 id INTEGER PRIMARY KEY,
 schema TEXT NOT NULL,
 name TEXT NOT NULL,
 manifest_id TEXT,
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
CREATE TABLE jack_node_kind (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES jack_document(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL
);
CREATE TABLE jack_edge_kind (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES jack_document(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL
);
CREATE TABLE jack_port_kind (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES jack_document(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL,
 direction TEXT NOT NULL
);
CREATE TABLE jack_node_kind_port (
 id INTEGER PRIMARY KEY,
 node_kind_id INTEGER NOT NULL REFERENCES jack_node_kind(id),
 ordinal INTEGER NOT NULL,
 port_kind TEXT NOT NULL
);
CREATE TABLE jack_value_type (
 id INTEGER PRIMARY KEY,
 variant TEXT NOT NULL
);
CREATE TABLE jack_value_type_list (
 id INTEGER PRIMARY KEY,
 value_type_id INTEGER NOT NULL REFERENCES jack_value_type(id),
 child_type_id INTEGER NOT NULL REFERENCES jack_value_type(id)
);
CREATE TABLE jack_value_type_schema (
 id INTEGER PRIMARY KEY,
 value_type_id INTEGER NOT NULL REFERENCES jack_value_type(id),
 schema TEXT NOT NULL
);
CREATE TABLE jack_node_property (
 id INTEGER PRIMARY KEY,
 node_kind_id INTEGER NOT NULL REFERENCES jack_node_kind(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL,
 property_kind TEXT NOT NULL,
 expression TEXT,
 value_type_id INTEGER NOT NULL REFERENCES jack_value_type(id)
);
CREATE TABLE jack_edge_property (
 id INTEGER PRIMARY KEY,
 edge_kind_id INTEGER NOT NULL REFERENCES jack_edge_kind(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL,
 property_kind TEXT NOT NULL,
 expression TEXT,
 value_type_id INTEGER NOT NULL REFERENCES jack_value_type(id)
);
CREATE TABLE jack_port_property (
 id INTEGER PRIMARY KEY,
 port_kind_id INTEGER NOT NULL REFERENCES jack_port_kind(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL,
 property_kind TEXT NOT NULL,
 expression TEXT,
 value_type_id INTEGER NOT NULL REFERENCES jack_value_type(id)
);
