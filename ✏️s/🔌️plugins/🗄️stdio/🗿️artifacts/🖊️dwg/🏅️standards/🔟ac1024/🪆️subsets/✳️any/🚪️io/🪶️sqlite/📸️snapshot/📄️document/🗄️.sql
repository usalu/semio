CREATE TABLE dwg_document (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  schema TEXT NOT NULL,
  version TEXT NOT NULL,
  maintenance_version INTEGER NOT NULL CHECK (maintenance_version BETWEEN 0 AND 255),
  codepage INTEGER NOT NULL CHECK (codepage BETWEEN 0 AND 65535)
);
CREATE TABLE dwg_summary (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES dwg_document(id),
  title TEXT NOT NULL,
  subject TEXT NOT NULL,
  author TEXT NOT NULL,
  keywords TEXT NOT NULL,
  comments TEXT NOT NULL,
  last_saved_by TEXT NOT NULL,
  revision_number TEXT NOT NULL,
  hyperlink_base TEXT NOT NULL,
  total_editing_time_high INTEGER NOT NULL CHECK (total_editing_time_high BETWEEN 0 AND 4294967295),
  total_editing_time_low INTEGER NOT NULL CHECK (total_editing_time_low BETWEEN 0 AND 4294967295),
  created_days INTEGER NOT NULL CHECK (created_days BETWEEN 0 AND 4294967295),
  created_milliseconds INTEGER NOT NULL CHECK (created_milliseconds BETWEEN 0 AND 4294967295),
  modified_days INTEGER NOT NULL CHECK (modified_days BETWEEN 0 AND 4294967295),
  modified_milliseconds INTEGER NOT NULL CHECK (modified_milliseconds BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_custom_property (
  id INTEGER PRIMARY KEY,
  summary_id INTEGER NOT NULL REFERENCES dwg_summary(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  property_key TEXT NOT NULL,
  property_value TEXT NOT NULL
);
CREATE TABLE dwg_application_info (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES dwg_document(id),
  name TEXT NOT NULL,
  version_checksum TEXT NOT NULL,
  version TEXT NOT NULL,
  comment_checksum TEXT NOT NULL,
  comment TEXT NOT NULL,
  product_checksum TEXT NOT NULL,
  product TEXT NOT NULL,
  application_version TEXT NOT NULL
);
CREATE TABLE dwg_template (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES dwg_document(id),
  description TEXT NOT NULL,
  measurement TEXT NOT NULL CHECK (measurement IN ('english', 'metric'))
);
CREATE TABLE dwg_auxiliary_header (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES dwg_document(id),
  total_saves INTEGER NOT NULL CHECK (total_saves BETWEEN 0 AND 4294967295),
  save_partition_one INTEGER NOT NULL CHECK (save_partition_one BETWEEN 0 AND 65535),
  save_partition_two INTEGER NOT NULL CHECK (save_partition_two BETWEEN 0 AND 65535),
  save_generation INTEGER NOT NULL CHECK (save_generation BETWEEN 0 AND 4294967295),
  legacy_stamp_one_version INTEGER NOT NULL CHECK (legacy_stamp_one_version BETWEEN 0 AND 65535),
  legacy_stamp_one_maintenance INTEGER NOT NULL CHECK (legacy_stamp_one_maintenance BETWEEN 0 AND 65535),
  legacy_stamp_two_version INTEGER NOT NULL CHECK (legacy_stamp_two_version BETWEEN 0 AND 65535),
  legacy_stamp_two_maintenance INTEGER NOT NULL CHECK (legacy_stamp_two_maintenance BETWEEN 0 AND 65535),
  compatibility_profile TEXT NOT NULL CHECK (compatibility_profile = 'autocad2009'),
  created_days INTEGER NOT NULL CHECK (created_days BETWEEN 0 AND 4294967295),
  created_milliseconds INTEGER NOT NULL CHECK (created_milliseconds BETWEEN 0 AND 4294967295),
  updated_days INTEGER NOT NULL CHECK (updated_days BETWEEN 0 AND 4294967295),
  updated_milliseconds INTEGER NOT NULL CHECK (updated_milliseconds BETWEEN 0 AND 4294967295),
  handle_seed_high INTEGER NOT NULL CHECK (handle_seed_high BETWEEN 0 AND 4294967295),
  handle_seed_low INTEGER NOT NULL CHECK (handle_seed_low BETWEEN 0 AND 4294967295),
  terminal_save_generation INTEGER NOT NULL CHECK (terminal_save_generation BETWEEN 0 AND 65535)
);
CREATE TABLE dwg_revision_history (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES dwg_document(id),
  format_major INTEGER NOT NULL CHECK (format_major BETWEEN 0 AND 4294967295),
  format_minor INTEGER NOT NULL CHECK (format_minor BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_revision (
  id INTEGER PRIMARY KEY,
  history_id INTEGER NOT NULL REFERENCES dwg_revision_history(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  revision_value INTEGER NOT NULL CHECK (revision_value BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_preview (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES dwg_document(id),
  width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295),
  height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295),
  origin TEXT NOT NULL CHECK (origin = 'bottom_up'),
  background_palette_index INTEGER NOT NULL CHECK (background_palette_index BETWEEN 0 AND 255)
);
CREATE TABLE dwg_preview_palette_entry (
  id INTEGER PRIMARY KEY,
  preview_id INTEGER NOT NULL REFERENCES dwg_preview(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255),
  green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255),
  blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255),
  alpha INTEGER NOT NULL CHECK (alpha BETWEEN 0 AND 255)
);
CREATE TABLE dwg_preview_pixel_index (
  id INTEGER PRIMARY KEY,
  preview_id INTEGER NOT NULL REFERENCES dwg_preview(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  palette_index INTEGER NOT NULL CHECK (palette_index BETWEEN 0 AND 255)
);
CREATE TABLE dwg_application_history (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES dwg_document(id),
  history_identifier_one TEXT NOT NULL,
  history_identifier_two TEXT NOT NULL,
  class_version INTEGER NOT NULL CHECK (class_version BETWEEN 0 AND 4294967295),
  application_version_digest TEXT NOT NULL,
  application_version TEXT NOT NULL,
  trust_comment_digest TEXT NOT NULL,
  trust_comment TEXT NOT NULL,
  property_set_digest TEXT NOT NULL,
  property_format_identifier TEXT NOT NULL,
  product_digest TEXT NOT NULL
);
CREATE TABLE dwg_application_property (
  id INTEGER PRIMARY KEY,
  history_id INTEGER NOT NULL REFERENCES dwg_application_history(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  property_identifier INTEGER NOT NULL CHECK (property_identifier BETWEEN 0 AND 4294967295),
  kind TEXT NOT NULL CHECK (kind IN ('string', 'date_time')),
  value TEXT NOT NULL
);
CREATE TABLE dwg_product_information (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  history_id INTEGER NOT NULL REFERENCES dwg_application_history(id),
  name TEXT NOT NULL,
  build_version TEXT NOT NULL,
  registry_version TEXT NOT NULL,
  install_id TEXT NOT NULL,
  locale_id TEXT NOT NULL
);
CREATE TABLE dwg_class (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES dwg_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  class_number INTEGER NOT NULL CHECK (class_number BETWEEN 0 AND 65535),
  proxy_flags INTEGER NOT NULL CHECK (proxy_flags BETWEEN 0 AND 4294967295),
  application_name TEXT NOT NULL,
  cpp_class_name TEXT NOT NULL,
  dxf_name TEXT NOT NULL,
  was_zombie INTEGER NOT NULL CHECK (was_zombie IN (0, 1)),
  item_class_id INTEGER NOT NULL CHECK (item_class_id BETWEEN 0 AND 65535),
  object_count INTEGER NOT NULL CHECK (object_count BETWEEN 0 AND 4294967295),
  dwg_version INTEGER NOT NULL CHECK (dwg_version BETWEEN 0 AND 4294967295),
  maintenance_version INTEGER NOT NULL CHECK (maintenance_version BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_class_reserved_value (
  id INTEGER PRIMARY KEY,
  class_id INTEGER NOT NULL REFERENCES dwg_class(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_dependency (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES dwg_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  feature TEXT NOT NULL,
  full_path TEXT NOT NULL,
  relative_path TEXT NOT NULL,
  fingerprint TEXT NOT NULL,
  version TEXT NOT NULL,
  timestamp INTEGER NOT NULL CHECK (timestamp BETWEEN 0 AND 4294967295),
  file_size INTEGER NOT NULL CHECK (file_size BETWEEN 0 AND 4294967295),
  affects_graphics INTEGER NOT NULL CHECK (affects_graphics IN (0, 1)),
  reference_count INTEGER NOT NULL CHECK (reference_count BETWEEN 0 AND 4294967295)
);

