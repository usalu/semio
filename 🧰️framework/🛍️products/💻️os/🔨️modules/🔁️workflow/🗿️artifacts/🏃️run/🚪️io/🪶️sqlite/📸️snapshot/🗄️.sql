CREATE TABLE run_document (
 id INTEGER PRIMARY KEY CHECK (id = 1),
 schema TEXT NOT NULL,
 workflow_ref TEXT NOT NULL,
 workflow_checkpoint_id TEXT NOT NULL,
 input_collection_ref TEXT NOT NULL,
 input_snapshot_id TEXT NOT NULL,
 output_collection_ref TEXT NOT NULL,
 status INTEGER NOT NULL CHECK (status >= 0 AND status <= 4),
 started_at TEXT NOT NULL,
 finished_at TEXT,
 sealed INTEGER NOT NULL CHECK (sealed IN (0,1))
);
CREATE TABLE run_trigger (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES run_document(id),
 kind TEXT NOT NULL CHECK (kind IN ('manual','automation')),
 actor TEXT,
 automation_ref TEXT,
 event_fingerprint TEXT,
 CHECK ((kind = 'manual' AND actor IS NOT NULL AND automation_ref IS NULL AND event_fingerprint IS NULL) OR (kind = 'automation' AND actor IS NULL AND automation_ref IS NOT NULL AND event_fingerprint IS NOT NULL))
);
CREATE TABLE run_parameter_value (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES run_document(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 parameter_id TEXT NOT NULL,
 value TEXT NOT NULL
);
CREATE TABLE run_node (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES run_document(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 node_id TEXT NOT NULL,
 status INTEGER NOT NULL CHECK (status >= 0 AND status <= 2),
 document_fingerprint TEXT NOT NULL,
 config_fingerprint TEXT NOT NULL,
 duration_ms REAL,
 duration_ms_bits INTEGER NOT NULL,
 duration_ms_class TEXT NOT NULL CHECK (duration_ms_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 CHECK ((duration_ms_class = 'nan' AND duration_ms IS NULL) OR (duration_ms_class <> 'nan' AND duration_ms IS NOT NULL))
);
CREATE TABLE run_port_fingerprint (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 node_id INTEGER NOT NULL REFERENCES run_node(id),
 direction TEXT NOT NULL CHECK (direction IN ('input','output')),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 port_id TEXT NOT NULL,
 fingerprint TEXT NOT NULL
);
CREATE TABLE run_output_artifact (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 node_id INTEGER NOT NULL REFERENCES run_node(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 port_id TEXT NOT NULL,
 artifact_id TEXT NOT NULL,
 path TEXT NOT NULL
);
CREATE TABLE run_log (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES run_document(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 node_id TEXT NOT NULL,
 level TEXT NOT NULL,
 message TEXT NOT NULL,
 at TEXT NOT NULL
);
