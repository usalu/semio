CREATE TABLE rewriting_pattern (
 id INTEGER PRIMARY KEY,
 role TEXT NOT NULL CHECK(role IN ('lhs','create','merge')),
 ordinal INTEGER NOT NULL,
 left_var TEXT NOT NULL,
 left_kind TEXT NOT NULL,
 edge_var TEXT,
 edge_kind TEXT,
 right_var TEXT,
 right_kind TEXT
);
CREATE TABLE rewriting_lhs (
 id INTEGER PRIMARY KEY,
 pattern_id INTEGER NOT NULL REFERENCES rewriting_pattern(id),
 where_clause TEXT
);
CREATE TABLE rewriting_assignment (
 id INTEGER PRIMARY KEY,
 rhs_id INTEGER NOT NULL,
 ordinal INTEGER NOT NULL,
 var TEXT NOT NULL,
 prop TEXT NOT NULL
);
CREATE TABLE rewriting_parameter (
 id INTEGER PRIMARY KEY,
 rhs_id INTEGER NOT NULL,
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL,
 kind TEXT NOT NULL CHECK(kind IN ('string','number','boolean'))
);
