CREATE TABLE gltf_json_value (
 id INTEGER PRIMARY KEY,
 kind TEXT NOT NULL CHECK(kind IN ('null','boolean','number','string','array','object')),
 boolean_value INTEGER CHECK(boolean_value IN (0,1)),
 number_value REAL,
 string_value TEXT,
 number_value_ieee754_bits INTEGER,
 number_value_numeric_class TEXT CHECK(number_value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 CHECK((kind='boolean' AND boolean_value IS NOT NULL) OR (kind!='boolean' AND boolean_value IS NULL)),
 CHECK((kind='string' AND string_value IS NOT NULL) OR (kind!='string' AND string_value IS NULL)),
 CHECK((kind='number' AND number_value_ieee754_bits IS NOT NULL AND number_value_numeric_class IS NOT NULL) OR (kind!='number' AND number_value IS NULL AND number_value_ieee754_bits IS NULL AND number_value_numeric_class IS NULL))
);
CREATE TABLE gltf_json_array_element (
 id INTEGER PRIMARY KEY,
 array_id INTEGER NOT NULL REFERENCES gltf_json_value(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 value_id INTEGER NOT NULL REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_json_object_member (
 id INTEGER PRIMARY KEY,
 object_id INTEGER NOT NULL REFERENCES gltf_json_value(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 name TEXT NOT NULL,
 value_id INTEGER NOT NULL REFERENCES gltf_json_value(id)
);
