CREATE TABLE step_document (
  id INTEGER PRIMARY KEY,
  schema TEXT NOT NULL,
  header_id INTEGER NOT NULL REFERENCES step_header(id)
);
CREATE TABLE step_header (
  id INTEGER PRIMARY KEY,
  implementation_level TEXT NOT NULL,
  name TEXT NOT NULL,
  timestamp TEXT NOT NULL,
  preprocessor_version TEXT NOT NULL,
  originating_system TEXT NOT NULL,
  authorization TEXT NOT NULL
);
CREATE TABLE step_description (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES step_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  content TEXT NOT NULL
);
CREATE TABLE step_author (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES step_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE step_organization (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES step_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE step_schema_identifier (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES step_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE step_entity (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES step_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  instance_id TEXT NOT NULL,
  name TEXT NOT NULL
);
CREATE TABLE step_complex_type (
  id INTEGER PRIMARY KEY,
  entity_id INTEGER NOT NULL REFERENCES step_entity(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE step_argument (
  id INTEGER PRIMARY KEY,
  entity_id INTEGER REFERENCES step_entity(id),
  complex_type_id INTEGER REFERENCES step_complex_type(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES step_value(id),
  CHECK ((entity_id IS NULL) != (complex_type_id IS NULL))
);
CREATE TABLE step_value (
  id INTEGER PRIMARY KEY,
  kind TEXT NOT NULL CHECK (kind IN ('unset','derived','integer','real','string','enum','reference','aggregate','typedValue')),
  integer_value INTEGER,
  real_value REAL,
  string_value TEXT,
  enum_value TEXT,
  reference_instance_id TEXT,
  reference_entity_id INTEGER REFERENCES step_entity(id),
  real_bits INTEGER,
  real_class TEXT CHECK (real_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  CHECK ((kind = 'integer') = (integer_value IS NOT NULL)),
  CHECK ((kind = 'string') = (string_value IS NOT NULL)),
  CHECK ((kind = 'enum') = (enum_value IS NOT NULL)),
  CHECK ((kind = 'reference') = (reference_instance_id IS NOT NULL)),
  CHECK ((kind = 'reference') = (reference_entity_id IS NOT NULL)),
  CHECK ((kind = 'real') = (real_bits IS NOT NULL)),
  CHECK ((kind = 'real') = (real_class IS NOT NULL)),
  CHECK ((real_value IS NOT NULL) = (kind = 'real' AND real_class != 'nan'))
);
CREATE TABLE step_aggregate_element (
  id INTEGER PRIMARY KEY,
  aggregate_value_id INTEGER NOT NULL REFERENCES step_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES step_value(id)
);
CREATE TABLE step_typed_value (
  id INTEGER PRIMARY KEY,
  wrapper_value_id INTEGER NOT NULL REFERENCES step_value(id),
  type_name TEXT NOT NULL,
  value_id INTEGER NOT NULL REFERENCES step_value(id)
);
