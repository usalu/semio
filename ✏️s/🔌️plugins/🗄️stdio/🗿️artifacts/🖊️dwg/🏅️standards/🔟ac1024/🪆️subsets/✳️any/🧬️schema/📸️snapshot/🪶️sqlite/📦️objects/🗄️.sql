CREATE TABLE dwg_placeholder (id INTEGER PRIMARY KEY REFERENCES dwg_object(id));
CREATE TABLE dwg_dictionary_variable (id INTEGER PRIMARY KEY REFERENCES dwg_object(id), value TEXT NOT NULL);
CREATE TABLE dwg_annotation_scale (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), name TEXT NOT NULL,
 paper_units_class TEXT NOT NULL CHECK(paper_units_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), paper_units_ieee754_bits INTEGER, paper_units REAL,
 drawing_units_class TEXT NOT NULL CHECK(drawing_units_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), drawing_units_ieee754_bits INTEGER, drawing_units REAL,
 is_unit_scale INTEGER NOT NULL CHECK(is_unit_scale IN (0,1)),
 CHECK((paper_units_class='finite' AND paper_units IS NOT NULL AND abs(paper_units)<=1.7976931348623157e308) OR (paper_units_class!='finite' AND paper_units IS NULL)),
 CHECK((drawing_units_class='finite' AND drawing_units IS NOT NULL AND abs(drawing_units)<=1.7976931348623157e308) OR (drawing_units_class!='finite' AND drawing_units IS NULL))
);
CREATE TABLE dwg_sort_entities_table (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), block_header_handle_high INTEGER NOT NULL CHECK(block_header_handle_high BETWEEN 0 AND 4294967295), block_header_handle_low INTEGER NOT NULL CHECK(block_header_handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_draw_order_entry (
 id INTEGER PRIMARY KEY, sort_entities_table_id INTEGER NOT NULL REFERENCES dwg_sort_entities_table(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 entity_handle_high INTEGER NOT NULL CHECK(entity_handle_high BETWEEN 0 AND 4294967295), entity_handle_low INTEGER NOT NULL CHECK(entity_handle_low BETWEEN 0 AND 4294967295),
 sort_handle_high INTEGER NOT NULL CHECK(sort_handle_high BETWEEN 0 AND 4294967295), sort_handle_low INTEGER NOT NULL CHECK(sort_handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_block_parameter_dependency_body (id INTEGER PRIMARY KEY REFERENCES dwg_object(id), name TEXT NOT NULL);
CREATE TABLE dwg_associative_dimension_dependency_body (id INTEGER PRIMARY KEY REFERENCES dwg_object(id), name TEXT NOT NULL);
CREATE TABLE dwg_block_representation_data (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), represented_block_header_handle_high INTEGER NOT NULL CHECK(represented_block_header_handle_high BETWEEN 0 AND 4294967295), represented_block_header_handle_low INTEGER NOT NULL CHECK(represented_block_header_handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_dynamic_block_purge_preventer (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), protected_block_header_handle_high INTEGER NOT NULL CHECK(protected_block_header_handle_high BETWEEN 0 AND 4294967295), protected_block_header_handle_low INTEGER NOT NULL CHECK(protected_block_header_handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_evaluation_graph (id INTEGER PRIMARY KEY REFERENCES dwg_object(id));
CREATE TABLE dwg_evaluation_graph_node (
 id INTEGER PRIMARY KEY, evaluation_graph_id INTEGER NOT NULL REFERENCES dwg_evaluation_graph(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), node_identifier INTEGER NOT NULL CHECK(node_identifier BETWEEN 0 AND 4294967295),
 expression_handle_high INTEGER NOT NULL CHECK(expression_handle_high BETWEEN 0 AND 4294967295), expression_handle_low INTEGER NOT NULL CHECK(expression_handle_low BETWEEN 0 AND 4294967295)
);
CREATE TABLE dwg_evaluation_graph_edge (
 id INTEGER PRIMARY KEY, evaluation_graph_id INTEGER NOT NULL REFERENCES dwg_evaluation_graph(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 from_node_identifier INTEGER NOT NULL CHECK(from_node_identifier BETWEEN 0 AND 4294967295), to_node_identifier INTEGER NOT NULL CHECK(to_node_identifier BETWEEN 0 AND 4294967295),
 reference_count INTEGER NOT NULL CHECK(reference_count BETWEEN 0 AND 4294967295), invertible INTEGER NOT NULL CHECK(invertible IN (0,1)), suppressed INTEGER NOT NULL CHECK(suppressed IN (0,1))
);
