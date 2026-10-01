CREATE TABLE gltf_animation (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_animation_sampler (
 id INTEGER PRIMARY KEY,
 animation_id INTEGER NOT NULL REFERENCES gltf_animation(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 input_index_high INTEGER NOT NULL CHECK(input_index_high BETWEEN 0 AND 4294967295),
 input_index_low INTEGER NOT NULL CHECK(input_index_low BETWEEN 0 AND 4294967295),
 output_index_high INTEGER NOT NULL CHECK(output_index_high BETWEEN 0 AND 4294967295),
 output_index_low INTEGER NOT NULL CHECK(output_index_low BETWEEN 0 AND 4294967295),
 interpolation TEXT NOT NULL CHECK(interpolation IN ('LINEAR','STEP','CUBICSPLINE')),
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_animation_channel (
 id INTEGER PRIMARY KEY,
 animation_id INTEGER NOT NULL REFERENCES gltf_animation(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 sampler_index_high INTEGER NOT NULL CHECK(sampler_index_high BETWEEN 0 AND 4294967295),
 sampler_index_low INTEGER NOT NULL CHECK(sampler_index_low BETWEEN 0 AND 4294967295),
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_animation_channel_target (
 id INTEGER PRIMARY KEY REFERENCES gltf_animation_channel(id),
 node_index_high INTEGER CHECK(node_index_high BETWEEN 0 AND 4294967295),
 node_index_low INTEGER CHECK(node_index_low BETWEEN 0 AND 4294967295),
 path TEXT NOT NULL CHECK(path IN ('translation','rotation','scale','weights')),
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 CHECK((node_index_high IS NULL)=(node_index_low IS NULL))
);
