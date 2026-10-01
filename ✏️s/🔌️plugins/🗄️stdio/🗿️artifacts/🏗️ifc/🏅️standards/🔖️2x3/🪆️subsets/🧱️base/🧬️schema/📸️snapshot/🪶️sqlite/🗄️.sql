CREATE TABLE ifc2x3_document (
  id INTEGER PRIMARY KEY,
  schema TEXT NOT NULL,
  header_id INTEGER NOT NULL REFERENCES ifc2x3_header(id),
  edm_preamble_id INTEGER REFERENCES ifc2x3_edm_preamble(id)
);
CREATE TABLE ifc2x3_header (id INTEGER PRIMARY KEY);
CREATE TABLE ifc2x3_file_description_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc2x3_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_file_name_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc2x3_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_file_schema_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc2x3_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_instance (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES ifc2x3_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  instance_id TEXT NOT NULL
);
CREATE TABLE ifc2x3_entity_type (
  id INTEGER PRIMARY KEY,
  instance_id INTEGER NOT NULL REFERENCES ifc2x3_instance(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE ifc2x3_entity_argument (
  id INTEGER PRIMARY KEY,
  entity_type_id INTEGER NOT NULL REFERENCES ifc2x3_entity_type(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_value (
  id INTEGER PRIMARY KEY,
  kind TEXT NOT NULL CHECK (kind IN ('unset','derived','int','real','str','enum','ref','list','typed')),
  integer_value INTEGER,
  decimal_negative INTEGER CHECK (decimal_negative IN (0,1)),
  decimal_coefficient TEXT,
  decimal_scale INTEGER CHECK (decimal_scale >= 0 AND decimal_scale <= 4294967295),
  decimal_exponent INTEGER CHECK (decimal_exponent >= -2147483648 AND decimal_exponent <= 2147483647),
  string_value TEXT,
  enum_value TEXT,
  reference_instance_id TEXT,
  reference_instance_row_id INTEGER REFERENCES ifc2x3_instance(id),
  typed_name TEXT,
  CHECK ((kind = 'int') = (integer_value IS NOT NULL)),
  CHECK ((kind = 'real') = (decimal_negative IS NOT NULL)),
  CHECK ((kind = 'real') = (decimal_coefficient IS NOT NULL)),
  CHECK ((kind = 'real') = (decimal_scale IS NOT NULL)),
  CHECK (kind = 'real' OR decimal_exponent IS NULL),
  CHECK ((kind = 'str') = (string_value IS NOT NULL)),
  CHECK ((kind = 'enum') = (enum_value IS NOT NULL)),
  CHECK ((kind = 'ref') = (reference_instance_id IS NOT NULL)),
  CHECK ((kind = 'ref') = (reference_instance_row_id IS NOT NULL)),
  CHECK ((kind = 'typed') = (typed_name IS NOT NULL))
);
CREATE TABLE ifc2x3_list_element (
  id INTEGER PRIMARY KEY,
  list_value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_typed_argument (
  id INTEGER PRIMARY KEY,
  typed_value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_edm_preamble (
  id INTEGER PRIMARY KEY,
  producer TEXT NOT NULL,
  module TEXT NOT NULL,
  creation_date TEXT NOT NULL,
  host TEXT NOT NULL,
  database TEXT NOT NULL,
  database_version TEXT NOT NULL,
  database_creation_date TEXT NOT NULL,
  schema TEXT NOT NULL,
  model TEXT NOT NULL,
  model_creation_date TEXT NOT NULL,
  header_model TEXT NOT NULL,
  header_model_creation_date TEXT NOT NULL,
  user TEXT NOT NULL,
  group_name TEXT NOT NULL,
  license TEXT NOT NULL,
  options TEXT NOT NULL
);
