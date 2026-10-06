CREATE TABLE bmp_document (
 id INTEGER PRIMARY KEY CHECK(id=1),
 schema TEXT NOT NULL,
 state TEXT NOT NULL CHECK(state IN ('valid_layout','literal_octets')),
 diagnostic TEXT NOT NULL,
 CHECK((state='valid_layout' AND diagnostic='') OR (state='literal_octets' AND diagnostic<>''))
);
CREATE TABLE bmp_file_header (
 id INTEGER PRIMARY KEY CHECK(id=1), document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 declared_file_size INTEGER NOT NULL CHECK(declared_file_size BETWEEN 0 AND 4294967295),
 reserved_1 INTEGER NOT NULL CHECK(reserved_1 BETWEEN 0 AND 65535),
 reserved_2 INTEGER NOT NULL CHECK(reserved_2 BETWEEN 0 AND 65535),
 pixel_offset INTEGER NOT NULL CHECK(pixel_offset BETWEEN 0 AND 4294967295)
);
CREATE TABLE bmp_info_header (
 id INTEGER PRIMARY KEY CHECK(id=1), document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 header_size INTEGER NOT NULL CHECK(header_size=40),
 width INTEGER NOT NULL CHECK(width BETWEEN 0 AND 2147483647),
 signed_height INTEGER NOT NULL CHECK(signed_height BETWEEN -2147483647 AND 2147483647),
 planes INTEGER NOT NULL CHECK(planes=1),
 bits_per_pixel INTEGER NOT NULL CHECK(bits_per_pixel IN (1,4,8,16,24,32)),
 compression INTEGER NOT NULL CHECK(compression IN (0,3)),
 declared_image_size INTEGER NOT NULL CHECK(declared_image_size BETWEEN 0 AND 4294967295),
 x_pixels_per_meter INTEGER NOT NULL CHECK(x_pixels_per_meter BETWEEN -2147483648 AND 2147483647),
 y_pixels_per_meter INTEGER NOT NULL CHECK(y_pixels_per_meter BETWEEN -2147483648 AND 2147483647),
 colors_used INTEGER NOT NULL CHECK(colors_used BETWEEN 0 AND 4294967295),
 colors_important INTEGER NOT NULL CHECK(colors_important BETWEEN 0 AND 4294967295)
);
CREATE TABLE bmp_channel_mask (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 2),
 channel TEXT NOT NULL CHECK(channel IN ('red','green','blue')),
 mask INTEGER NOT NULL CHECK(mask BETWEEN 1 AND 4294967295)
);
CREATE TABLE bmp_palette_entry (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 blue INTEGER NOT NULL CHECK(blue BETWEEN 0 AND 255), green INTEGER NOT NULL CHECK(green BETWEEN 0 AND 255),
 red INTEGER NOT NULL CHECK(red BETWEEN 0 AND 255), reserved INTEGER NOT NULL CHECK(reserved BETWEEN 0 AND 255)
);
CREATE TABLE bmp_pixel_index (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 x INTEGER NOT NULL CHECK(x>=0), y INTEGER NOT NULL CHECK(y>=0),
 palette_index INTEGER NOT NULL CHECK(palette_index BETWEEN 0 AND 255)
);
CREATE TABLE bmp_pixel_sample (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 x INTEGER NOT NULL CHECK(x>=0), y INTEGER NOT NULL CHECK(y>=0),
 red INTEGER NOT NULL CHECK(red BETWEEN 0 AND 4294967295), green INTEGER NOT NULL CHECK(green BETWEEN 0 AND 4294967295),
 blue INTEGER NOT NULL CHECK(blue BETWEEN 0 AND 4294967295), unused_bits INTEGER NOT NULL CHECK(unused_bits BETWEEN 0 AND 4294967295)
);
CREATE TABLE bmp_row_tail_bits (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 y INTEGER NOT NULL CHECK(y>=0), unused_bits INTEGER NOT NULL CHECK(unused_bits BETWEEN 0 AND 127)
);
CREATE TABLE bmp_row_padding_octet (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 y INTEGER NOT NULL CHECK(y>=0), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)
);
CREATE TABLE bmp_gap_octet (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0), value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)
);
CREATE TABLE bmp_trailer_octet (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0), value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)
);
CREATE TABLE bmp_literal_octet (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0), value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)
);
