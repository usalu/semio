CREATE TABLE gltf_document (
 id INTEGER PRIMARY KEY CHECK(id=1),
 schema TEXT NOT NULL,
 source_form TEXT NOT NULL CHECK(source_form IN ('json','glb')),
 scene_index_high INTEGER CHECK(scene_index_high BETWEEN 0 AND 4294967295),
 scene_index_low INTEGER CHECK(scene_index_low BETWEEN 0 AND 4294967295),
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 CHECK((scene_index_high IS NULL)=(scene_index_low IS NULL))
);
CREATE TABLE gltf_asset (
 id INTEGER PRIMARY KEY CHECK(id=1),
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 version TEXT NOT NULL,
 generator TEXT,
 copyright TEXT,
 min_version TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_scene (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_scene_node (
 id INTEGER PRIMARY KEY,
 scene_id INTEGER NOT NULL REFERENCES gltf_scene(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 node_index_high INTEGER NOT NULL CHECK(node_index_high BETWEEN 0 AND 4294967295),
 node_index_low INTEGER NOT NULL CHECK(node_index_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE gltf_extension_used (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 name TEXT NOT NULL
);
CREATE TABLE gltf_extension_required (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 name TEXT NOT NULL
);
CREATE TABLE gltf_resolved_buffer (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0)
);
CREATE TABLE gltf_resolved_byte (
 id INTEGER PRIMARY KEY,
 buffer_id INTEGER NOT NULL REFERENCES gltf_resolved_buffer(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)
);
