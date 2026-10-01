CREATE TABLE gltf_camera (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 projection TEXT NOT NULL CHECK(projection IN ('perspective','orthographic')),
 name TEXT,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_camera_orthographic (
 id INTEGER PRIMARY KEY REFERENCES gltf_camera(id),
 xmag REAL,
 ymag REAL,
 zfar REAL,
 znear REAL,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 xmag_ieee754_bits INTEGER NOT NULL,
 xmag_numeric_class TEXT NOT NULL CHECK(xmag_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 ymag_ieee754_bits INTEGER NOT NULL,
 ymag_numeric_class TEXT NOT NULL CHECK(ymag_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 zfar_ieee754_bits INTEGER NOT NULL,
 zfar_numeric_class TEXT NOT NULL CHECK(zfar_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 znear_ieee754_bits INTEGER NOT NULL,
 znear_numeric_class TEXT NOT NULL CHECK(znear_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_camera_perspective (
 id INTEGER PRIMARY KEY REFERENCES gltf_camera(id),
 aspect_ratio REAL,
 yfov REAL,
 zfar REAL,
 znear REAL,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 aspect_ratio_ieee754_bits INTEGER,
 aspect_ratio_numeric_class TEXT CHECK(aspect_ratio_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 yfov_ieee754_bits INTEGER NOT NULL,
 yfov_numeric_class TEXT NOT NULL CHECK(yfov_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 zfar_ieee754_bits INTEGER,
 zfar_numeric_class TEXT CHECK(zfar_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 znear_ieee754_bits INTEGER NOT NULL,
 znear_numeric_class TEXT NOT NULL CHECK(znear_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 CHECK((aspect_ratio_ieee754_bits IS NULL AND aspect_ratio_numeric_class IS NULL AND aspect_ratio IS NULL) OR (aspect_ratio_ieee754_bits IS NOT NULL AND aspect_ratio_numeric_class IS NOT NULL)),
 CHECK((zfar_ieee754_bits IS NULL AND zfar_numeric_class IS NULL AND zfar IS NULL) OR (zfar_ieee754_bits IS NOT NULL AND zfar_numeric_class IS NOT NULL))
);
