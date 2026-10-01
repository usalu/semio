CREATE TABLE text_document (
  id INTEGER PRIMARY KEY CHECK(id = 1),
  schema TEXT NOT NULL,
  trailing_newline INTEGER NOT NULL CHECK(trailing_newline IN (0, 1)),
  line_ending TEXT NOT NULL CHECK(line_ending IN ('lf', 'crlf'))
);
CREATE TABLE text_line (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES text_document(id),
  ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
  content TEXT NOT NULL
);
