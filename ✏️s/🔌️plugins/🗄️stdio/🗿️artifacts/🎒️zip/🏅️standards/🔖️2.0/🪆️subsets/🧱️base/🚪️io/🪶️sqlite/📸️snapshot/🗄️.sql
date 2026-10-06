CREATE TABLE zip_archive (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  schema TEXT NOT NULL,
  comment TEXT NOT NULL,
  comment_utf8 INTEGER NOT NULL CHECK (comment_utf8 IN (0, 1))
);
CREATE TABLE zip_entry (
  id INTEGER PRIMARY KEY,
  archive_id INTEGER NOT NULL REFERENCES zip_archive(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL,
  compression_method INTEGER NOT NULL CHECK (compression_method BETWEEN 0 AND 65535),
  data_descriptor_signature INTEGER NOT NULL CHECK (data_descriptor_signature IN (0, 1))
);
CREATE TABLE zip_entry_byte (
  id INTEGER PRIMARY KEY,
  entry_id INTEGER NOT NULL REFERENCES zip_entry(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)
);
CREATE TABLE zip_local_header (
  id INTEGER PRIMARY KEY REFERENCES zip_entry(id),
  version_needed INTEGER NOT NULL CHECK (version_needed BETWEEN 0 AND 65535),
  flags INTEGER NOT NULL CHECK (flags BETWEEN 0 AND 65535),
  modified_time INTEGER NOT NULL CHECK (modified_time BETWEEN 0 AND 65535),
  modified_date INTEGER NOT NULL CHECK (modified_date BETWEEN 0 AND 65535),
  legacy_name_present INTEGER NOT NULL CHECK (legacy_name_present IN (0, 1))
);
CREATE TABLE zip_local_legacy_name_byte (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES zip_local_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)
);
CREATE TABLE zip_central_header (
  id INTEGER PRIMARY KEY REFERENCES zip_entry(id),
  version_made_by INTEGER NOT NULL CHECK (version_made_by BETWEEN 0 AND 65535),
  version_needed INTEGER NOT NULL CHECK (version_needed BETWEEN 0 AND 65535),
  flags INTEGER NOT NULL CHECK (flags BETWEEN 0 AND 65535),
  modified_time INTEGER NOT NULL CHECK (modified_time BETWEEN 0 AND 65535),
  modified_date INTEGER NOT NULL CHECK (modified_date BETWEEN 0 AND 65535),
  legacy_name_present INTEGER NOT NULL CHECK (legacy_name_present IN (0, 1)),
  comment TEXT NOT NULL,
  legacy_comment_present INTEGER NOT NULL CHECK (legacy_comment_present IN (0, 1)),
  internal_attributes INTEGER NOT NULL CHECK (internal_attributes BETWEEN 0 AND 65535),
  external_attributes INTEGER NOT NULL CHECK (external_attributes BETWEEN 0 AND 4294967295)
);
CREATE TABLE zip_central_legacy_name_byte (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES zip_central_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)
);
CREATE TABLE zip_central_legacy_comment_byte (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES zip_central_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)
);
CREATE TABLE zip_local_extra_field (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES zip_local_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  tag INTEGER NOT NULL CHECK (tag BETWEEN 0 AND 65535)
);
CREATE TABLE zip_local_extra_field_byte (
  id INTEGER PRIMARY KEY,
  extra_field_id INTEGER NOT NULL REFERENCES zip_local_extra_field(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)
);
CREATE TABLE zip_central_extra_field (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES zip_central_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  tag INTEGER NOT NULL CHECK (tag BETWEEN 0 AND 65535)
);
CREATE TABLE zip_central_extra_field_byte (
  id INTEGER PRIMARY KEY,
  extra_field_id INTEGER NOT NULL REFERENCES zip_central_extra_field(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)
);
