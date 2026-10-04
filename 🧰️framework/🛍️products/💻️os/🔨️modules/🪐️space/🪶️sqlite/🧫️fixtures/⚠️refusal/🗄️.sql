CREATE TABLE space_field_parent (id INTEGER PRIMARY KEY);
CREATE TABLE space_field_child (id INTEGER PRIMARY KEY, parent_id INTEGER NOT NULL REFERENCES space_field_parent(id), ordinal INTEGER NOT NULL, label TEXT NOT NULL);
