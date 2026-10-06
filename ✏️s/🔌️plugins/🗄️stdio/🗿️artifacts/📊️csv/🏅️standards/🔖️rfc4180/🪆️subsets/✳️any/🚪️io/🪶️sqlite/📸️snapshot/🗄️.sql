CREATE TABLE csv_document (
  id INTEGER PRIMARY KEY CHECK(id = 1),
  schema TEXT NOT NULL,
  has_header INTEGER NOT NULL CHECK(has_header IN (0, 1))
);
CREATE TABLE csv_record (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES csv_document(id),
  ordinal INTEGER NOT NULL CHECK(ordinal >= 0)
);
CREATE TABLE csv_field (
  id INTEGER PRIMARY KEY,
  record_id INTEGER NOT NULL REFERENCES csv_record(id),
  ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
  value TEXT NOT NULL,
  quoted INTEGER NOT NULL CHECK(quoted IN (0, 1))
);
