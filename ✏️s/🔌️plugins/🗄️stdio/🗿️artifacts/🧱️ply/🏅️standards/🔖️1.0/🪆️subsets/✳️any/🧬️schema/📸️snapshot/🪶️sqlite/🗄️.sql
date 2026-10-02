CREATE TABLE ply_document (
  id INTEGER PRIMARY KEY,
  schema TEXT NOT NULL,
  format TEXT NOT NULL CHECK (format IN ('ascii', 'binary_little_endian', 'binary_big_endian'))
);
CREATE TABLE ply_comment (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES ply_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  content TEXT NOT NULL
);
CREATE TABLE ply_element (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES ply_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL,
  declared_count_high INTEGER NOT NULL CHECK (declared_count_high BETWEEN 0 AND 4294967295),
  declared_count_low INTEGER NOT NULL CHECK (declared_count_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE ply_property (
  id INTEGER PRIMARY KEY,
  element_id INTEGER NOT NULL REFERENCES ply_element(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL,
  form TEXT NOT NULL CHECK (form IN ('scalar', 'list')),
  scalar_kind TEXT CHECK (scalar_kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double')),
  count_kind TEXT CHECK (count_kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double')),
  value_kind TEXT CHECK (value_kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double')),
  CHECK ((form = 'scalar' AND scalar_kind IS NOT NULL AND count_kind IS NULL AND value_kind IS NULL) OR (form = 'list' AND scalar_kind IS NULL AND count_kind IS NOT NULL AND value_kind IS NOT NULL))
);
CREATE TABLE ply_row (
  id INTEGER PRIMARY KEY,
  element_id INTEGER NOT NULL REFERENCES ply_element(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0)
);
CREATE TABLE ply_cell (
  id INTEGER PRIMARY KEY,
  row_id INTEGER NOT NULL REFERENCES ply_row(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ply_value(id)
);
CREATE TABLE ply_value (
  id INTEGER PRIMARY KEY,
  kind TEXT NOT NULL CHECK (kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double', 'list')),
  integer_value INTEGER,
  real_value REAL,
  real_value_ieee754_bits INTEGER,
  real_value_numeric_class TEXT CHECK(real_value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  CHECK ((kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint') AND integer_value IS NOT NULL AND real_value IS NULL AND real_value_ieee754_bits IS NULL AND real_value_numeric_class IS NULL) OR (kind IN ('float', 'double') AND integer_value IS NULL AND real_value_ieee754_bits IS NOT NULL AND real_value_numeric_class IS NOT NULL AND ((real_value_numeric_class='nan' AND real_value IS NULL) OR (real_value_numeric_class!='nan' AND real_value IS NOT NULL))) OR (kind = 'list' AND integer_value IS NULL AND real_value IS NULL AND real_value_ieee754_bits IS NULL AND real_value_numeric_class IS NULL)),
  CHECK(kind!='float' OR real_value_ieee754_bits BETWEEN 0 AND 4294967295)
);
CREATE TABLE ply_list_item (
  id INTEGER PRIMARY KEY,
  list_id INTEGER NOT NULL REFERENCES ply_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ply_value(id)
);
