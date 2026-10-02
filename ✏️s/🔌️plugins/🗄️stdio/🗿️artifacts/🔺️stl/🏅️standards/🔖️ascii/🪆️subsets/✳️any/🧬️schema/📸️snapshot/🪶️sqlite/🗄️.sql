CREATE TABLE stl_solid (
  id INTEGER PRIMARY KEY,
  schema TEXT NOT NULL,
  name TEXT NOT NULL
);
CREATE TABLE stl_facet (
  id INTEGER PRIMARY KEY,
  solid_id INTEGER NOT NULL REFERENCES stl_solid(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  normal_x REAL,
  normal_y REAL,
  normal_z REAL,
  normal_x_ieee754_bits INTEGER NOT NULL,
  normal_x_numeric_class TEXT NOT NULL CHECK(normal_x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  normal_y_ieee754_bits INTEGER NOT NULL,
  normal_y_numeric_class TEXT NOT NULL CHECK(normal_y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  normal_z_ieee754_bits INTEGER NOT NULL,
  normal_z_numeric_class TEXT NOT NULL CHECK(normal_z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE stl_vertex (
  id INTEGER PRIMARY KEY,
  facet_id INTEGER NOT NULL REFERENCES stl_facet(id),
  ordinal INTEGER NOT NULL CHECK (ordinal BETWEEN 0 AND 2),
  x REAL,
  y REAL,
  z REAL,
  x_ieee754_bits INTEGER NOT NULL,
  x_numeric_class TEXT NOT NULL CHECK(x_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  y_ieee754_bits INTEGER NOT NULL,
  y_numeric_class TEXT NOT NULL CHECK(y_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  z_ieee754_bits INTEGER NOT NULL,
  z_numeric_class TEXT NOT NULL CHECK(z_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
