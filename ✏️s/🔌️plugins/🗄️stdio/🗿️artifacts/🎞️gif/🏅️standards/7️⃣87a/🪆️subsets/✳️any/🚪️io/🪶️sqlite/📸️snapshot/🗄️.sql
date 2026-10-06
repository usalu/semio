CREATE TABLE gif87_document (
 id INTEGER PRIMARY KEY CHECK (id = 1), schema TEXT NOT NULL,
 screen_width INTEGER NOT NULL CHECK (screen_width BETWEEN 0 AND 4294967295),
 screen_height INTEGER NOT NULL CHECK (screen_height BETWEEN 0 AND 4294967295),
 background_color_index INTEGER NOT NULL CHECK (background_color_index BETWEEN 0 AND 255),
 pixel_aspect_ratio INTEGER NOT NULL CHECK (pixel_aspect_ratio BETWEEN 0 AND 255)
);
CREATE TABLE gif87_global_palette (
 id INTEGER PRIMARY KEY REFERENCES gif87_document(id) CHECK (id = 1), sorted INTEGER NOT NULL CHECK (sorted IN (0,1))
);
CREATE TABLE gif87_global_color (
 id INTEGER PRIMARY KEY, palette_id INTEGER NOT NULL REFERENCES gif87_global_palette(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255), green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255), blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255)
);
CREATE TABLE gif87_image (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES gif87_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 left_position INTEGER NOT NULL CHECK (left_position BETWEEN 0 AND 4294967295), top_position INTEGER NOT NULL CHECK (top_position BETWEEN 0 AND 4294967295),
 width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295),
 interlace INTEGER NOT NULL CHECK (interlace IN (0,1))
);
CREATE TABLE gif87_local_palette (
 id INTEGER PRIMARY KEY REFERENCES gif87_image(id), sorted INTEGER NOT NULL CHECK (sorted IN (0,1))
);
CREATE TABLE gif87_local_color (
 id INTEGER PRIMARY KEY, palette_id INTEGER NOT NULL REFERENCES gif87_local_palette(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255), green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255), blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255)
);
CREATE TABLE gif87_pixel (
 id INTEGER PRIMARY KEY, image_id INTEGER NOT NULL REFERENCES gif87_image(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 color_index INTEGER NOT NULL CHECK (color_index BETWEEN 0 AND 255)
);
