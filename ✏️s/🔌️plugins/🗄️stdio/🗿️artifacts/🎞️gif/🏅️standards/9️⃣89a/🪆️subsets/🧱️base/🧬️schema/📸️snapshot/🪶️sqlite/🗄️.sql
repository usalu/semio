CREATE TABLE gif89_document (
 id INTEGER PRIMARY KEY CHECK (id = 1), schema TEXT NOT NULL,
 screen_width INTEGER NOT NULL CHECK (screen_width BETWEEN 0 AND 4294967295), screen_height INTEGER NOT NULL CHECK (screen_height BETWEEN 0 AND 4294967295),
 background_color_index INTEGER NOT NULL CHECK (background_color_index BETWEEN 0 AND 255), pixel_aspect_ratio INTEGER NOT NULL CHECK (pixel_aspect_ratio BETWEEN 0 AND 255),
 loop_count INTEGER CHECK (loop_count BETWEEN 0 AND 65535)
);
CREATE TABLE gif89_global_palette (
 id INTEGER PRIMARY KEY REFERENCES gif89_document(id) CHECK (id = 1), sorted INTEGER NOT NULL CHECK (sorted IN (0,1))
);
CREATE TABLE gif89_global_color (
 id INTEGER PRIMARY KEY, palette_id INTEGER NOT NULL REFERENCES gif89_global_palette(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255), green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255), blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255)
);
CREATE TABLE gif89_frame (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES gif89_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 left_position INTEGER NOT NULL CHECK (left_position BETWEEN 0 AND 4294967295), top_position INTEGER NOT NULL CHECK (top_position BETWEEN 0 AND 4294967295),
 width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295), interlace INTEGER NOT NULL CHECK (interlace IN (0,1)),
 delay_centiseconds INTEGER NOT NULL CHECK (delay_centiseconds BETWEEN 0 AND 65535),
 disposal TEXT NOT NULL CHECK (disposal IN ('unspecified','do_not_dispose','restore_to_background','restore_to_previous')),
 transparent_color_index INTEGER CHECK (transparent_color_index BETWEEN 0 AND 255), user_input INTEGER NOT NULL CHECK (user_input IN (0,1))
);
CREATE TABLE gif89_local_palette (
 id INTEGER PRIMARY KEY REFERENCES gif89_frame(id), sorted INTEGER NOT NULL CHECK (sorted IN (0,1))
);
CREATE TABLE gif89_local_color (
 id INTEGER PRIMARY KEY, palette_id INTEGER NOT NULL REFERENCES gif89_local_palette(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255), green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255), blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255)
);
CREATE TABLE gif89_pixel (
 id INTEGER PRIMARY KEY, frame_id INTEGER NOT NULL REFERENCES gif89_frame(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), color_index INTEGER NOT NULL CHECK (color_index BETWEEN 0 AND 255)
);
CREATE TABLE gif89_plain_text (
 id INTEGER PRIMARY KEY REFERENCES gif89_frame(id),
 left_position INTEGER NOT NULL CHECK (left_position BETWEEN 0 AND 4294967295), top_position INTEGER NOT NULL CHECK (top_position BETWEEN 0 AND 4294967295),
 width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295),
 cell_width INTEGER NOT NULL CHECK (cell_width BETWEEN 0 AND 255), cell_height INTEGER NOT NULL CHECK (cell_height BETWEEN 0 AND 255),
 foreground_color_index INTEGER NOT NULL CHECK (foreground_color_index BETWEEN 0 AND 255), background_color_index INTEGER NOT NULL CHECK (background_color_index BETWEEN 0 AND 255), text TEXT NOT NULL
);
CREATE TABLE gif89_comment (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES gif89_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), text TEXT NOT NULL
);
CREATE TABLE gif89_application (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES gif89_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 identifier_0 INTEGER NOT NULL CHECK (identifier_0 BETWEEN 0 AND 255), identifier_1 INTEGER NOT NULL CHECK (identifier_1 BETWEEN 0 AND 255),
 identifier_2 INTEGER NOT NULL CHECK (identifier_2 BETWEEN 0 AND 255), identifier_3 INTEGER NOT NULL CHECK (identifier_3 BETWEEN 0 AND 255),
 identifier_4 INTEGER NOT NULL CHECK (identifier_4 BETWEEN 0 AND 255), identifier_5 INTEGER NOT NULL CHECK (identifier_5 BETWEEN 0 AND 255),
 identifier_6 INTEGER NOT NULL CHECK (identifier_6 BETWEEN 0 AND 255), identifier_7 INTEGER NOT NULL CHECK (identifier_7 BETWEEN 0 AND 255),
 authentication_0 INTEGER NOT NULL CHECK (authentication_0 BETWEEN 0 AND 255), authentication_1 INTEGER NOT NULL CHECK (authentication_1 BETWEEN 0 AND 255), authentication_2 INTEGER NOT NULL CHECK (authentication_2 BETWEEN 0 AND 255)
);
CREATE TABLE gif89_application_byte (
 id INTEGER PRIMARY KEY, application_id INTEGER NOT NULL REFERENCES gif89_application(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), value INTEGER NOT NULL CHECK (value BETWEEN 0 AND 255)
);
