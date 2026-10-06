CREATE TABLE chart_document (
 id INTEGER PRIMARY KEY CHECK(id > 0),
 width REAL NOT NULL, width_kind TEXT NOT NULL, width_exact TEXT NOT NULL,
 height REAL NOT NULL, height_kind TEXT NOT NULL, height_exact TEXT NOT NULL,
 language TEXT,
 tables_present INTEGER NOT NULL CHECK(tables_present IN (0,1)),
 scales_present INTEGER NOT NULL CHECK(scales_present IN (0,1)),
 guides_present INTEGER NOT NULL CHECK(guides_present IN (0,1)),
 annotations_present INTEGER NOT NULL CHECK(annotations_present IN (0,1)),
 presets_present INTEGER NOT NULL CHECK(presets_present IN (0,1))
);
CREATE TABLE chart_margin (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES chart_document(id),
 top REAL NOT NULL, top_kind TEXT NOT NULL, top_exact TEXT NOT NULL,
 right REAL NOT NULL, right_kind TEXT NOT NULL, right_exact TEXT NOT NULL,
 bottom REAL NOT NULL, bottom_kind TEXT NOT NULL, bottom_exact TEXT NOT NULL,
 left REAL NOT NULL, left_kind TEXT NOT NULL, left_exact TEXT NOT NULL
);
CREATE TABLE chart_theme (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES chart_document(id),
 name TEXT, appearance TEXT, palette_present INTEGER NOT NULL CHECK(palette_present IN (0,1))
);
CREATE TABLE chart_palette (
 id INTEGER PRIMARY KEY CHECK(id > 0), theme_id INTEGER NOT NULL REFERENCES chart_theme(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), value TEXT NOT NULL
);
CREATE TABLE chart_table (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES chart_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), name TEXT NOT NULL
);
CREATE TABLE chart_column (
 id INTEGER PRIMARY KEY CHECK(id > 0), table_id INTEGER NOT NULL REFERENCES chart_table(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), name TEXT NOT NULL
);
CREATE TABLE chart_row (
 id INTEGER PRIMARY KEY CHECK(id > 0), table_id INTEGER NOT NULL REFERENCES chart_table(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0)
);
CREATE TABLE chart_cell (
 id INTEGER PRIMARY KEY CHECK(id > 0), row_id INTEGER NOT NULL REFERENCES chart_row(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_scale (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES chart_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), name TEXT NOT NULL, kind TEXT NOT NULL, options_present INTEGER NOT NULL CHECK(options_present IN (0,1))
);
CREATE TABLE chart_domain (
 id INTEGER PRIMARY KEY CHECK(id > 0), scale_id INTEGER NOT NULL REFERENCES chart_scale(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_range (
 id INTEGER PRIMARY KEY CHECK(id > 0), scale_id INTEGER NOT NULL REFERENCES chart_scale(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_scale_option (
 id INTEGER PRIMARY KEY CHECK(id > 0), scale_id INTEGER NOT NULL REFERENCES chart_scale(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_scale_tick (
 id INTEGER PRIMARY KEY CHECK(id > 0), option_id INTEGER NOT NULL REFERENCES chart_scale_option(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_coordinate (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES chart_document(id), kind TEXT NOT NULL, options_present INTEGER NOT NULL CHECK(options_present IN (0,1))
);
CREATE TABLE chart_coordinate_option (
 id INTEGER PRIMARY KEY CHECK(id > 0), coordinate_id INTEGER NOT NULL REFERENCES chart_coordinate(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_coordinate_item (
 id INTEGER PRIMARY KEY CHECK(id > 0), option_id INTEGER NOT NULL REFERENCES chart_coordinate_option(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_layer (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES chart_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), mark TEXT NOT NULL, data TEXT,
 transform_present INTEGER NOT NULL CHECK(transform_present IN (0,1)), encodings_present INTEGER NOT NULL CHECK(encodings_present IN (0,1)), options_present INTEGER NOT NULL CHECK(options_present IN (0,1))
);
CREATE TABLE chart_transform (
 id INTEGER PRIMARY KEY CHECK(id > 0), layer_id INTEGER NOT NULL REFERENCES chart_layer(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), kind TEXT NOT NULL, options_present INTEGER NOT NULL CHECK(options_present IN (0,1))
);
CREATE TABLE chart_transform_option (
 id INTEGER PRIMARY KEY CHECK(id > 0), transform_id INTEGER NOT NULL REFERENCES chart_transform(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_transform_item (
 id INTEGER PRIMARY KEY CHECK(id > 0), option_id INTEGER NOT NULL REFERENCES chart_transform_option(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_layout (
 id INTEGER PRIMARY KEY CHECK(id > 0), layer_id INTEGER NOT NULL REFERENCES chart_layer(id), algorithm TEXT NOT NULL, options_present INTEGER NOT NULL CHECK(options_present IN (0,1))
);
CREATE TABLE chart_layout_option (
 id INTEGER PRIMARY KEY CHECK(id > 0), layout_id INTEGER NOT NULL REFERENCES chart_layout(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_encoding (
 id INTEGER PRIMARY KEY CHECK(id > 0), layer_id INTEGER NOT NULL REFERENCES chart_layer(id), channel TEXT NOT NULL, column_name TEXT, scale TEXT,
 value_kind TEXT, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_layer_option (
 id INTEGER PRIMARY KEY CHECK(id > 0), layer_id INTEGER NOT NULL REFERENCES chart_layer(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_guide (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES chart_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), kind TEXT NOT NULL,
 options_present INTEGER NOT NULL CHECK(options_present IN (0,1)), ticks_present INTEGER NOT NULL CHECK(ticks_present IN (0,1))
);
CREATE TABLE chart_guide_property (
 id INTEGER PRIMARY KEY CHECK(id > 0), guide_id INTEGER NOT NULL REFERENCES chart_guide(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_guide_option (
 id INTEGER PRIMARY KEY CHECK(id > 0), guide_id INTEGER NOT NULL REFERENCES chart_guide(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_tick (
 id INTEGER PRIMARY KEY CHECK(id > 0), guide_id INTEGER NOT NULL REFERENCES chart_guide(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_guide_title (
 id INTEGER PRIMARY KEY CHECK(id > 0), guide_id INTEGER NOT NULL REFERENCES chart_guide(id), en TEXT NOT NULL, de TEXT NOT NULL
);
CREATE TABLE chart_title (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES chart_document(id), en TEXT NOT NULL, de TEXT NOT NULL
);
CREATE TABLE chart_annotation (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES chart_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), kind TEXT NOT NULL,
 x REAL NOT NULL, x_kind TEXT NOT NULL, x_exact TEXT NOT NULL, y REAL NOT NULL, y_kind TEXT NOT NULL, y_exact TEXT NOT NULL,
 options_present INTEGER NOT NULL CHECK(options_present IN (0,1))
);
CREATE TABLE chart_annotation_property (
 id INTEGER PRIMARY KEY CHECK(id > 0), annotation_id INTEGER NOT NULL REFERENCES chart_annotation(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_annotation_option (
 id INTEGER PRIMARY KEY CHECK(id > 0), annotation_id INTEGER NOT NULL REFERENCES chart_annotation(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
CREATE TABLE chart_annotation_text (
 id INTEGER PRIMARY KEY CHECK(id > 0), annotation_id INTEGER NOT NULL REFERENCES chart_annotation(id), en TEXT NOT NULL, de TEXT NOT NULL
);
CREATE TABLE chart_preset (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES chart_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), kind TEXT NOT NULL, data TEXT, options_present INTEGER NOT NULL CHECK(options_present IN (0,1))
);
CREATE TABLE chart_preset_option (
 id INTEGER PRIMARY KEY CHECK(id > 0), preset_id INTEGER NOT NULL REFERENCES chart_preset(id), name TEXT NOT NULL,
 value_kind TEXT NOT NULL, string_value TEXT, number_value REAL, boolean_value INTEGER, number_kind TEXT, number_exact TEXT
);
