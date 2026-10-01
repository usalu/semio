CREATE TABLE gltf_buffer (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 byte_length_high INTEGER NOT NULL CHECK(byte_length_high BETWEEN 0 AND 4294967295),
 byte_length_low INTEGER NOT NULL CHECK(byte_length_low BETWEEN 0 AND 4294967295),
 uri TEXT,
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_buffer_view (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 buffer_index_high INTEGER NOT NULL CHECK(buffer_index_high BETWEEN 0 AND 4294967295),
 buffer_index_low INTEGER NOT NULL CHECK(buffer_index_low BETWEEN 0 AND 4294967295),
 byte_offset_high INTEGER NOT NULL CHECK(byte_offset_high BETWEEN 0 AND 4294967295),
 byte_offset_low INTEGER NOT NULL CHECK(byte_offset_low BETWEEN 0 AND 4294967295),
 byte_length_high INTEGER NOT NULL CHECK(byte_length_high BETWEEN 0 AND 4294967295),
 byte_length_low INTEGER NOT NULL CHECK(byte_length_low BETWEEN 0 AND 4294967295),
 byte_stride_high INTEGER CHECK(byte_stride_high BETWEEN 0 AND 4294967295),
 byte_stride_low INTEGER CHECK(byte_stride_low BETWEEN 0 AND 4294967295),
 target_high INTEGER CHECK(target_high BETWEEN 0 AND 4294967295),
 target_low INTEGER CHECK(target_low BETWEEN 0 AND 4294967295),
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 CHECK((byte_stride_high IS NULL)=(byte_stride_low IS NULL)),
 CHECK((target_high IS NULL)=(target_low IS NULL))
);
CREATE TABLE gltf_accessor (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 buffer_view_index_high INTEGER CHECK(buffer_view_index_high BETWEEN 0 AND 4294967295),
 buffer_view_index_low INTEGER CHECK(buffer_view_index_low BETWEEN 0 AND 4294967295),
 byte_offset_high INTEGER NOT NULL CHECK(byte_offset_high BETWEEN 0 AND 4294967295),
 byte_offset_low INTEGER NOT NULL CHECK(byte_offset_low BETWEEN 0 AND 4294967295),
 component_type INTEGER NOT NULL CHECK(component_type IN (5120,5121,5122,5123,5125,5126)),
 normalized INTEGER NOT NULL CHECK(normalized IN (0,1)),
 count_high INTEGER NOT NULL CHECK(count_high BETWEEN 0 AND 4294967295),
 count_low INTEGER NOT NULL CHECK(count_low BETWEEN 0 AND 4294967295),
 kind TEXT NOT NULL CHECK(kind IN ('SCALAR','VEC2','VEC3','VEC4','MAT2','MAT3','MAT4')),
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 CHECK((buffer_view_index_high IS NULL)=(buffer_view_index_low IS NULL))
);
CREATE TABLE gltf_accessor_max (
 id INTEGER PRIMARY KEY REFERENCES gltf_accessor(id)
);
CREATE TABLE gltf_accessor_max_component (
 id INTEGER PRIMARY KEY,
 accessor_id INTEGER NOT NULL REFERENCES gltf_accessor_max(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 value REAL,
 value_ieee754_bits INTEGER NOT NULL,
 value_numeric_class TEXT NOT NULL CHECK(value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_accessor_min (
 id INTEGER PRIMARY KEY REFERENCES gltf_accessor(id)
);
CREATE TABLE gltf_accessor_min_component (
 id INTEGER PRIMARY KEY,
 accessor_id INTEGER NOT NULL REFERENCES gltf_accessor_min(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 value REAL,
 value_ieee754_bits INTEGER NOT NULL,
 value_numeric_class TEXT NOT NULL CHECK(value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_sparse_accessor (
 id INTEGER PRIMARY KEY REFERENCES gltf_accessor(id),
 count_high INTEGER NOT NULL CHECK(count_high BETWEEN 0 AND 4294967295),
 count_low INTEGER NOT NULL CHECK(count_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE gltf_sparse_indices (
 id INTEGER PRIMARY KEY REFERENCES gltf_sparse_accessor(id),
 buffer_view_index_high INTEGER NOT NULL CHECK(buffer_view_index_high BETWEEN 0 AND 4294967295),
 buffer_view_index_low INTEGER NOT NULL CHECK(buffer_view_index_low BETWEEN 0 AND 4294967295),
 byte_offset_high INTEGER NOT NULL CHECK(byte_offset_high BETWEEN 0 AND 4294967295),
 byte_offset_low INTEGER NOT NULL CHECK(byte_offset_low BETWEEN 0 AND 4294967295),
 component_type INTEGER NOT NULL CHECK(component_type IN (5120,5121,5122,5123,5125,5126))
);
CREATE TABLE gltf_sparse_values (
 id INTEGER PRIMARY KEY REFERENCES gltf_sparse_accessor(id),
 buffer_view_index_high INTEGER NOT NULL CHECK(buffer_view_index_high BETWEEN 0 AND 4294967295),
 buffer_view_index_low INTEGER NOT NULL CHECK(buffer_view_index_low BETWEEN 0 AND 4294967295),
 byte_offset_high INTEGER NOT NULL CHECK(byte_offset_high BETWEEN 0 AND 4294967295),
 byte_offset_low INTEGER NOT NULL CHECK(byte_offset_low BETWEEN 0 AND 4294967295)
);
