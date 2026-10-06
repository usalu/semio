CREATE TABLE tsv_document (
  id INTEGER PRIMARY KEY CHECK(id = 1),
  schema TEXT NOT NULL,
  trailing_newline INTEGER NOT NULL CHECK(trailing_newline IN (0, 1)),
  line_ending TEXT NOT NULL CHECK(line_ending IN ('lf', 'crlf'))
);
CREATE TABLE tsv_record (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES tsv_document(id),
  ordinal INTEGER NOT NULL CHECK(ordinal >= 0)
);
CREATE TABLE tsv_field (
  id INTEGER PRIMARY KEY,
  record_id INTEGER NOT NULL REFERENCES tsv_record(id),
  ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
  value TEXT NOT NULL
);
