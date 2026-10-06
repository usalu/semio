CREATE TABLE gltf_mesh (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_mesh_weight (
 id INTEGER PRIMARY KEY,
 mesh_id INTEGER NOT NULL REFERENCES gltf_mesh(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 value REAL,
 value_ieee754_bits INTEGER NOT NULL,
 value_numeric_class TEXT NOT NULL CHECK(value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_primitive (
 id INTEGER PRIMARY KEY,
 mesh_id INTEGER NOT NULL REFERENCES gltf_mesh(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 indices_index_high INTEGER CHECK(indices_index_high BETWEEN 0 AND 4294967295),
 indices_index_low INTEGER CHECK(indices_index_low BETWEEN 0 AND 4294967295),
 material_index_high INTEGER CHECK(material_index_high BETWEEN 0 AND 4294967295),
 material_index_low INTEGER CHECK(material_index_low BETWEEN 0 AND 4294967295),
 mode_high INTEGER CHECK(mode_high BETWEEN 0 AND 4294967295),
 mode_low INTEGER CHECK(mode_low BETWEEN 0 AND 4294967295),
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 CHECK((indices_index_high IS NULL)=(indices_index_low IS NULL)),
 CHECK((material_index_high IS NULL)=(material_index_low IS NULL)),
 CHECK((mode_high IS NULL)=(mode_low IS NULL))
);
CREATE TABLE gltf_primitive_attribute (
 id INTEGER PRIMARY KEY,
 primitive_id INTEGER NOT NULL REFERENCES gltf_primitive(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 semantic TEXT NOT NULL,
 accessor_index_high INTEGER NOT NULL CHECK(accessor_index_high BETWEEN 0 AND 4294967295),
 accessor_index_low INTEGER NOT NULL CHECK(accessor_index_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE gltf_morph_target (
 id INTEGER PRIMARY KEY,
 primitive_id INTEGER NOT NULL REFERENCES gltf_primitive(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0)
);
CREATE TABLE gltf_morph_attribute (
 id INTEGER PRIMARY KEY,
 target_id INTEGER NOT NULL REFERENCES gltf_morph_target(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 semantic TEXT NOT NULL,
 accessor_index_high INTEGER NOT NULL CHECK(accessor_index_high BETWEEN 0 AND 4294967295),
 accessor_index_low INTEGER NOT NULL CHECK(accessor_index_low BETWEEN 0 AND 4294967295)
);
