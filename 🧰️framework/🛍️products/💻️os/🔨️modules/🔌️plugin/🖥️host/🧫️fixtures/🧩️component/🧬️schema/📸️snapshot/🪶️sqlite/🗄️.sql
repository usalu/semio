CREATE TABLE fixture_counter (id INTEGER PRIMARY KEY CHECK (id = 1), count INTEGER NOT NULL CHECK (typeof(count) = 'integer' AND count BETWEEN -2147483648 AND 2147483647));
