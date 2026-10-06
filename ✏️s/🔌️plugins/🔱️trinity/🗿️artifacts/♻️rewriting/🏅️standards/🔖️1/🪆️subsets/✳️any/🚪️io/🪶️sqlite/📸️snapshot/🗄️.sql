CREATE TABLE rewriting_document (id INTEGER PRIMARY KEY, working_graph_id INTEGER NOT NULL REFERENCES jack_document(id));
CREATE TABLE rewriting_layout (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES rewriting_document(id),
 map_key TEXT NOT NULL,
 x REAL,
 y REAL,
 x_ieee754_bits INTEGER NOT NULL,
 x_numeric_class TEXT NOT NULL,
 y_ieee754_bits INTEGER NOT NULL,
 y_numeric_class TEXT NOT NULL
);
CREATE TABLE rewriting_value (
 id INTEGER PRIMARY KEY,
 variant TEXT NOT NULL
);
CREATE TABLE rewriting_binding (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES rewriting_document(id),
 map_key TEXT NOT NULL,
 value_id INTEGER NOT NULL REFERENCES rewriting_value(id)
);
CREATE TABLE rewriting_boolean (
 id INTEGER PRIMARY KEY,
 value_id INTEGER NOT NULL REFERENCES rewriting_value(id),
 value INTEGER NOT NULL
);
CREATE TABLE rewriting_number (
 id INTEGER PRIMARY KEY,
 value_id INTEGER NOT NULL REFERENCES rewriting_value(id),
 value REAL,
 value_ieee754_bits INTEGER NOT NULL,
 value_numeric_class TEXT NOT NULL
);
CREATE TABLE rewriting_string (
 id INTEGER PRIMARY KEY,
 value_id INTEGER NOT NULL REFERENCES rewriting_value(id),
 value TEXT NOT NULL
);
CREATE TABLE rewriting_array_element (
 id INTEGER PRIMARY KEY,
 array_id INTEGER NOT NULL REFERENCES rewriting_value(id),
 ordinal INTEGER NOT NULL,
 value_id INTEGER NOT NULL REFERENCES rewriting_value(id)
);
CREATE TABLE rewriting_object_member (
 id INTEGER PRIMARY KEY,
 object_id INTEGER NOT NULL REFERENCES rewriting_value(id),
 ordinal INTEGER NOT NULL,
 map_key TEXT NOT NULL,
 value_id INTEGER NOT NULL REFERENCES rewriting_value(id)
);
CREATE TABLE rewriting_lhs (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES rewriting_document(id), pattern_id INTEGER NOT NULL REFERENCES rewriting_pattern(id), where_clause TEXT);
CREATE TABLE rewriting_rhs (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES rewriting_document(id));
CREATE TABLE rewriting_pattern (id INTEGER PRIMARY KEY, left_var TEXT NOT NULL, left_kind TEXT NOT NULL, edge_var TEXT, edge_kind TEXT, right_var TEXT, right_kind TEXT);
CREATE TABLE rewriting_create (id INTEGER PRIMARY KEY, rhs_id INTEGER NOT NULL REFERENCES rewriting_rhs(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), pattern_id INTEGER NOT NULL REFERENCES rewriting_pattern(id));
CREATE TABLE rewriting_merge (id INTEGER PRIMARY KEY, rhs_id INTEGER NOT NULL REFERENCES rewriting_rhs(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), pattern_id INTEGER NOT NULL REFERENCES rewriting_pattern(id));
CREATE TABLE rewriting_delete (id INTEGER PRIMARY KEY, rhs_id INTEGER NOT NULL REFERENCES rewriting_rhs(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), var TEXT NOT NULL);
CREATE TABLE rewriting_assignment (id INTEGER PRIMARY KEY, rhs_id INTEGER NOT NULL REFERENCES rewriting_rhs(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), var TEXT NOT NULL, prop TEXT NOT NULL, value_id INTEGER NOT NULL REFERENCES rewriting_value(id));
CREATE TABLE rewriting_parameter (id INTEGER PRIMARY KEY, rhs_id INTEGER NOT NULL REFERENCES rewriting_rhs(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), name TEXT NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('string','number','boolean')), value_id INTEGER NOT NULL REFERENCES rewriting_value(id));
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
