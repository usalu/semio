CREATE TABLE binary_document (
  id INTEGER PRIMARY KEY CHECK(id = 1),
  schema TEXT NOT NULL
);
CREATE TABLE binary_byte (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES binary_document(id),
  ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
  value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)
);
