CREATE TABLE block5_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL);
CREATE TABLE block5_kind (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id), native_id TEXT NOT NULL, name TEXT NOT NULL, label TEXT NOT NULL, variant TEXT, description TEXT NOT NULL, icon TEXT, unit TEXT);
CREATE TABLE block5_part2d (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id), shape TEXT,
 radius REAL, radius_ieee754_bits INTEGER, radius_numeric_class TEXT,
 width REAL, width_ieee754_bits INTEGER, width_numeric_class TEXT,
 height REAL, height_ieee754_bits INTEGER, height_numeric_class TEXT, color TEXT, icon_kind TEXT);
CREATE TABLE block5_part3d (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id),
 orientation_x REAL, orientation_x_ieee754_bits INTEGER, orientation_x_numeric_class TEXT,
 orientation_y REAL, orientation_y_ieee754_bits INTEGER, orientation_y_numeric_class TEXT,
 orientation_z REAL, orientation_z_ieee754_bits INTEGER, orientation_z_numeric_class TEXT,
 orientation_w REAL, orientation_w_ieee754_bits INTEGER, orientation_w_numeric_class TEXT,
 scale_x REAL, scale_x_ieee754_bits INTEGER, scale_x_numeric_class TEXT,
 scale_y REAL, scale_y_ieee754_bits INTEGER, scale_y_numeric_class TEXT,
 scale_z REAL, scale_z_ieee754_bits INTEGER, scale_z_numeric_class TEXT);
CREATE TABLE block5_representation (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, name TEXT NOT NULL, mesh_url TEXT, lod TEXT, description TEXT NOT NULL);
CREATE TABLE block5_representation_tag (id INTEGER PRIMARY KEY, representation_id INTEGER NOT NULL REFERENCES block5_representation(id), ordinal INTEGER NOT NULL, value TEXT NOT NULL);
CREATE TABLE block5_representation_attribute (id INTEGER PRIMARY KEY, representation_id INTEGER NOT NULL REFERENCES block5_representation(id), ordinal INTEGER NOT NULL, key TEXT NOT NULL, value TEXT NOT NULL, definition TEXT);
CREATE TABLE block5_grip_kind (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, name TEXT NOT NULL, label TEXT NOT NULL, color TEXT NOT NULL, default_rope_kind TEXT NOT NULL);
CREATE TABLE block5_grip (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, grip_kind TEXT NOT NULL,
 angle REAL, angle_ieee754_bits INTEGER NOT NULL, angle_numeric_class TEXT NOT NULL,
 radius2d REAL, radius2d_ieee754_bits INTEGER NOT NULL, radius2d_numeric_class TEXT NOT NULL,
 position_x REAL, position_x_ieee754_bits INTEGER NOT NULL, position_x_numeric_class TEXT NOT NULL,
 position_y REAL, position_y_ieee754_bits INTEGER NOT NULL, position_y_numeric_class TEXT NOT NULL,
 position_z REAL, position_z_ieee754_bits INTEGER NOT NULL, position_z_numeric_class TEXT NOT NULL,
 direction_x REAL, direction_x_ieee754_bits INTEGER NOT NULL, direction_x_numeric_class TEXT NOT NULL,
 direction_y REAL, direction_y_ieee754_bits INTEGER NOT NULL, direction_y_numeric_class TEXT NOT NULL,
 direction_z REAL, direction_z_ieee754_bits INTEGER NOT NULL, direction_z_numeric_class TEXT NOT NULL,
 radius3d REAL, radius3d_ieee754_bits INTEGER NOT NULL, radius3d_numeric_class TEXT NOT NULL);
CREATE TABLE block5_compatibility (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, source TEXT NOT NULL, target TEXT NOT NULL, bidirectional INTEGER NOT NULL);
CREATE TABLE block5_attribute (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id), ordinal INTEGER NOT NULL, key TEXT NOT NULL, value TEXT NOT NULL, definition TEXT);
CREATE TABLE block5_author (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id), ordinal INTEGER NOT NULL, native_id TEXT NOT NULL, name TEXT NOT NULL, email TEXT);
CREATE TABLE block5_camera2d (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id),
 x REAL, x_ieee754_bits INTEGER NOT NULL, x_numeric_class TEXT NOT NULL,
 y REAL, y_ieee754_bits INTEGER NOT NULL, y_numeric_class TEXT NOT NULL,
 zoom REAL, zoom_ieee754_bits INTEGER NOT NULL, zoom_numeric_class TEXT NOT NULL);
CREATE TABLE block5_camera3d (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id),
 position_x REAL, position_x_ieee754_bits INTEGER NOT NULL, position_x_numeric_class TEXT NOT NULL,
 position_y REAL, position_y_ieee754_bits INTEGER NOT NULL, position_y_numeric_class TEXT NOT NULL,
 position_z REAL, position_z_ieee754_bits INTEGER NOT NULL, position_z_numeric_class TEXT NOT NULL,
 target_x REAL, target_x_ieee754_bits INTEGER NOT NULL, target_x_numeric_class TEXT NOT NULL,
 target_y REAL, target_y_ieee754_bits INTEGER NOT NULL, target_y_numeric_class TEXT NOT NULL,
 target_z REAL, target_z_ieee754_bits INTEGER NOT NULL, target_z_numeric_class TEXT NOT NULL,
 zoom REAL, zoom_ieee754_bits INTEGER NOT NULL, zoom_numeric_class TEXT NOT NULL);
CREATE TABLE block5_meta (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES block5_document(id), description TEXT NOT NULL);

