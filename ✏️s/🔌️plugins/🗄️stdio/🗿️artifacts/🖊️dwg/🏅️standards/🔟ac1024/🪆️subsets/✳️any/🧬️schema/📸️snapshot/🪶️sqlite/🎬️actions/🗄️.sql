CREATE TABLE dwg_block_action (id INTEGER PRIMARY KEY REFERENCES dwg_evaluation_expression(id), name TEXT NOT NULL);
CREATE TABLE dwg_block_action_display_coordinate (
 id INTEGER PRIMARY KEY, block_action_id INTEGER NOT NULL REFERENCES dwg_block_action(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_action_dependency (
 id INTEGER PRIMARY KEY, block_action_id INTEGER NOT NULL REFERENCES dwg_block_action(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), object_handle_high INTEGER NOT NULL CHECK(object_handle_high BETWEEN 0 AND 4294967295), object_handle_low INTEGER NOT NULL CHECK(object_handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_block_action_node_identifier (id INTEGER PRIMARY KEY, block_action_id INTEGER NOT NULL REFERENCES dwg_block_action(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), node_identifier INTEGER NOT NULL CHECK(node_identifier BETWEEN 0 AND 4294967295));
CREATE TABLE dwg_block_move_action (
 id INTEGER PRIMARY KEY REFERENCES dwg_block_action(id), x_node_identifier INTEGER NOT NULL CHECK(x_node_identifier BETWEEN 0 AND 4294967295), x_name TEXT NOT NULL, y_node_identifier INTEGER NOT NULL CHECK(y_node_identifier BETWEEN 0 AND 4294967295), y_name TEXT NOT NULL,
 distance_multiplier_class TEXT NOT NULL CHECK(distance_multiplier_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), distance_multiplier_ieee754_bits INTEGER, distance_multiplier REAL,
 angle_offset_class TEXT NOT NULL CHECK(angle_offset_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), angle_offset_ieee754_bits INTEGER, angle_offset REAL, coordinate_mode TEXT NOT NULL CHECK(coordinate_mode='cartesian_xy'),
 CHECK((distance_multiplier_class='finite')=(distance_multiplier IS NOT NULL)), CHECK(distance_multiplier IS NULL OR abs(distance_multiplier)<=1.7976931348623157e308), CHECK((angle_offset_class='finite')=(angle_offset IS NOT NULL)), CHECK(angle_offset IS NULL OR abs(angle_offset)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_stretch_action (
 id INTEGER PRIMARY KEY REFERENCES dwg_block_action(id), x_node_identifier INTEGER NOT NULL CHECK(x_node_identifier BETWEEN 0 AND 4294967295), x_name TEXT NOT NULL, y_node_identifier INTEGER NOT NULL CHECK(y_node_identifier BETWEEN 0 AND 4294967295), y_name TEXT NOT NULL,
 distance_multiplier_class TEXT NOT NULL CHECK(distance_multiplier_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), distance_multiplier_ieee754_bits INTEGER, distance_multiplier REAL,
 angle_offset_class TEXT NOT NULL CHECK(angle_offset_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), angle_offset_ieee754_bits INTEGER, angle_offset REAL, coordinate_mode TEXT NOT NULL CHECK(coordinate_mode='cartesian_xy'),
 CHECK((distance_multiplier_class='finite')=(distance_multiplier IS NOT NULL)), CHECK(distance_multiplier IS NULL OR abs(distance_multiplier)<=1.7976931348623157e308), CHECK((angle_offset_class='finite')=(angle_offset IS NOT NULL)), CHECK(angle_offset IS NULL OR abs(angle_offset)<=1.7976931348623157e308)
);
CREATE TABLE dwg_stretch_point (id INTEGER PRIMARY KEY, stretch_action_id INTEGER NOT NULL REFERENCES dwg_block_stretch_action(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0));
CREATE TABLE dwg_stretch_point_coordinate (
 id INTEGER PRIMARY KEY, stretch_point_id INTEGER NOT NULL REFERENCES dwg_stretch_point(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_stretch_selection (
 id INTEGER PRIMARY KEY, stretch_action_id INTEGER NOT NULL REFERENCES dwg_block_stretch_action(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), object_handle_high INTEGER NOT NULL CHECK(object_handle_high BETWEEN 0 AND 4294967295), object_handle_low INTEGER NOT NULL CHECK(object_handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_stretch_selection_vertex_index (id INTEGER PRIMARY KEY, stretch_selection_id INTEGER NOT NULL REFERENCES dwg_stretch_selection(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), vertex_index INTEGER NOT NULL CHECK(vertex_index BETWEEN 0 AND 4294967295));
CREATE TABLE dwg_stretch_selector (id INTEGER PRIMARY KEY, stretch_action_id INTEGER NOT NULL REFERENCES dwg_block_stretch_action(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), node_identifier INTEGER NOT NULL CHECK(node_identifier BETWEEN 0 AND 4294967295));
CREATE TABLE dwg_stretch_selector_point_index (id INTEGER PRIMARY KEY, stretch_selector_id INTEGER NOT NULL REFERENCES dwg_stretch_selector(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), point_index INTEGER NOT NULL CHECK(point_index BETWEEN 0 AND 4294967295));
CREATE TABLE dwg_block_action_base_point (
 id INTEGER PRIMARY KEY REFERENCES dwg_block_action(id), x_base_node_identifier INTEGER NOT NULL CHECK(x_base_node_identifier BETWEEN 0 AND 4294967295), x_base_name TEXT NOT NULL, y_base_node_identifier INTEGER NOT NULL CHECK(y_base_node_identifier BETWEEN 0 AND 4294967295), y_base_name TEXT NOT NULL, dependent INTEGER NOT NULL CHECK(dependent IN (0,1))
);
CREATE TABLE dwg_block_action_offset_coordinate (
 id INTEGER PRIMARY KEY, action_base_point_id INTEGER NOT NULL REFERENCES dwg_block_action_base_point(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_action_base_point_coordinate (
 id INTEGER PRIMARY KEY, action_base_point_id INTEGER NOT NULL REFERENCES dwg_block_action_base_point(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_scale_action (
 id INTEGER PRIMARY KEY REFERENCES dwg_block_action_base_point(id), uniform_scale_node_identifier INTEGER NOT NULL CHECK(uniform_scale_node_identifier BETWEEN 0 AND 4294967295), uniform_scale_name TEXT NOT NULL,
 x_scale_node_identifier INTEGER NOT NULL CHECK(x_scale_node_identifier BETWEEN 0 AND 4294967295), x_scale_name TEXT NOT NULL, y_scale_node_identifier INTEGER NOT NULL CHECK(y_scale_node_identifier BETWEEN 0 AND 4294967295), y_scale_name TEXT NOT NULL, mode TEXT NOT NULL CHECK(mode='xy')
);
CREATE TABLE dwg_block_flip_action (
 id INTEGER PRIMARY KEY REFERENCES dwg_block_action(id), flip_node_identifier INTEGER NOT NULL CHECK(flip_node_identifier BETWEEN 0 AND 4294967295), flip_name TEXT NOT NULL,
 updated_flip_node_identifier INTEGER NOT NULL CHECK(updated_flip_node_identifier BETWEEN 0 AND 4294967295), updated_flip_name TEXT NOT NULL, updated_base_node_identifier INTEGER NOT NULL CHECK(updated_base_node_identifier BETWEEN 0 AND 4294967295), updated_base_name TEXT NOT NULL, updated_end_node_identifier INTEGER NOT NULL CHECK(updated_end_node_identifier BETWEEN 0 AND 4294967295), updated_end_name TEXT NOT NULL
);
