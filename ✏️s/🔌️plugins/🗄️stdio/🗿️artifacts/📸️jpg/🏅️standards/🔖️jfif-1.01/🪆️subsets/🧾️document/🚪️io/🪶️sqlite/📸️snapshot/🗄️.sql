CREATE TABLE jpg_document (
 id INTEGER PRIMARY KEY CHECK (id = 1), schema TEXT NOT NULL,
 width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295),
 jfif_major INTEGER NOT NULL CHECK (jfif_major BETWEEN 0 AND 255), jfif_minor INTEGER NOT NULL CHECK (jfif_minor BETWEEN 0 AND 255),
 density_units TEXT NOT NULL CHECK (density_units IN ('aspect','pixelsPerInch','pixelsPerCm')),
 x_density INTEGER NOT NULL CHECK (x_density BETWEEN 0 AND 65535), y_density INTEGER NOT NULL CHECK (y_density BETWEEN 0 AND 65535)
);
CREATE TABLE jpg_rgba_pixel (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES jpg_document(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255), green INTEGER CHECK (green BETWEEN 0 AND 255),
 blue INTEGER CHECK (blue BETWEEN 0 AND 255), alpha INTEGER CHECK (alpha BETWEEN 0 AND 255),
 CHECK (green IS NOT NULL OR (blue IS NULL AND alpha IS NULL)), CHECK (blue IS NOT NULL OR alpha IS NULL)
);
CREATE TABLE jpg_thumbnail (
 id INTEGER PRIMARY KEY REFERENCES jpg_document(id) CHECK (id = 1),
 width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 255), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 255)
);
CREATE TABLE jpg_thumbnail_rgb_pixel (
 id INTEGER PRIMARY KEY, thumbnail_id INTEGER NOT NULL REFERENCES jpg_thumbnail(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255), green INTEGER CHECK (green BETWEEN 0 AND 255), blue INTEGER CHECK (blue BETWEEN 0 AND 255),
 CHECK (green IS NOT NULL OR blue IS NULL)
);
CREATE TABLE jpg_segment (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES jpg_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 marker INTEGER NOT NULL CHECK (marker BETWEEN 0 AND 255)
);
CREATE TABLE jpg_segment_octet (
 id INTEGER PRIMARY KEY, segment_id INTEGER NOT NULL REFERENCES jpg_segment(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), octet INTEGER NOT NULL CHECK (octet BETWEEN 0 AND 255)
);
