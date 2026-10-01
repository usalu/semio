CREATE TABLE ifc_document (
  id INTEGER PRIMARY KEY,
  schema TEXT NOT NULL,
  header_id INTEGER NOT NULL REFERENCES ifc_header(id)
);
CREATE TABLE ifc_header (id INTEGER PRIMARY KEY);
CREATE TABLE ifc_file_description_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id)
);
CREATE TABLE ifc_file_name_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id)
);
CREATE TABLE ifc_file_schema_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id)
);
CREATE TABLE ifc_entity (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES ifc_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  instance_id TEXT NOT NULL,
  name TEXT NOT NULL
);
CREATE TABLE ifc_complex_type (
  id INTEGER PRIMARY KEY,
  entity_id INTEGER NOT NULL REFERENCES ifc_entity(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE ifc_argument (
  id INTEGER PRIMARY KEY,
  entity_id INTEGER REFERENCES ifc_entity(id),
  complex_type_id INTEGER REFERENCES ifc_complex_type(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id),
  CHECK ((entity_id IS NULL) != (complex_type_id IS NULL))
);
CREATE TABLE ifc_value (
  id INTEGER PRIMARY KEY,
  kind TEXT NOT NULL CHECK (kind IN ('unset','derived','integer','real','string','enum','reference','aggregate','typedValue')),
  integer_value INTEGER,
  real_value REAL,
  string_value TEXT,
  enum_value TEXT,
  reference_instance_id TEXT,
  reference_entity_id INTEGER REFERENCES ifc_entity(id),
  typed_name TEXT,
  real_bits INTEGER,
  real_class TEXT CHECK (real_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  CHECK ((kind = 'integer') = (integer_value IS NOT NULL)),
  CHECK ((kind = 'string') = (string_value IS NOT NULL)),
  CHECK ((kind = 'enum') = (enum_value IS NOT NULL)),
  CHECK ((kind = 'reference') = (reference_instance_id IS NOT NULL)),
  CHECK ((kind = 'reference') = (reference_entity_id IS NOT NULL)),
  CHECK ((kind = 'typedValue') = (typed_name IS NOT NULL)),
  CHECK ((kind = 'real') = (real_bits IS NOT NULL)),
  CHECK ((kind = 'real') = (real_class IS NOT NULL)),
  CHECK ((real_value IS NOT NULL) = (kind = 'real' AND real_class != 'nan'))
);
CREATE TABLE ifc_aggregate_element (
  id INTEGER PRIMARY KEY,
  aggregate_value_id INTEGER NOT NULL REFERENCES ifc_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id)
);
CREATE TABLE ifc_typed_argument (
  id INTEGER PRIMARY KEY,
  typed_value_id INTEGER NOT NULL REFERENCES ifc_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id)
);
