CREATE TABLE jpg_document (
 id INTEGER PRIMARY KEY CHECK (id = 1), schema TEXT NOT NULL,
 width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295),
 jfif_major INTEGER NOT NULL CHECK (jfif_major BETWEEN 0 AND 255), jfif_minor INTEGER NOT NULL CHECK (jfif_minor BETWEEN 0 AND 255),
 density_units TEXT NOT NULL CHECK (density_units IN ('aspect','pixelsPerInch','pixelsPerCm')),
 x_density INTEGER NOT NULL CHECK (x_density BETWEEN 0 AND 65535), y_density INTEGER NOT NULL CHECK (y_density BETWEEN 0 AND 65535),
 sof_marker INTEGER NOT NULL CHECK (sof_marker BETWEEN 0 AND 255), arithmetic INTEGER NOT NULL CHECK (arithmetic IN (0,1)),
 restart_interval INTEGER CHECK (restart_interval BETWEEN 0 AND 65535)
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
CREATE TABLE jpg_frame (
 id INTEGER PRIMARY KEY REFERENCES jpg_document(id) CHECK (id = 1),
 precision INTEGER NOT NULL CHECK (precision BETWEEN 0 AND 255),
 width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 65535), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 65535)
);
CREATE TABLE jpg_quantizer (
 id INTEGER PRIMARY KEY CHECK (id BETWEEN 0 AND 255), document_id INTEGER NOT NULL REFERENCES jpg_document(id)
);
CREATE TABLE jpg_frame_component (
 id INTEGER PRIMARY KEY, frame_id INTEGER NOT NULL REFERENCES jpg_frame(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 component_id INTEGER NOT NULL CHECK (component_id BETWEEN 0 AND 255),
 h_sampling INTEGER NOT NULL CHECK (h_sampling BETWEEN 0 AND 255), v_sampling INTEGER NOT NULL CHECK (v_sampling BETWEEN 0 AND 255),
 quantizer_id INTEGER NOT NULL REFERENCES jpg_quantizer(id)
);
CREATE TABLE jpg_quantization_table (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES jpg_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 quantizer_id INTEGER NOT NULL REFERENCES jpg_quantizer(id), precision INTEGER NOT NULL CHECK (precision BETWEEN 0 AND 255)
);
CREATE TABLE jpg_quantization_coefficient (
 id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL REFERENCES jpg_quantization_table(id),
 zigzag_ordinal INTEGER NOT NULL CHECK (zigzag_ordinal BETWEEN 0 AND 63), coefficient INTEGER NOT NULL CHECK (coefficient BETWEEN 0 AND 65535)
);
CREATE TABLE jpg_huffman_table (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES jpg_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 table_id INTEGER NOT NULL CHECK (table_id BETWEEN 0 AND 255), table_class TEXT NOT NULL CHECK (table_class IN ('dc','ac'))
);
CREATE TABLE jpg_huffman_code_length (
 id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL REFERENCES jpg_huffman_table(id),
 length_ordinal INTEGER NOT NULL CHECK (length_ordinal BETWEEN 0 AND 15), symbol_count INTEGER NOT NULL CHECK (symbol_count BETWEEN 0 AND 255)
);
CREATE TABLE jpg_huffman_symbol (
 id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL REFERENCES jpg_huffman_table(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), symbol INTEGER NOT NULL CHECK (symbol BETWEEN 0 AND 255)
);
CREATE TABLE jpg_segment (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES jpg_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 marker INTEGER NOT NULL CHECK (marker BETWEEN 0 AND 255)
);
CREATE TABLE jpg_segment_octet (
 id INTEGER PRIMARY KEY, segment_id INTEGER NOT NULL REFERENCES jpg_segment(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), octet INTEGER NOT NULL CHECK (octet BETWEEN 0 AND 255)
);
