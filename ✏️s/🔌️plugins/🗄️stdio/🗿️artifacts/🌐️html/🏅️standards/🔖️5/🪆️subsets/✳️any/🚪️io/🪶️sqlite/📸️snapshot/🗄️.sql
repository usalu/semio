CREATE TABLE html_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL, doctype TEXT, root_node_id INTEGER NOT NULL REFERENCES html_node(id));
CREATE TABLE html_node (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK(kind IN ('element','text','comment','raw_text')));
CREATE TABLE html_element (node_id INTEGER PRIMARY KEY REFERENCES html_node(id), name TEXT NOT NULL);
CREATE TABLE html_text (node_id INTEGER PRIMARY KEY REFERENCES html_node(id), text TEXT NOT NULL);
CREATE TABLE html_comment (node_id INTEGER PRIMARY KEY REFERENCES html_node(id), text TEXT NOT NULL);
CREATE TABLE html_raw_text (node_id INTEGER PRIMARY KEY REFERENCES html_node(id), parent_kind TEXT NOT NULL CHECK(parent_kind IN ('script','style')), text TEXT NOT NULL);
CREATE TABLE html_attribute (id INTEGER PRIMARY KEY, element_node_id INTEGER NOT NULL REFERENCES html_element(node_id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), name TEXT NOT NULL, value TEXT);
CREATE TABLE html_child (id INTEGER PRIMARY KEY, parent_element_node_id INTEGER NOT NULL REFERENCES html_element(node_id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), child_node_id INTEGER NOT NULL REFERENCES html_node(id));
