CREATE TABLE las_document (id INTEGER PRIMARY KEY CHECK(id = 1), schema TEXT NOT NULL);
CREATE TABLE las_header (
  id INTEGER PRIMARY KEY CHECK(id = 1), document_id INTEGER NOT NULL REFERENCES las_document(id) CHECK(document_id = 1),
  version_major INTEGER NOT NULL CHECK(version_major BETWEEN 0 AND 255), version_minor INTEGER NOT NULL CHECK(version_minor BETWEEN 0 AND 255),
  system_identifier TEXT NOT NULL, generating_software TEXT NOT NULL,
  creation_day_of_year INTEGER NOT NULL CHECK(creation_day_of_year BETWEEN 0 AND 65535), creation_year INTEGER NOT NULL CHECK(creation_year BETWEEN 0 AND 65535),
  header_size INTEGER NOT NULL CHECK(header_size BETWEEN 0 AND 65535), offset_to_point_data INTEGER NOT NULL CHECK(offset_to_point_data BETWEEN 0 AND 4294967295),
  number_of_vlrs INTEGER NOT NULL CHECK(number_of_vlrs BETWEEN 0 AND 4294967295), point_data_format_id INTEGER NOT NULL CHECK(point_data_format_id BETWEEN 0 AND 255),
  point_data_record_length INTEGER NOT NULL CHECK(point_data_record_length BETWEEN 0 AND 65535), number_of_point_records INTEGER NOT NULL CHECK(number_of_point_records BETWEEN 0 AND 4294967295),
  x_scale REAL, y_scale REAL, z_scale REAL, x_offset REAL, y_offset REAL, z_offset REAL,
  max_x REAL, min_x REAL, max_y REAL, min_y REAL, max_z REAL, min_z REAL,
  x_scale_ieee754_bits INTEGER NOT NULL, x_scale_numeric_class TEXT NOT NULL CHECK(x_scale_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  y_scale_ieee754_bits INTEGER NOT NULL, y_scale_numeric_class TEXT NOT NULL CHECK(y_scale_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  z_scale_ieee754_bits INTEGER NOT NULL, z_scale_numeric_class TEXT NOT NULL CHECK(z_scale_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  x_offset_ieee754_bits INTEGER NOT NULL, x_offset_numeric_class TEXT NOT NULL CHECK(x_offset_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  y_offset_ieee754_bits INTEGER NOT NULL, y_offset_numeric_class TEXT NOT NULL CHECK(y_offset_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  z_offset_ieee754_bits INTEGER NOT NULL, z_offset_numeric_class TEXT NOT NULL CHECK(z_offset_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  max_x_ieee754_bits INTEGER NOT NULL, max_x_numeric_class TEXT NOT NULL CHECK(max_x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  min_x_ieee754_bits INTEGER NOT NULL, min_x_numeric_class TEXT NOT NULL CHECK(min_x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  max_y_ieee754_bits INTEGER NOT NULL, max_y_numeric_class TEXT NOT NULL CHECK(max_y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  min_y_ieee754_bits INTEGER NOT NULL, min_y_numeric_class TEXT NOT NULL CHECK(min_y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  max_z_ieee754_bits INTEGER NOT NULL, max_z_numeric_class TEXT NOT NULL CHECK(max_z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  min_z_ieee754_bits INTEGER NOT NULL, min_z_numeric_class TEXT NOT NULL CHECK(min_z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE las_return_histogram (id INTEGER PRIMARY KEY, header_id INTEGER NOT NULL REFERENCES las_header(id) CHECK(header_id = 1), ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 4), count INTEGER NOT NULL CHECK(count BETWEEN 0 AND 4294967295));
CREATE TABLE las_vlr (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES las_document(id) CHECK(document_id = 1), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), user_id TEXT NOT NULL, record_id INTEGER NOT NULL CHECK(record_id BETWEEN 0 AND 65535), description TEXT NOT NULL);
CREATE TABLE las_vlr_octet (id INTEGER PRIMARY KEY, vlr_id INTEGER NOT NULL REFERENCES las_vlr(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255));
CREATE TABLE las_point (
  id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES las_document(id) CHECK(document_id = 1), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
  x REAL, y REAL, z REAL, intensity INTEGER NOT NULL CHECK(intensity BETWEEN 0 AND 65535),
  return_number INTEGER NOT NULL CHECK(return_number BETWEEN 0 AND 255), number_of_returns INTEGER NOT NULL CHECK(number_of_returns BETWEEN 0 AND 255),
  scan_direction_flag INTEGER NOT NULL CHECK(scan_direction_flag IN (0,1)), edge_of_flight_line INTEGER NOT NULL CHECK(edge_of_flight_line IN (0,1)),
  classification INTEGER NOT NULL CHECK(classification BETWEEN 0 AND 255), scan_angle_rank INTEGER NOT NULL CHECK(scan_angle_rank BETWEEN -128 AND 127),
  user_data INTEGER NOT NULL CHECK(user_data BETWEEN 0 AND 255), point_source_id INTEGER NOT NULL CHECK(point_source_id BETWEEN 0 AND 65535),
  x_ieee754_bits INTEGER NOT NULL, x_numeric_class TEXT NOT NULL CHECK(x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  y_ieee754_bits INTEGER NOT NULL, y_numeric_class TEXT NOT NULL CHECK(y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  z_ieee754_bits INTEGER NOT NULL, z_numeric_class TEXT NOT NULL CHECK(z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE las_point_gps (id INTEGER PRIMARY KEY REFERENCES las_point(id), time REAL, time_ieee754_bits INTEGER NOT NULL, time_numeric_class TEXT NOT NULL CHECK(time_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE las_point_rgb (id INTEGER PRIMARY KEY REFERENCES las_point(id), red INTEGER NOT NULL CHECK(red BETWEEN 0 AND 65535), green INTEGER NOT NULL CHECK(green BETWEEN 0 AND 65535), blue INTEGER NOT NULL CHECK(blue BETWEEN 0 AND 65535));
