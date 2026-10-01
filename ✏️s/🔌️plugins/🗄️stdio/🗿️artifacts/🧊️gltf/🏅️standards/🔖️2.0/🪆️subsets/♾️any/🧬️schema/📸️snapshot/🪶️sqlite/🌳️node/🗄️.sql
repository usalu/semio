CREATE TABLE gltf_node (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 name TEXT,
 mesh_index_high INTEGER CHECK(mesh_index_high BETWEEN 0 AND 4294967295),
 mesh_index_low INTEGER CHECK(mesh_index_low BETWEEN 0 AND 4294967295),
 camera_index_high INTEGER CHECK(camera_index_high BETWEEN 0 AND 4294967295),
 camera_index_low INTEGER CHECK(camera_index_low BETWEEN 0 AND 4294967295),
 skin_index_high INTEGER CHECK(skin_index_high BETWEEN 0 AND 4294967295),
 skin_index_low INTEGER CHECK(skin_index_low BETWEEN 0 AND 4294967295),
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 CHECK((mesh_index_high IS NULL)=(mesh_index_low IS NULL)),
 CHECK((camera_index_high IS NULL)=(camera_index_low IS NULL)),
 CHECK((skin_index_high IS NULL)=(skin_index_low IS NULL))
);
CREATE TABLE gltf_node_child (
 id INTEGER PRIMARY KEY,
 node_id INTEGER NOT NULL REFERENCES gltf_node(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 child_index_high INTEGER NOT NULL CHECK(child_index_high BETWEEN 0 AND 4294967295),
 child_index_low INTEGER NOT NULL CHECK(child_index_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE gltf_node_matrix (
 id INTEGER PRIMARY KEY REFERENCES gltf_node(id)
);
CREATE TABLE gltf_node_matrix_component (
 id INTEGER PRIMARY KEY,
 matrix_id INTEGER NOT NULL REFERENCES gltf_node_matrix(id),
 ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 15),
 value REAL,
 value_ieee754_bits INTEGER NOT NULL,
 value_numeric_class TEXT NOT NULL CHECK(value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_node_translation (
 id INTEGER PRIMARY KEY REFERENCES gltf_node(id)
);
CREATE TABLE gltf_node_translation_component (
 id INTEGER PRIMARY KEY,
 translation_id INTEGER NOT NULL REFERENCES gltf_node_translation(id),
 ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 2),
 value REAL,
 value_ieee754_bits INTEGER NOT NULL,
 value_numeric_class TEXT NOT NULL CHECK(value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_node_rotation (
 id INTEGER PRIMARY KEY REFERENCES gltf_node(id)
);
CREATE TABLE gltf_node_rotation_component (
 id INTEGER PRIMARY KEY,
 rotation_id INTEGER NOT NULL REFERENCES gltf_node_rotation(id),
 ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 3),
 value REAL,
 value_ieee754_bits INTEGER NOT NULL,
 value_numeric_class TEXT NOT NULL CHECK(value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_node_scale (
 id INTEGER PRIMARY KEY REFERENCES gltf_node(id)
);
CREATE TABLE gltf_node_scale_component (
 id INTEGER PRIMARY KEY,
 scale_id INTEGER NOT NULL REFERENCES gltf_node_scale(id),
 ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 2),
 value REAL,
 value_ieee754_bits INTEGER NOT NULL,
 value_numeric_class TEXT NOT NULL CHECK(value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_node_weight (
 id INTEGER PRIMARY KEY,
 node_id INTEGER NOT NULL REFERENCES gltf_node(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 value REAL,
 value_ieee754_bits INTEGER NOT NULL,
 value_numeric_class TEXT NOT NULL CHECK(value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
