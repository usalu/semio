CREATE TABLE block2_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL);
CREATE TABLE block2_kind (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block2_document(id), native_id TEXT NOT NULL, name TEXT NOT NULL, label TEXT NOT NULL, variant TEXT, description TEXT NOT NULL, icon TEXT, unit TEXT);
CREATE TABLE block2_presentation (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block2_document(id), shape TEXT,
 radius REAL, radius_ieee754_bits INTEGER, radius_numeric_class TEXT,
 width REAL, width_ieee754_bits INTEGER, width_numeric_class TEXT,
 height REAL, height_ieee754_bits INTEGER, height_numeric_class TEXT, color TEXT, icon_kind TEXT);
CREATE TABLE block2_handle_kind (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block2_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, name TEXT NOT NULL, label TEXT NOT NULL, color TEXT NOT NULL, default_wire_kind TEXT NOT NULL);
CREATE TABLE block2_handle (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block2_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, handle_kind TEXT NOT NULL,
 angle REAL, angle_ieee754_bits INTEGER NOT NULL, angle_numeric_class TEXT NOT NULL,
 radius REAL, radius_ieee754_bits INTEGER NOT NULL, radius_numeric_class TEXT NOT NULL);
CREATE TABLE block2_compatibility (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block2_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, source TEXT NOT NULL, target TEXT NOT NULL, bidirectional INTEGER NOT NULL);
CREATE TABLE block2_attribute (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block2_document(id), ordinal INTEGER NOT NULL, key TEXT NOT NULL, value TEXT NOT NULL, definition TEXT);
CREATE TABLE block2_author (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block2_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, name TEXT NOT NULL, email TEXT);
CREATE TABLE block2_camera2d (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block2_document(id),
 x REAL, x_ieee754_bits INTEGER NOT NULL, x_numeric_class TEXT NOT NULL,
 y REAL, y_ieee754_bits INTEGER NOT NULL, y_numeric_class TEXT NOT NULL,
 zoom REAL, zoom_ieee754_bits INTEGER NOT NULL, zoom_numeric_class TEXT NOT NULL);
CREATE TABLE block2_meta (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block2_document(id), description TEXT NOT NULL);
