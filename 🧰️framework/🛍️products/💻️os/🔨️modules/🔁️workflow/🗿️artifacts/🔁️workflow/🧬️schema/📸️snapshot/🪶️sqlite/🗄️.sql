CREATE TABLE workflow_document (id INTEGER PRIMARY KEY CHECK(id = 1), schema TEXT NOT NULL);
CREATE TABLE workflow_graph (id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES workflow_document(id), schema TEXT NOT NULL);
CREATE TABLE workflow_node (
 id INTEGER PRIMARY KEY CHECK(id > 0), graph_id INTEGER NOT NULL REFERENCES workflow_graph(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 node_id TEXT NOT NULL, plugin_id TEXT NOT NULL, app_id TEXT NOT NULL, label TEXT NOT NULL, yields TEXT NOT NULL, artifact_ref TEXT NOT NULL, config_ref TEXT NOT NULL,
 x REAL, y REAL, width REAL, height REAL,
 x_bits INTEGER NOT NULL, x_class TEXT NOT NULL CHECK(x_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 y_bits INTEGER NOT NULL, y_class TEXT NOT NULL CHECK(y_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 width_bits INTEGER NOT NULL, width_class TEXT NOT NULL CHECK(width_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 height_bits INTEGER NOT NULL, height_class TEXT NOT NULL CHECK(height_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 CHECK((x_class = 'nan' AND x IS NULL) OR (x_class <> 'nan' AND x IS NOT NULL)),
 CHECK((y_class = 'nan' AND y IS NULL) OR (y_class <> 'nan' AND y IS NOT NULL)),
 CHECK((width_class = 'nan' AND width IS NULL) OR (width_class <> 'nan' AND width IS NOT NULL)),
 CHECK((height_class = 'nan' AND height IS NULL) OR (height_class <> 'nan' AND height IS NOT NULL))
);
CREATE TABLE workflow_port (
 id INTEGER PRIMARY KEY CHECK(id > 0), node_id INTEGER NOT NULL REFERENCES workflow_node(id), collection TEXT NOT NULL CHECK(collection IN ('input','output')), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 endpoint_id TEXT NOT NULL, port_id TEXT NOT NULL, label TEXT NOT NULL, direction INTEGER NOT NULL CHECK(direction IN (0,1)), media_class INTEGER NOT NULL CHECK(media_class >= 0 AND media_class <= 7), media_form INTEGER NOT NULL CHECK(media_form >= 0 AND media_form <= 15),
 kind_id TEXT, required INTEGER NOT NULL CHECK(required IN (0,1)), multiplicity INTEGER NOT NULL CHECK(multiplicity IN (0,1))
);
CREATE TABLE workflow_edge (
 id INTEGER PRIMARY KEY CHECK(id > 0), graph_id INTEGER NOT NULL REFERENCES workflow_graph(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 edge_id TEXT NOT NULL, source_node_id TEXT NOT NULL, source_port_id TEXT NOT NULL, target_node_id TEXT NOT NULL, target_port_id TEXT NOT NULL
);
CREATE TABLE workflow_contract (
 id INTEGER PRIMARY KEY CHECK(id > 0), edge_id INTEGER NOT NULL REFERENCES workflow_edge(id), kind_id TEXT NOT NULL,
 media_class INTEGER NOT NULL CHECK(media_class >= 0 AND media_class <= 7), media_form INTEGER NOT NULL CHECK(media_form >= 0 AND media_form <= 15),
 wire_kind TEXT NOT NULL CHECK(wire_kind IN ('binary','document')), format_kind TEXT, document_schema TEXT,
 conversion_from INTEGER CHECK(conversion_from >= 0 AND conversion_from <= 15), conversion_to INTEGER CHECK(conversion_to >= 0 AND conversion_to <= 15),
 CHECK((wire_kind = 'binary' AND format_kind IS NOT NULL AND document_schema IS NULL) OR (wire_kind = 'document' AND format_kind IS NULL AND document_schema IS NOT NULL)),
 CHECK((conversion_from IS NULL AND conversion_to IS NULL) OR (conversion_from IS NOT NULL AND conversion_to IS NOT NULL))
);
CREATE TABLE workflow_parameter (
 id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES workflow_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0),
 parameter_id TEXT NOT NULL, name TEXT NOT NULL, kind TEXT NOT NULL CHECK(kind IN ('numeric','categorical','toggle','text'))
);
CREATE TABLE workflow_numeric (
 id INTEGER PRIMARY KEY CHECK(id > 0), parameter_id INTEGER NOT NULL REFERENCES workflow_parameter(id), value REAL, min REAL, max REAL, step REAL,
 value_bits INTEGER NOT NULL, value_class TEXT NOT NULL CHECK(value_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 min_bits INTEGER, min_class TEXT CHECK(min_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 max_bits INTEGER, max_class TEXT CHECK(max_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 step_bits INTEGER, step_class TEXT CHECK(step_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 CHECK((value_class = 'nan' AND value IS NULL) OR (value_class <> 'nan' AND value IS NOT NULL)),
 CHECK((min IS NULL AND min_bits IS NULL AND min_class IS NULL) OR (min_bits IS NOT NULL AND min_class IS NOT NULL AND ((min_class = 'nan' AND min IS NULL) OR (min_class <> 'nan' AND min IS NOT NULL)))),
 CHECK((max IS NULL AND max_bits IS NULL AND max_class IS NULL) OR (max_bits IS NOT NULL AND max_class IS NOT NULL AND ((max_class = 'nan' AND max IS NULL) OR (max_class <> 'nan' AND max IS NOT NULL)))),
 CHECK((step IS NULL AND step_bits IS NULL AND step_class IS NULL) OR (step_bits IS NOT NULL AND step_class IS NOT NULL AND ((step_class = 'nan' AND step IS NULL) OR (step_class <> 'nan' AND step IS NOT NULL))))
);
CREATE TABLE workflow_categorical (id INTEGER PRIMARY KEY CHECK(id > 0), parameter_id INTEGER NOT NULL REFERENCES workflow_parameter(id), value TEXT NOT NULL);
CREATE TABLE workflow_category_option (id INTEGER PRIMARY KEY CHECK(id > 0), categorical_id INTEGER NOT NULL REFERENCES workflow_categorical(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), value TEXT NOT NULL);
CREATE TABLE workflow_toggle (id INTEGER PRIMARY KEY CHECK(id > 0), parameter_id INTEGER NOT NULL REFERENCES workflow_parameter(id), value INTEGER NOT NULL CHECK(value IN (0,1)));
CREATE TABLE workflow_text (id INTEGER PRIMARY KEY CHECK(id > 0), parameter_id INTEGER NOT NULL REFERENCES workflow_parameter(id), value TEXT NOT NULL);
CREATE TABLE workflow_parameter_binding (id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES workflow_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), parameter_id TEXT NOT NULL, node_id TEXT NOT NULL, field_path TEXT NOT NULL);
CREATE TABLE workflow_input (id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES workflow_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), input_id TEXT NOT NULL, kind_id TEXT NOT NULL, selector TEXT NOT NULL, required INTEGER NOT NULL CHECK(required IN (0,1)), multiplicity INTEGER NOT NULL CHECK(multiplicity IN (0,1)));
CREATE TABLE workflow_input_binding (id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES workflow_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), input_id TEXT NOT NULL, node_id TEXT NOT NULL, port_id TEXT NOT NULL);
CREATE TABLE workflow_output_binding (id INTEGER PRIMARY KEY CHECK(id > 0), document_id INTEGER NOT NULL REFERENCES workflow_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), node_id TEXT NOT NULL, port_id TEXT NOT NULL, path_template TEXT NOT NULL);
