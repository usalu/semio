CREATE TABLE probe_node (
  id INTEGER PRIMARY KEY,
  parent_id INTEGER REFERENCES probe_node(id),
  position INTEGER CHECK (position >= 0),
  member_name TEXT,
  node_type TEXT NOT NULL CHECK (node_type IN ('null', 'boolean', 'number', 'string', 'array', 'object')),
  boolean_value INTEGER CHECK (boolean_value IN (0, 1)),
  number_value TEXT,
  string_value TEXT,
  CHECK ((parent_id IS NULL AND position IS NULL AND member_name IS NULL) OR (parent_id IS NOT NULL AND position IS NOT NULL)),
  CHECK (
    (node_type IN ('null', 'array', 'object') AND boolean_value IS NULL AND number_value IS NULL AND string_value IS NULL) OR
    (node_type = 'boolean' AND boolean_value IS NOT NULL AND number_value IS NULL AND string_value IS NULL) OR
    (node_type = 'number' AND boolean_value IS NULL AND number_value IS NOT NULL AND string_value IS NULL) OR
    (node_type = 'string' AND boolean_value IS NULL AND number_value IS NULL AND string_value IS NOT NULL)
  )
);
