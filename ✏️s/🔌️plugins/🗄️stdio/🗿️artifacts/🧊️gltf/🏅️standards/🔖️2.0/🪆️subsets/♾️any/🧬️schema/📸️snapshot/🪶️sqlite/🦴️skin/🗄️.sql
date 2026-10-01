CREATE TABLE gltf_skin (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 inverse_bind_matrices_index_high INTEGER CHECK(inverse_bind_matrices_index_high BETWEEN 0 AND 4294967295),
 inverse_bind_matrices_index_low INTEGER CHECK(inverse_bind_matrices_index_low BETWEEN 0 AND 4294967295),
 skeleton_index_high INTEGER CHECK(skeleton_index_high BETWEEN 0 AND 4294967295),
 skeleton_index_low INTEGER CHECK(skeleton_index_low BETWEEN 0 AND 4294967295),
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 CHECK((inverse_bind_matrices_index_high IS NULL)=(inverse_bind_matrices_index_low IS NULL)),
 CHECK((skeleton_index_high IS NULL)=(skeleton_index_low IS NULL))
);
CREATE TABLE gltf_skin_joint (
 id INTEGER PRIMARY KEY,
 skin_id INTEGER NOT NULL REFERENCES gltf_skin(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 node_index_high INTEGER NOT NULL CHECK(node_index_high BETWEEN 0 AND 4294967295),
 node_index_low INTEGER NOT NULL CHECK(node_index_low BETWEEN 0 AND 4294967295)
);
