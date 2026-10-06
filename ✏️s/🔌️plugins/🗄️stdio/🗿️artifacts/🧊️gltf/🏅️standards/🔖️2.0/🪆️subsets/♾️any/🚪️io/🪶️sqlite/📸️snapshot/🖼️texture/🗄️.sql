CREATE TABLE gltf_texture (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 sampler_index_high INTEGER CHECK(sampler_index_high BETWEEN 0 AND 4294967295),
 sampler_index_low INTEGER CHECK(sampler_index_low BETWEEN 0 AND 4294967295),
 source_index_high INTEGER CHECK(source_index_high BETWEEN 0 AND 4294967295),
 source_index_low INTEGER CHECK(source_index_low BETWEEN 0 AND 4294967295),
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 CHECK((sampler_index_high IS NULL)=(sampler_index_low IS NULL)),
 CHECK((source_index_high IS NULL)=(source_index_low IS NULL))
);
CREATE TABLE gltf_image (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 uri TEXT,
 mime_type TEXT,
 buffer_view_index_high INTEGER CHECK(buffer_view_index_high BETWEEN 0 AND 4294967295),
 buffer_view_index_low INTEGER CHECK(buffer_view_index_low BETWEEN 0 AND 4294967295),
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 CHECK((buffer_view_index_high IS NULL)=(buffer_view_index_low IS NULL))
);
CREATE TABLE gltf_sampler (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 mag_filter_high INTEGER CHECK(mag_filter_high BETWEEN 0 AND 4294967295),
 mag_filter_low INTEGER CHECK(mag_filter_low BETWEEN 0 AND 4294967295),
 min_filter_high INTEGER CHECK(min_filter_high BETWEEN 0 AND 4294967295),
 min_filter_low INTEGER CHECK(min_filter_low BETWEEN 0 AND 4294967295),
 wrap_s_high INTEGER NOT NULL CHECK(wrap_s_high BETWEEN 0 AND 4294967295),
 wrap_s_low INTEGER NOT NULL CHECK(wrap_s_low BETWEEN 0 AND 4294967295),
 wrap_t_high INTEGER NOT NULL CHECK(wrap_t_high BETWEEN 0 AND 4294967295),
 wrap_t_low INTEGER NOT NULL CHECK(wrap_t_low BETWEEN 0 AND 4294967295),
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 CHECK((mag_filter_high IS NULL)=(mag_filter_low IS NULL)),
 CHECK((min_filter_high IS NULL)=(min_filter_low IS NULL))
);
