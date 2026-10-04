CREATE TABLE block3_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL);
CREATE TABLE block3_kind (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block3_document(id), native_id TEXT NOT NULL, name TEXT NOT NULL, label TEXT NOT NULL, variant TEXT, description TEXT NOT NULL, icon TEXT, unit TEXT);
CREATE TABLE block3_catalog_child (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block3_document(id), child_id TEXT NOT NULL, target_artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL);
CREATE TABLE block3_representation (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block3_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, name TEXT NOT NULL, mesh_url TEXT, lod TEXT, description TEXT NOT NULL);
CREATE TABLE block3_representation_tag (id INTEGER PRIMARY KEY, representation_id INTEGER NOT NULL REFERENCES block3_representation(id), ordinal INTEGER NOT NULL, value TEXT NOT NULL);
CREATE TABLE block3_representation_attribute (id INTEGER PRIMARY KEY, representation_id INTEGER NOT NULL REFERENCES block3_representation(id), ordinal INTEGER NOT NULL, key TEXT NOT NULL, value TEXT NOT NULL, definition TEXT);
CREATE TABLE block3_vortex_kind_extra (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block3_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, name TEXT NOT NULL, label TEXT NOT NULL, color TEXT NOT NULL, default_cable_kind TEXT NOT NULL);
CREATE TABLE block3_vortex (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block3_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, vortex_kind TEXT NOT NULL,
 position_x REAL, position_x_ieee754_bits INTEGER NOT NULL, position_x_numeric_class TEXT NOT NULL,
 position_y REAL, position_y_ieee754_bits INTEGER NOT NULL, position_y_numeric_class TEXT NOT NULL,
 position_z REAL, position_z_ieee754_bits INTEGER NOT NULL, position_z_numeric_class TEXT NOT NULL,
 direction_x REAL, direction_x_ieee754_bits INTEGER NOT NULL, direction_x_numeric_class TEXT NOT NULL,
 direction_y REAL, direction_y_ieee754_bits INTEGER NOT NULL, direction_y_numeric_class TEXT NOT NULL,
 direction_z REAL, direction_z_ieee754_bits INTEGER NOT NULL, direction_z_numeric_class TEXT NOT NULL,
 radius REAL, radius_ieee754_bits INTEGER NOT NULL, radius_numeric_class TEXT NOT NULL, label TEXT);
CREATE TABLE block3_compatibility (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block3_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, source TEXT NOT NULL, target TEXT NOT NULL, bidirectional INTEGER NOT NULL);
CREATE TABLE block3_attribute (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block3_document(id), ordinal INTEGER NOT NULL, key TEXT NOT NULL, value TEXT NOT NULL, definition TEXT);
CREATE TABLE block3_author (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block3_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, name TEXT NOT NULL, email TEXT);
CREATE TABLE block3_camera3d (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block3_document(id),
 position_x REAL, position_x_ieee754_bits INTEGER NOT NULL, position_x_numeric_class TEXT NOT NULL,
 position_y REAL, position_y_ieee754_bits INTEGER NOT NULL, position_y_numeric_class TEXT NOT NULL,
 position_z REAL, position_z_ieee754_bits INTEGER NOT NULL, position_z_numeric_class TEXT NOT NULL,
 target_x REAL, target_x_ieee754_bits INTEGER NOT NULL, target_x_numeric_class TEXT NOT NULL,
 target_y REAL, target_y_ieee754_bits INTEGER NOT NULL, target_y_numeric_class TEXT NOT NULL,
 target_z REAL, target_z_ieee754_bits INTEGER NOT NULL, target_z_numeric_class TEXT NOT NULL,
 zoom REAL, zoom_ieee754_bits INTEGER NOT NULL, zoom_numeric_class TEXT NOT NULL);
CREATE TABLE block3_meta (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block3_document(id), description TEXT NOT NULL);
