CREATE TABLE png_document (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  schema TEXT NOT NULL,
  width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295),
  height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295),
  bit_depth INTEGER NOT NULL CHECK (bit_depth BETWEEN 0 AND 255),
  color_type INTEGER NOT NULL CHECK (color_type IN (0, 2, 3, 4, 6)),
  interlace INTEGER NOT NULL CHECK (interlace IN (0, 1))
);
CREATE TABLE png_palette (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES png_document(id)
);
CREATE TABLE png_palette_entry (
  id INTEGER PRIMARY KEY,
  palette_id INTEGER NOT NULL REFERENCES png_palette(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255),
  green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255),
  blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255)
);
CREATE TABLE png_transparency (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  color_model TEXT NOT NULL CHECK (color_model IN ('indexed', 'grayscale', 'rgb')),
  gray INTEGER CHECK (gray BETWEEN 0 AND 65535),
  red INTEGER CHECK (red BETWEEN 0 AND 65535),
  green INTEGER CHECK (green BETWEEN 0 AND 65535),
  blue INTEGER CHECK (blue BETWEEN 0 AND 65535),
  CHECK ((color_model = 'indexed' AND gray IS NULL AND red IS NULL AND green IS NULL AND blue IS NULL) OR (color_model = 'grayscale' AND gray IS NOT NULL AND red IS NULL AND green IS NULL AND blue IS NULL) OR (color_model = 'rgb' AND gray IS NULL AND red IS NOT NULL AND green IS NOT NULL AND blue IS NOT NULL))
);
CREATE TABLE png_transparency_alpha (
  id INTEGER PRIMARY KEY,
  transparency_id INTEGER NOT NULL REFERENCES png_transparency(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  alpha INTEGER NOT NULL CHECK (alpha BETWEEN 0 AND 255)
);
CREATE TABLE png_gamma (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  gamma_times_100000 INTEGER NOT NULL CHECK (gamma_times_100000 BETWEEN 0 AND 4294967295)
);
CREATE TABLE png_chromaticity (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  white_x INTEGER NOT NULL CHECK (white_x BETWEEN 0 AND 4294967295),
  white_y INTEGER NOT NULL CHECK (white_y BETWEEN 0 AND 4294967295),
  red_x INTEGER NOT NULL CHECK (red_x BETWEEN 0 AND 4294967295),
  red_y INTEGER NOT NULL CHECK (red_y BETWEEN 0 AND 4294967295),
  green_x INTEGER NOT NULL CHECK (green_x BETWEEN 0 AND 4294967295),
  green_y INTEGER NOT NULL CHECK (green_y BETWEEN 0 AND 4294967295),
  blue_x INTEGER NOT NULL CHECK (blue_x BETWEEN 0 AND 4294967295),
  blue_y INTEGER NOT NULL CHECK (blue_y BETWEEN 0 AND 4294967295)
);
CREATE TABLE png_srgb (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  rendering_intent INTEGER NOT NULL CHECK (rendering_intent BETWEEN 0 AND 3)
);
CREATE TABLE png_physical_dimensions (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  pixels_per_unit_x INTEGER NOT NULL CHECK (pixels_per_unit_x BETWEEN 0 AND 4294967295),
  pixels_per_unit_y INTEGER NOT NULL CHECK (pixels_per_unit_y BETWEEN 0 AND 4294967295),
  unit_is_meter INTEGER NOT NULL CHECK (unit_is_meter IN (0, 1))
);
CREATE TABLE png_modification_time (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  year INTEGER NOT NULL CHECK (year BETWEEN 0 AND 65535),
  month INTEGER NOT NULL CHECK (month BETWEEN 0 AND 255),
  day INTEGER NOT NULL CHECK (day BETWEEN 0 AND 255),
  hour INTEGER NOT NULL CHECK (hour BETWEEN 0 AND 255),
  minute INTEGER NOT NULL CHECK (minute BETWEEN 0 AND 255),
  second INTEGER NOT NULL CHECK (second BETWEEN 0 AND 255)
);
CREATE TABLE png_background (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  color_model TEXT NOT NULL CHECK (color_model IN ('indexed', 'grayscale', 'rgb')),
  gray INTEGER CHECK (gray BETWEEN 0 AND 65535),
  red INTEGER CHECK (red BETWEEN 0 AND 65535),
  green INTEGER CHECK (green BETWEEN 0 AND 65535),
  blue INTEGER CHECK (blue BETWEEN 0 AND 65535),
  palette_index INTEGER CHECK (palette_index BETWEEN 0 AND 255),
  CHECK ((color_model = 'indexed' AND gray IS NULL AND red IS NULL AND green IS NULL AND blue IS NULL AND palette_index IS NOT NULL) OR (color_model = 'grayscale' AND gray IS NOT NULL AND red IS NULL AND green IS NULL AND blue IS NULL AND palette_index IS NULL) OR (color_model = 'rgb' AND gray IS NULL AND red IS NOT NULL AND green IS NOT NULL AND blue IS NOT NULL AND palette_index IS NULL))
);
CREATE TABLE png_text (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  keyword TEXT NOT NULL,
  content TEXT NOT NULL,
  compressed INTEGER NOT NULL CHECK (compressed IN (0, 1)),
  chunk_kind TEXT NOT NULL CHECK (chunk_kind IN ('text', 'ztext', 'itext')),
  language_tag TEXT NOT NULL,
  translated_keyword TEXT NOT NULL
);
CREATE TABLE png_pixel (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  x INTEGER NOT NULL CHECK (x BETWEEN 0 AND 4294967295),
  y INTEGER NOT NULL CHECK (y BETWEEN 0 AND 4294967295),
  red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255),
  green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255),
  blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255),
  alpha INTEGER NOT NULL CHECK (alpha BETWEEN 0 AND 255)
);
CREATE TABLE png_unknown_chunk (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  kind_octet_1 INTEGER NOT NULL CHECK (kind_octet_1 BETWEEN 0 AND 255),
  kind_octet_2 INTEGER NOT NULL CHECK (kind_octet_2 BETWEEN 0 AND 255),
  kind_octet_3 INTEGER NOT NULL CHECK (kind_octet_3 BETWEEN 0 AND 255),
  kind_octet_4 INTEGER NOT NULL CHECK (kind_octet_4 BETWEEN 0 AND 255)
);
CREATE TABLE png_unknown_chunk_byte (
  id INTEGER PRIMARY KEY,
  unknown_chunk_id INTEGER NOT NULL REFERENCES png_unknown_chunk(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)
);
CREATE TABLE png_chunk_sequence (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES png_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  chunk_kind TEXT NOT NULL CHECK (chunk_kind IN ('ihdr', 'plte', 'trns', 'gama', 'chrm', 'srgb', 'phys', 'time', 'bkgd', 'idat', 'iend', 'text', 'unknown')),
  text_id INTEGER REFERENCES png_text(id),
  unknown_chunk_id INTEGER REFERENCES png_unknown_chunk(id),
  CHECK ((chunk_kind = 'text' AND text_id IS NOT NULL AND unknown_chunk_id IS NULL) OR (chunk_kind = 'unknown' AND text_id IS NULL AND unknown_chunk_id IS NOT NULL) OR (chunk_kind NOT IN ('text', 'unknown') AND text_id IS NULL AND unknown_chunk_id IS NULL))
);
