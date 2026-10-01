CREATE TABLE obj_document (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  schema TEXT NOT NULL,
  material_library TEXT
);
CREATE TABLE obj_vertex (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES obj_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  x REAL,
  y REAL,
  z REAL,
  w REAL,
  x_ieee754_bits INTEGER NOT NULL,
  x_numeric_class TEXT NOT NULL CHECK(x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  y_ieee754_bits INTEGER NOT NULL,
  y_numeric_class TEXT NOT NULL CHECK(y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  z_ieee754_bits INTEGER NOT NULL,
  z_numeric_class TEXT NOT NULL CHECK(z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  w_ieee754_bits INTEGER,
  w_numeric_class TEXT CHECK(w_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  CHECK((w_ieee754_bits IS NULL AND w_numeric_class IS NULL AND w IS NULL) OR (w_ieee754_bits IS NOT NULL AND w_numeric_class IS NOT NULL))
);
CREATE TABLE obj_texcoord (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES obj_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  u REAL,
  v REAL,
  w REAL,
  u_ieee754_bits INTEGER NOT NULL,
  u_numeric_class TEXT NOT NULL CHECK(u_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  v_ieee754_bits INTEGER NOT NULL,
  v_numeric_class TEXT NOT NULL CHECK(v_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  w_ieee754_bits INTEGER,
  w_numeric_class TEXT CHECK(w_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  CHECK((w_ieee754_bits IS NULL AND w_numeric_class IS NULL AND w IS NULL) OR (w_ieee754_bits IS NOT NULL AND w_numeric_class IS NOT NULL))
);
CREATE TABLE obj_normal (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES obj_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  x REAL,
  y REAL,
  z REAL,
  x_ieee754_bits INTEGER NOT NULL,
  x_numeric_class TEXT NOT NULL CHECK(x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  y_ieee754_bits INTEGER NOT NULL,
  y_numeric_class TEXT NOT NULL CHECK(y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  z_ieee754_bits INTEGER NOT NULL,
  z_numeric_class TEXT NOT NULL CHECK(z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE obj_face (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES obj_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0)
);
CREATE TABLE obj_face_vertex (
  id INTEGER PRIMARY KEY,
  face_id INTEGER NOT NULL REFERENCES obj_face(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  vertex_id INTEGER NOT NULL REFERENCES obj_vertex(id),
  texcoord_id INTEGER REFERENCES obj_texcoord(id),
  normal_id INTEGER REFERENCES obj_normal(id)
);
CREATE TABLE obj_group (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES obj_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE obj_group_face (
  id INTEGER PRIMARY KEY,
  group_id INTEGER NOT NULL REFERENCES obj_group(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  face_id INTEGER NOT NULL REFERENCES obj_face(id)
);
CREATE TABLE obj_object (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES obj_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE obj_object_face (
  id INTEGER PRIMARY KEY,
  object_id INTEGER NOT NULL REFERENCES obj_object(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  face_id INTEGER NOT NULL REFERENCES obj_face(id)
);
CREATE TABLE obj_face_boundary (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES obj_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  face_id INTEGER REFERENCES obj_face(id)
);
CREATE TABLE obj_material_range (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES obj_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  first_boundary_id INTEGER NOT NULL REFERENCES obj_face_boundary(id),
  material TEXT NOT NULL
);
CREATE TABLE obj_smoothing_range (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES obj_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  first_boundary_id INTEGER NOT NULL REFERENCES obj_face_boundary(id),
  group_number INTEGER CHECK (group_number BETWEEN 0 AND 4294967295)
);
CREATE TABLE obj_unknown_statement (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES obj_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  source_line_ordinal_high INTEGER NOT NULL CHECK (source_line_ordinal_high BETWEEN 0 AND 4294967295),
  source_line_ordinal_low INTEGER NOT NULL CHECK (source_line_ordinal_low BETWEEN 0 AND 4294967295),
  raw TEXT NOT NULL
);
