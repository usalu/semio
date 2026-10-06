CREATE TABLE dwg_assoc_2d_constraint_group (
 id INTEGER PRIMARY KEY REFERENCES dwg_associative_action(id), do_not_check_newly_added_constraints INTEGER NOT NULL CHECK(do_not_check_newly_added_constraints IN (0,1))
);
CREATE TABLE dwg_constraint_work_plane_vector (id INTEGER PRIMARY KEY, constraint_group_id INTEGER NOT NULL REFERENCES dwg_assoc_2d_constraint_group(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0));
CREATE TABLE dwg_constraint_work_plane_coordinate (
 id INTEGER PRIMARY KEY, work_plane_vector_id INTEGER NOT NULL REFERENCES dwg_constraint_work_plane_vector(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL CHECK((coordinate_class='finite')=(coordinate IS NOT NULL))
);
CREATE TABLE dwg_constraint_group_member_handle (
 id INTEGER PRIMARY KEY, constraint_group_id INTEGER NOT NULL REFERENCES dwg_assoc_2d_constraint_group(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), handle_high INTEGER NOT NULL CHECK(handle_high BETWEEN 0 AND 4294967295), handle_low INTEGER NOT NULL CHECK(handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_constraint_node (
 id INTEGER PRIMARY KEY, constraint_group_id INTEGER NOT NULL REFERENCES dwg_assoc_2d_constraint_group(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 kind TEXT NOT NULL CHECK(kind IN ('implicit_point','point_curve','bounded_line','point_coincidence','distance','perpendicular','horizontal','parallel','midpoint','equal_length','colinear','datum_line','fixed','vertical')), node_identifier INTEGER NOT NULL CHECK(node_identifier BETWEEN -2147483648 AND 2147483647)
);
CREATE TABLE dwg_constraint_connected_node_identifier (
 id INTEGER PRIMARY KEY, constraint_node_id INTEGER NOT NULL REFERENCES dwg_constraint_node(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), node_identifier INTEGER NOT NULL CHECK(node_identifier BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_geometric_constraint (
 id INTEGER PRIMARY KEY REFERENCES dwg_constraint_node(id), owner_node_identifier INTEGER NOT NULL CHECK(owner_node_identifier BETWEEN 0 AND 4294967295), implied INTEGER NOT NULL CHECK(implied IN (0,1)), active INTEGER NOT NULL CHECK(active IN (0,1))
);
CREATE TABLE dwg_constraint_geometry (
 id INTEGER PRIMARY KEY REFERENCES dwg_constraint_node(id), geometry_dependency_handle_high INTEGER CHECK(geometry_dependency_handle_high BETWEEN 0 AND 4294967295), geometry_dependency_handle_low INTEGER CHECK(geometry_dependency_handle_low BETWEEN 0 AND 4294967295), geometry_node_identifier INTEGER NOT NULL CHECK(geometry_node_identifier BETWEEN 0 AND 4294967295), CHECK((geometry_dependency_handle_high IS NULL)=(geometry_dependency_handle_low IS NULL))
);
CREATE TABLE dwg_explicit_constraint (
 id INTEGER PRIMARY KEY REFERENCES dwg_geometric_constraint(id), value_dependency_handle_high INTEGER NOT NULL CHECK(value_dependency_handle_high BETWEEN 0 AND 4294967295), value_dependency_handle_low INTEGER NOT NULL CHECK(value_dependency_handle_low BETWEEN 0 AND 4294967295), dimension_dependency_handle_high INTEGER NOT NULL CHECK(dimension_dependency_handle_high BETWEEN 0 AND 4294967295), dimension_dependency_handle_low INTEGER NOT NULL CHECK(dimension_dependency_handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_constrained_implicit_point (
 id INTEGER PRIMARY KEY REFERENCES dwg_constraint_geometry(id), point_present INTEGER NOT NULL CHECK(point_present IN (0,1)), point_kind INTEGER NOT NULL CHECK(point_kind BETWEEN 0 AND 255), point_index INTEGER NOT NULL CHECK(point_index BETWEEN -2147483648 AND 2147483647), curve_node_identifier INTEGER NOT NULL CHECK(curve_node_identifier BETWEEN -2147483648 AND 2147483647)
);
CREATE TABLE dwg_constrained_bounded_line (id INTEGER PRIMARY KEY REFERENCES dwg_constraint_geometry(id), ray INTEGER NOT NULL CHECK(ray IN (0,1)), bounded INTEGER NOT NULL CHECK(bounded IN (0,1)));
CREATE TABLE dwg_distance_constraint (id INTEGER PRIMARY KEY REFERENCES dwg_explicit_constraint(id), direction_kind INTEGER NOT NULL CHECK(direction_kind BETWEEN 0 AND 255), direction_present INTEGER NOT NULL CHECK(direction_present IN (0,1)));
CREATE TABLE dwg_axis_constraint (id INTEGER PRIMARY KEY REFERENCES dwg_geometric_constraint(id), datum_line_index INTEGER NOT NULL CHECK(datum_line_index BETWEEN -2147483648 AND 2147483647));
CREATE TABLE dwg_constrained_datum_line (id INTEGER PRIMARY KEY REFERENCES dwg_constraint_geometry(id));
CREATE TABLE dwg_constraint_coordinate (
 id INTEGER PRIMARY KEY, constraint_node_id INTEGER NOT NULL REFERENCES dwg_constraint_node(id), vector TEXT NOT NULL CHECK(vector IN ('point','origin','direction','start_point','end_point')),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0), component_ordinal INTEGER NOT NULL CHECK(component_ordinal>=0), coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL CHECK((coordinate_class='finite')=(coordinate IS NOT NULL))
);
