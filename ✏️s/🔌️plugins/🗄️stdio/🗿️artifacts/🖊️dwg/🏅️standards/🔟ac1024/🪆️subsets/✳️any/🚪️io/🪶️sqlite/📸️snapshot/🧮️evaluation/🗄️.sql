CREATE TABLE dwg_evaluation_expression (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), parent_identifier INTEGER NOT NULL CHECK(parent_identifier BETWEEN -2147483648 AND 2147483647),
 major_version INTEGER NOT NULL CHECK(major_version BETWEEN 0 AND 4294967295), minor_version INTEGER NOT NULL CHECK(minor_version BETWEEN 0 AND 4294967295), node_identifier INTEGER NOT NULL CHECK(node_identifier BETWEEN 0 AND 4294967295),
 value_kind TEXT NOT NULL CHECK(value_kind IN ('empty','double','point_group_10','point_group_11','string','integer32','object_reference','integer16')),
 double_class TEXT CHECK(double_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), double_value_ieee754_bits INTEGER, double_value REAL, string_value TEXT,
 integer32_value INTEGER CHECK(integer32_value BETWEEN -2147483648 AND 2147483647),
 object_reference_high INTEGER CHECK(object_reference_high BETWEEN 0 AND 4294967295), object_reference_low INTEGER CHECK(object_reference_low BETWEEN 0 AND 4294967295),
 integer16_value INTEGER CHECK(integer16_value BETWEEN -32768 AND 32767),
 CHECK((double_class IS NULL AND double_value IS NULL) OR (double_class='finite' AND double_value IS NOT NULL AND abs(double_value)<=1.7976931348623157e308) OR (double_class!='finite' AND double_value IS NULL)),
 CHECK((value_kind='double')=(double_class IS NOT NULL)), CHECK((value_kind='string')=(string_value IS NOT NULL)), CHECK((value_kind='integer32')=(integer32_value IS NOT NULL)),
 CHECK((value_kind='object_reference')=(object_reference_high IS NOT NULL)), CHECK((object_reference_high IS NULL)=(object_reference_low IS NULL)), CHECK((value_kind='integer16')=(integer16_value IS NOT NULL))
);
CREATE TABLE dwg_evaluation_point_group_10_coordinate (
 id INTEGER PRIMARY KEY, evaluation_expression_id INTEGER NOT NULL REFERENCES dwg_evaluation_expression(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL,
 CHECK((coordinate_class='finite' AND coordinate IS NOT NULL AND abs(coordinate)<=1.7976931348623157e308) OR (coordinate_class!='finite' AND coordinate IS NULL))
);
CREATE TABLE dwg_evaluation_point_group_11_coordinate (
 id INTEGER PRIMARY KEY, evaluation_expression_id INTEGER NOT NULL REFERENCES dwg_evaluation_expression(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL,
 CHECK((coordinate_class='finite' AND coordinate IS NOT NULL AND abs(coordinate)<=1.7976931348623157e308) OR (coordinate_class!='finite' AND coordinate IS NULL))
);
CREATE TABLE dwg_block_grip_location_component (
 id INTEGER PRIMARY KEY REFERENCES dwg_evaluation_expression(id), grip_type INTEGER NOT NULL CHECK(grip_type BETWEEN 0 AND 4294967295), grip_expression TEXT NOT NULL
);
CREATE TABLE dwg_dynamic_block_proxy_node (id INTEGER PRIMARY KEY REFERENCES dwg_evaluation_expression(id));
