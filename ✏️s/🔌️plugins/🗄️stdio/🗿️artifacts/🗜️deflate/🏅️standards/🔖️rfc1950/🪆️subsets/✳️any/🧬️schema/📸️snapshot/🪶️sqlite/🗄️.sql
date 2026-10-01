CREATE TABLE deflate_document (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  schema TEXT NOT NULL,
  compression_method INTEGER NOT NULL CHECK (compression_method BETWEEN 0 AND 15),
  window_bits INTEGER NOT NULL CHECK (window_bits BETWEEN 0 AND 15),
  compression_level_hint INTEGER NOT NULL CHECK (compression_level_hint BETWEEN 0 AND 3),
  dictionary_adler32 INTEGER CHECK (dictionary_adler32 BETWEEN 0 AND 4294967295)
);
CREATE TABLE deflate_payload_byte (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES deflate_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)
);
