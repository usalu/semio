-- 🧩️ Complete persisted board and ordered kind catalog ownership.
CREATE TABLE puzzle2d_document (
 id INTEGER PRIMARY KEY CHECK (id = 1),
 schema TEXT NOT NULL
);
CREATE TABLE puzzle2d_camera (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES puzzle2d_document(id),
 x REAL, y REAL, zoom REAL,
 x_bits INTEGER NOT NULL, x_class TEXT NOT NULL CHECK (x_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 y_bits INTEGER NOT NULL, y_class TEXT NOT NULL CHECK (y_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 zoom_bits INTEGER NOT NULL, zoom_class TEXT NOT NULL CHECK (zoom_class IN ('finite','nan','positiveInfinity','negativeInfinity'))
);
CREATE TABLE puzzle2d_meta (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES puzzle2d_document(id),
 manifest_id TEXT
);
CREATE TABLE puzzle2d_node (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES puzzle2d_document(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 node_id TEXT NOT NULL, node_kind TEXT, shape TEXT,
 x REAL, y REAL, radius REAL, width REAL, height REAL,
 text TEXT, icon_kind TEXT,
 root INTEGER CHECK (root IN (0,1)),
 scale REAL,
 visible INTEGER CHECK (visible IN (0,1)), locked INTEGER CHECK (locked IN (0,1)),
 anchor TEXT NOT NULL CHECK (anchor IN ('fixed','derived')),
 x_bits INTEGER NOT NULL, x_class TEXT NOT NULL CHECK (x_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 y_bits INTEGER NOT NULL, y_class TEXT NOT NULL CHECK (y_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 radius_bits INTEGER, radius_class TEXT CHECK (radius_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 width_bits INTEGER, width_class TEXT CHECK (width_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 height_bits INTEGER, height_class TEXT CHECK (height_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 scale_bits INTEGER, scale_class TEXT CHECK (scale_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 CHECK ((radius IS NULL AND radius_bits IS NULL AND radius_class IS NULL) OR (radius_bits IS NOT NULL AND radius_class IS NOT NULL)),
 CHECK ((width IS NULL AND width_bits IS NULL AND width_class IS NULL) OR (width_bits IS NOT NULL AND width_class IS NOT NULL)),
 CHECK ((height IS NULL AND height_bits IS NULL AND height_class IS NULL) OR (height_bits IS NOT NULL AND height_class IS NOT NULL)),
 CHECK ((scale IS NULL AND scale_bits IS NULL AND scale_class IS NULL) OR (scale_bits IS NOT NULL AND scale_class IS NOT NULL))
);
CREATE TABLE puzzle2d_handle (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 node_id INTEGER NOT NULL REFERENCES puzzle2d_node(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 handle_id TEXT NOT NULL, handle_kind TEXT, angle REAL, radius REAL, color TEXT, icon_kind TEXT, scale REAL,
 visible INTEGER CHECK (visible IN (0,1)), locked INTEGER CHECK (locked IN (0,1)),
 angle_bits INTEGER NOT NULL, angle_class TEXT NOT NULL CHECK (angle_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 radius_bits INTEGER, radius_class TEXT CHECK (radius_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 scale_bits INTEGER, scale_class TEXT CHECK (scale_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 CHECK ((radius IS NULL AND radius_bits IS NULL AND radius_class IS NULL) OR (radius_bits IS NOT NULL AND radius_class IS NOT NULL)),
 CHECK ((scale IS NULL AND scale_bits IS NULL AND scale_class IS NULL) OR (scale_bits IS NOT NULL AND scale_class IS NOT NULL))
);
CREATE TABLE puzzle2d_edge (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES puzzle2d_document(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 edge_id TEXT NOT NULL, source TEXT NOT NULL, target TEXT NOT NULL, edge_kind TEXT,
 gap REAL, shift REAL, rise REAL, rotation REAL, turn REAL, tilt REAL, x REAL, y REAL,
 source_tip TEXT, target_tip TEXT,
 visible INTEGER CHECK (visible IN (0,1)), locked INTEGER CHECK (locked IN (0,1)),
 gap_bits INTEGER NOT NULL, gap_class TEXT NOT NULL CHECK (gap_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 shift_bits INTEGER NOT NULL, shift_class TEXT NOT NULL CHECK (shift_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 rise_bits INTEGER NOT NULL, rise_class TEXT NOT NULL CHECK (rise_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 rotation_bits INTEGER NOT NULL, rotation_class TEXT NOT NULL CHECK (rotation_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 turn_bits INTEGER NOT NULL, turn_class TEXT NOT NULL CHECK (turn_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 tilt_bits INTEGER NOT NULL, tilt_class TEXT NOT NULL CHECK (tilt_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 x_bits INTEGER NOT NULL, x_class TEXT NOT NULL CHECK (x_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 y_bits INTEGER NOT NULL, y_class TEXT NOT NULL CHECK (y_class IN ('finite','nan','positiveInfinity','negativeInfinity'))
);
CREATE TABLE puzzle2d_target_region (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 document_id INTEGER NOT NULL REFERENCES puzzle2d_document(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 region_id TEXT NOT NULL, x REAL, y REAL, width REAL, height REAL, label TEXT,
 hidden INTEGER NOT NULL CHECK (hidden IN (0,1)), locked INTEGER NOT NULL CHECK (locked IN (0,1)),
 x_bits INTEGER NOT NULL, x_class TEXT NOT NULL CHECK (x_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 y_bits INTEGER NOT NULL, y_class TEXT NOT NULL CHECK (y_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 width_bits INTEGER NOT NULL, width_class TEXT NOT NULL CHECK (width_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 height_bits INTEGER NOT NULL, height_class TEXT NOT NULL CHECK (height_class IN ('finite','nan','positiveInfinity','negativeInfinity'))
);
CREATE TABLE puzzle2d_kind_compatibility (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 meta_id INTEGER NOT NULL REFERENCES puzzle2d_meta(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 source TEXT NOT NULL, target TEXT NOT NULL,
 bidirectional INTEGER NOT NULL CHECK (bidirectional IN (0,1)), important INTEGER NOT NULL CHECK (important IN (0,1)),
 specificity TEXT NOT NULL CHECK (specificity IN ('general','node','edge','handle','wire','vortex'))
);
CREATE TABLE puzzle2d_kind_catalogs (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 meta_id INTEGER NOT NULL REFERENCES puzzle2d_meta(id)
);
CREATE TABLE puzzle2d_catalog_node_kind (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 catalogs_id INTEGER NOT NULL REFERENCES puzzle2d_kind_catalogs(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 kind_id TEXT NOT NULL, name TEXT NOT NULL, label TEXT NOT NULL, description TEXT NOT NULL,
 icon TEXT NOT NULL, image TEXT NOT NULL, unit TEXT NOT NULL,
 abstract INTEGER NOT NULL CHECK (abstract IN (0,1))
);
CREATE TABLE puzzle2d_base_kind (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 node_kind_id INTEGER NOT NULL REFERENCES puzzle2d_catalog_node_kind(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 kind_id TEXT NOT NULL
);
CREATE TABLE puzzle2d_representation (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 node_kind_id INTEGER NOT NULL REFERENCES puzzle2d_catalog_node_kind(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 representation_id TEXT NOT NULL, name TEXT NOT NULL, url TEXT NOT NULL, mime TEXT NOT NULL, lod TEXT, description TEXT NOT NULL
);
CREATE TABLE puzzle2d_representation_tag (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 representation_id INTEGER NOT NULL REFERENCES puzzle2d_representation(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 tag TEXT NOT NULL
);
CREATE TABLE puzzle2d_handle_template (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 node_kind_id INTEGER NOT NULL REFERENCES puzzle2d_catalog_node_kind(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 template_id TEXT NOT NULL, name TEXT NOT NULL, label TEXT NOT NULL, description TEXT NOT NULL, icon TEXT NOT NULL, handle_kind TEXT,
 angle REAL, t REAL, mandatory INTEGER CHECK (mandatory IN (0,1)), radius REAL,
 angle_bits INTEGER NOT NULL, angle_class TEXT NOT NULL CHECK (angle_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 t_bits INTEGER, t_class TEXT CHECK (t_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 radius_bits INTEGER, radius_class TEXT CHECK (radius_class IN ('finite','nan','positiveInfinity','negativeInfinity')),
 CHECK ((t IS NULL AND t_bits IS NULL AND t_class IS NULL) OR (t_bits IS NOT NULL AND t_class IS NOT NULL)),
 CHECK ((radius IS NULL AND radius_bits IS NULL AND radius_class IS NULL) OR (radius_bits IS NOT NULL AND radius_class IS NOT NULL))
);
CREATE TABLE puzzle2d_attribute (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 node_kind_id INTEGER NOT NULL REFERENCES puzzle2d_catalog_node_kind(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 attribute_id TEXT NOT NULL, key TEXT NOT NULL, value TEXT NOT NULL, definition TEXT
);
CREATE TABLE puzzle2d_author (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 node_kind_id INTEGER NOT NULL REFERENCES puzzle2d_catalog_node_kind(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 author_id TEXT NOT NULL, name TEXT NOT NULL, email TEXT NOT NULL, role TEXT,
 rank INTEGER CHECK (rank BETWEEN -2147483648 AND 2147483647)
);
CREATE TABLE puzzle2d_catalog_handle_kind (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 catalogs_id INTEGER NOT NULL REFERENCES puzzle2d_kind_catalogs(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 kind_id TEXT NOT NULL, code TEXT, label TEXT,
 order_value INTEGER CHECK (order_value BETWEEN -2147483648 AND 2147483647),
 description TEXT NOT NULL, icon TEXT NOT NULL, color TEXT NOT NULL, default_wire_kind TEXT NOT NULL
);
CREATE TABLE puzzle2d_compatible_kind (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 handle_kind_id INTEGER NOT NULL REFERENCES puzzle2d_catalog_handle_kind(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 kind_id TEXT NOT NULL
);
CREATE TABLE puzzle2d_catalog_edge_kind (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 catalogs_id INTEGER NOT NULL REFERENCES puzzle2d_kind_catalogs(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 kind_id TEXT NOT NULL, name TEXT NOT NULL, label TEXT NOT NULL, description TEXT NOT NULL, icon TEXT NOT NULL, color TEXT NOT NULL
);
CREATE TABLE puzzle2d_catalog_wire_kind (
 id INTEGER PRIMARY KEY CHECK (id > 0),
 catalogs_id INTEGER NOT NULL REFERENCES puzzle2d_kind_catalogs(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 kind_id TEXT NOT NULL, name TEXT NOT NULL, label TEXT NOT NULL, description TEXT NOT NULL, icon TEXT NOT NULL, color TEXT NOT NULL, default_edge_kind TEXT NOT NULL
);
