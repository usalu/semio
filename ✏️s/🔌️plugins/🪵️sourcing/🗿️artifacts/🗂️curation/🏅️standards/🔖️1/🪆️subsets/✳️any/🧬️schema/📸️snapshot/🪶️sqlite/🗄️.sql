-- 🗂️ Complete literal Kit ownership and ordered sourcing recipe entities.
CREATE TABLE curation_document (
  id INTEGER PRIMARY KEY CHECK (id = 1)
);
CREATE TABLE curation_catalog (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  document_id INTEGER NOT NULL REFERENCES curation_document(id),
  child_id TEXT NOT NULL,
  artifact_id TEXT NOT NULL,
  artifact_kind TEXT NOT NULL CHECK (artifact_kind = 's.stdio.semio'),
  standard TEXT NOT NULL CHECK (standard = 'v1'),
  subset TEXT NOT NULL CHECK (subset = 'kit')
);
CREATE TABLE curation_stock_extra (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  document_id INTEGER NOT NULL REFERENCES curation_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  object_id TEXT NOT NULL,
  name TEXT NOT NULL,
  module_id TEXT NOT NULL,
  availability INTEGER NOT NULL CHECK (availability BETWEEN 0 AND 4294967295)
);
CREATE TABLE curation_typology_segment (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  stock_extra_id INTEGER NOT NULL REFERENCES curation_stock_extra(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  segment TEXT NOT NULL
);
CREATE TABLE curation_geometry (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  stock_extra_id INTEGER NOT NULL REFERENCES curation_stock_extra(id),
  kind TEXT NOT NULL CHECK (kind IN ('box','frame','slab','mesh','glb'))
);
CREATE TABLE curation_box (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  geometry_id INTEGER NOT NULL REFERENCES curation_geometry(id),
  width REAL,
  height REAL,
  depth REAL,
  width_bits INTEGER NOT NULL,
  width_class TEXT NOT NULL CHECK (width_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
  height_bits INTEGER NOT NULL,
  height_class TEXT NOT NULL CHECK (height_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
  depth_bits INTEGER NOT NULL,
  depth_class TEXT NOT NULL CHECK (depth_class IN ('finite','nan','positiveInfinity','negativeInfinity'))
);
CREATE TABLE curation_frame (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  geometry_id INTEGER NOT NULL REFERENCES curation_geometry(id),
  width REAL,
  height REAL,
  depth REAL,
  profile REAL,
  width_bits INTEGER NOT NULL,
  width_class TEXT NOT NULL CHECK (width_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
  height_bits INTEGER NOT NULL,
  height_class TEXT NOT NULL CHECK (height_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
  depth_bits INTEGER NOT NULL,
  depth_class TEXT NOT NULL CHECK (depth_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
  profile_bits INTEGER NOT NULL,
  profile_class TEXT NOT NULL CHECK (profile_class IN ('finite','nan','positiveInfinity','negativeInfinity'))
);
CREATE TABLE curation_slab (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  geometry_id INTEGER NOT NULL REFERENCES curation_geometry(id),
  width REAL,
  depth REAL,
  thickness REAL,
  width_bits INTEGER NOT NULL,
  width_class TEXT NOT NULL CHECK (width_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
  depth_bits INTEGER NOT NULL,
  depth_class TEXT NOT NULL CHECK (depth_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
  thickness_bits INTEGER NOT NULL,
  thickness_class TEXT NOT NULL CHECK (thickness_class IN ('finite','nan','positiveInfinity','negativeInfinity'))
);
CREATE TABLE curation_mesh (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  geometry_id INTEGER NOT NULL REFERENCES curation_geometry(id)
);
CREATE TABLE curation_glb (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  geometry_id INTEGER NOT NULL REFERENCES curation_geometry(id),
  url TEXT NOT NULL,
  extent REAL,
  extent_bits INTEGER NOT NULL,
  extent_class TEXT NOT NULL CHECK (extent_class IN ('finite','nan','positiveInfinity','negativeInfinity'))
);
CREATE TABLE curation_mesh_position (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  mesh_id INTEGER NOT NULL REFERENCES curation_mesh(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value REAL,
  value_bits INTEGER NOT NULL CHECK (value_bits BETWEEN 0 AND 4294967295),
  value_class TEXT NOT NULL CHECK (value_class IN ('finite','nan','positiveInfinity','negativeInfinity'))
);
CREATE TABLE curation_mesh_normal (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  mesh_id INTEGER NOT NULL REFERENCES curation_mesh(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value REAL,
  value_bits INTEGER NOT NULL CHECK (value_bits BETWEEN 0 AND 4294967295),
  value_class TEXT NOT NULL CHECK (value_class IN ('finite','nan','positiveInfinity','negativeInfinity'))
);
CREATE TABLE curation_mesh_index (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  mesh_id INTEGER NOT NULL REFERENCES curation_mesh(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 4294967295)
);
CREATE TABLE curation_curated (
  id INTEGER PRIMARY KEY CHECK (id > 0),
  document_id INTEGER NOT NULL REFERENCES curation_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  object_id TEXT NOT NULL,
  count INTEGER NOT NULL CHECK (count BETWEEN 0 AND 4294967295)
);
