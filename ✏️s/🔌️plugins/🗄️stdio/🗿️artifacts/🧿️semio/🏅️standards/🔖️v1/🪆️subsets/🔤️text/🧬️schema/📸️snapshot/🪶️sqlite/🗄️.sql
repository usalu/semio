CREATE TABLE semio_text_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL);
CREATE TABLE semio_text_run (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_text_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), language TEXT NOT NULL, content TEXT NOT NULL);
CREATE TABLE semio_text_mark (id INTEGER PRIMARY KEY, run_id INTEGER NOT NULL REFERENCES semio_text_run(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), kind TEXT NOT NULL CHECK(kind IN ('bold','italic','code','link')), href TEXT NOT NULL);
