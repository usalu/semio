CREATE TABLE dwg_block_element (id INTEGER PRIMARY KEY REFERENCES dwg_evaluation_expression(id), name TEXT NOT NULL);
CREATE TABLE dwg_block_grip (
 id INTEGER PRIMARY KEY REFERENCES dwg_block_element(id), insertion_cycling INTEGER NOT NULL CHECK(insertion_cycling IN (0,1)), insertion_cycling_weight INTEGER NOT NULL CHECK(insertion_cycling_weight BETWEEN -2147483648 AND 2147483647),
 updated_x_node_identifier INTEGER NOT NULL CHECK(updated_x_node_identifier BETWEEN 0 AND 4294967295), updated_x_expression_name TEXT NOT NULL,
 updated_y_node_identifier INTEGER NOT NULL CHECK(updated_y_node_identifier BETWEEN 0 AND 4294967295), updated_y_expression_name TEXT NOT NULL
);
CREATE TABLE dwg_block_grip_location_coordinate (
 id INTEGER PRIMARY KEY, block_grip_id INTEGER NOT NULL REFERENCES dwg_block_grip(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_two_point_parameter (
 id INTEGER PRIMARY KEY REFERENCES dwg_block_element(id), show_properties INTEGER NOT NULL CHECK(show_properties IN (0,1)), chain_actions INTEGER NOT NULL CHECK(chain_actions IN (0,1)), base_location TEXT NOT NULL CHECK(base_location IN ('start_point','midpoint'))
);
CREATE TABLE dwg_block_two_point_definition_base_coordinate (
 id INTEGER PRIMARY KEY, two_point_parameter_id INTEGER NOT NULL REFERENCES dwg_block_two_point_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_two_point_definition_end_coordinate (
 id INTEGER PRIMARY KEY, two_point_parameter_id INTEGER NOT NULL REFERENCES dwg_block_two_point_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_parameter_property (
 id INTEGER PRIMARY KEY, object_id INTEGER NOT NULL REFERENCES dwg_object(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0)
);
CREATE TABLE dwg_block_parameter_connection (
 id INTEGER PRIMARY KEY, parameter_property_id INTEGER NOT NULL REFERENCES dwg_block_parameter_property(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), code INTEGER NOT NULL CHECK(code BETWEEN 0 AND 4294967295), name TEXT NOT NULL
);
CREATE TABLE dwg_property_expression_reference (
 id INTEGER PRIMARY KEY, two_point_parameter_id INTEGER NOT NULL REFERENCES dwg_block_two_point_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 property_index INTEGER NOT NULL CHECK(property_index BETWEEN 0 AND 4294967295), node_identifier INTEGER NOT NULL CHECK(node_identifier BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_block_one_point_parameter (id INTEGER PRIMARY KEY REFERENCES dwg_block_element(id), show_properties INTEGER NOT NULL CHECK(show_properties IN (0,1)), chain_actions INTEGER NOT NULL CHECK(chain_actions IN (0,1)));
CREATE TABLE dwg_block_one_point_definition_coordinate (
 id INTEGER PRIMARY KEY, one_point_parameter_id INTEGER NOT NULL REFERENCES dwg_block_one_point_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_linear_parameter (
 id INTEGER PRIMARY KEY REFERENCES dwg_block_two_point_parameter(id), distance_name TEXT NOT NULL, distance_description TEXT NOT NULL,
 label_offset_class TEXT NOT NULL CHECK(label_offset_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), label_offset_ieee754_bits INTEGER, label_offset REAL, CHECK((label_offset_class='finite')=(label_offset IS NOT NULL)), CHECK(label_offset IS NULL OR abs(label_offset)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_linear_allowed_value (
 id INTEGER PRIMARY KEY, linear_parameter_id INTEGER NOT NULL REFERENCES dwg_block_linear_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 value_class TEXT NOT NULL CHECK(value_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), value_ieee754_bits INTEGER, value REAL, CHECK((value_class='finite')=(value IS NOT NULL)), CHECK(value IS NULL OR abs(value)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_linear_grip (id INTEGER PRIMARY KEY REFERENCES dwg_block_grip(id));
CREATE TABLE dwg_block_linear_grip_orientation_coordinate (
 id INTEGER PRIMARY KEY, linear_grip_id INTEGER NOT NULL REFERENCES dwg_block_linear_grip(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_flip_grip (id INTEGER PRIMARY KEY REFERENCES dwg_block_grip(id), updated_flip_node_identifier INTEGER NOT NULL CHECK(updated_flip_node_identifier BETWEEN 0 AND 4294967295), updated_flip_expression_name TEXT NOT NULL);
CREATE TABLE dwg_block_flip_grip_orientation_coordinate (
 id INTEGER PRIMARY KEY, flip_grip_id INTEGER NOT NULL REFERENCES dwg_block_flip_grip(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_visibility_grip (id INTEGER PRIMARY KEY REFERENCES dwg_block_grip(id));
CREATE TABLE dwg_block_alignment_parameter (id INTEGER PRIMARY KEY REFERENCES dwg_block_two_point_parameter(id), updated_grip_node_identifier INTEGER NOT NULL CHECK(updated_grip_node_identifier BETWEEN 0 AND 4294967295), align_perpendicular INTEGER NOT NULL CHECK(align_perpendicular IN (0,1)));
CREATE TABLE dwg_block_alignment_grip (id INTEGER PRIMARY KEY REFERENCES dwg_block_grip(id), first_location_node_identifier INTEGER NOT NULL CHECK(first_location_node_identifier BETWEEN 0 AND 4294967295), second_location_node_identifier INTEGER NOT NULL CHECK(second_location_node_identifier BETWEEN 0 AND 4294967295));
CREATE TABLE dwg_block_alignment_grip_orientation_coordinate (
 id INTEGER PRIMARY KEY, alignment_grip_id INTEGER NOT NULL REFERENCES dwg_block_alignment_grip(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_base_point_parameter (id INTEGER PRIMARY KEY REFERENCES dwg_block_one_point_parameter(id));
CREATE TABLE dwg_block_base_point_coordinate (
 id INTEGER PRIMARY KEY, base_point_parameter_id INTEGER NOT NULL REFERENCES dwg_block_base_point_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_base_point_base_coordinate (
 id INTEGER PRIMARY KEY, base_point_parameter_id INTEGER NOT NULL REFERENCES dwg_block_base_point_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_linear_constraint_parameter (
 id INTEGER PRIMARY KEY REFERENCES dwg_block_two_point_parameter(id), direction TEXT NOT NULL CHECK(direction IN ('vertical','horizontal')), displacement_grip_node_identifier INTEGER NOT NULL CHECK(displacement_grip_node_identifier BETWEEN 0 AND 4294967295),
 dependency_handle_high INTEGER NOT NULL CHECK(dependency_handle_high BETWEEN 0 AND 4294967295), dependency_handle_low INTEGER NOT NULL CHECK(dependency_handle_low BETWEEN 0 AND 4294967295), expression_name TEXT NOT NULL, expression_description TEXT NOT NULL,
 value_class TEXT NOT NULL CHECK(value_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), value_ieee754_bits INTEGER, value REAL, CHECK((value_class='finite')=(value IS NOT NULL)), CHECK(value IS NULL OR abs(value)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_linear_constraint_allowed_value (
 id INTEGER PRIMARY KEY, linear_constraint_parameter_id INTEGER NOT NULL REFERENCES dwg_block_linear_constraint_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 value_class TEXT NOT NULL CHECK(value_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), value_ieee754_bits INTEGER, value REAL, CHECK((value_class='finite')=(value IS NOT NULL)), CHECK(value IS NULL OR abs(value)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_flip_parameter (
 id INTEGER PRIMARY KEY REFERENCES dwg_evaluation_expression(id), name TEXT NOT NULL, show_properties INTEGER NOT NULL CHECK(show_properties IN (0,1)), chain_actions INTEGER NOT NULL CHECK(chain_actions IN (0,1)), base_location TEXT NOT NULL CHECK(base_location IN ('start_point','midpoint')),
 label TEXT NOT NULL, description TEXT NOT NULL, base_label TEXT NOT NULL, flipped_label TEXT NOT NULL, updated_flip_node_identifier INTEGER NOT NULL CHECK(updated_flip_node_identifier BETWEEN 0 AND 4294967295), updated_flip_expression_name TEXT NOT NULL
);
CREATE TABLE dwg_block_flip_definition_base_coordinate (
 id INTEGER PRIMARY KEY, flip_parameter_id INTEGER NOT NULL REFERENCES dwg_block_flip_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_flip_definition_end_coordinate (
 id INTEGER PRIMARY KEY, flip_parameter_id INTEGER NOT NULL REFERENCES dwg_block_flip_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_flip_label_point_coordinate (
 id INTEGER PRIMARY KEY, flip_parameter_id INTEGER NOT NULL REFERENCES dwg_block_flip_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_visibility_parameter (
 id INTEGER PRIMARY KEY REFERENCES dwg_evaluation_expression(id), element_name TEXT NOT NULL, show_properties INTEGER NOT NULL CHECK(show_properties IN (0,1)), chain_actions INTEGER NOT NULL CHECK(chain_actions IN (0,1)),
 updated_visibility_node_identifier INTEGER NOT NULL CHECK(updated_visibility_node_identifier BETWEEN 0 AND 4294967295), initialized INTEGER NOT NULL CHECK(initialized IN (0,1)), name TEXT NOT NULL, description TEXT NOT NULL, evaluation_history TEXT NOT NULL CHECK(evaluation_history IN ('stateless','required'))
);
CREATE TABLE dwg_block_visibility_definition_coordinate (
 id INTEGER PRIMARY KEY, visibility_parameter_id INTEGER NOT NULL REFERENCES dwg_block_visibility_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL, CHECK((coordinate_class='finite')=(coordinate IS NOT NULL)), CHECK(coordinate IS NULL OR abs(coordinate)<=1.7976931348623157e308)
);
CREATE TABLE dwg_block_visibility_eligible_handle (
 id INTEGER PRIMARY KEY, visibility_parameter_id INTEGER NOT NULL REFERENCES dwg_block_visibility_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 handle_high INTEGER NOT NULL CHECK(handle_high BETWEEN 0 AND 4294967295), handle_low INTEGER NOT NULL CHECK(handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_visibility_state (id INTEGER PRIMARY KEY, visibility_parameter_id INTEGER NOT NULL REFERENCES dwg_block_visibility_parameter(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), name TEXT NOT NULL);
CREATE TABLE dwg_visibility_state_visible_handle (
 id INTEGER PRIMARY KEY, visibility_state_id INTEGER NOT NULL REFERENCES dwg_visibility_state(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), handle_high INTEGER NOT NULL CHECK(handle_high BETWEEN 0 AND 4294967295), handle_low INTEGER NOT NULL CHECK(handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_visibility_state_expression_handle (
 id INTEGER PRIMARY KEY, visibility_state_id INTEGER NOT NULL REFERENCES dwg_visibility_state(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), handle_high INTEGER NOT NULL CHECK(handle_high BETWEEN 0 AND 4294967295), handle_low INTEGER NOT NULL CHECK(handle_low BETWEEN 0 AND 4294967295)
);
