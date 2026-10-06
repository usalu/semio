CREATE TABLE gltf_material (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES gltf_document(id),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0),
 name TEXT,
 emissive_red REAL,
 emissive_green REAL,
 emissive_blue REAL,
 alpha_mode TEXT NOT NULL CHECK(alpha_mode IN ('OPAQUE','MASK','BLEND')),
 alpha_cutoff REAL,
 double_sided INTEGER NOT NULL CHECK(double_sided IN (0,1)),
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 emissive_red_ieee754_bits INTEGER NOT NULL,
 emissive_red_numeric_class TEXT NOT NULL CHECK(emissive_red_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 emissive_green_ieee754_bits INTEGER NOT NULL,
 emissive_green_numeric_class TEXT NOT NULL CHECK(emissive_green_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 emissive_blue_ieee754_bits INTEGER NOT NULL,
 emissive_blue_numeric_class TEXT NOT NULL CHECK(emissive_blue_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 alpha_cutoff_ieee754_bits INTEGER NOT NULL,
 alpha_cutoff_numeric_class TEXT NOT NULL CHECK(alpha_cutoff_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_pbr_metallic_roughness (
 id INTEGER PRIMARY KEY REFERENCES gltf_material(id),
 base_color_red REAL,
 base_color_green REAL,
 base_color_blue REAL,
 base_color_alpha REAL,
 metallic_factor REAL,
 roughness_factor REAL,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 base_color_red_ieee754_bits INTEGER NOT NULL,
 base_color_red_numeric_class TEXT NOT NULL CHECK(base_color_red_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 base_color_green_ieee754_bits INTEGER NOT NULL,
 base_color_green_numeric_class TEXT NOT NULL CHECK(base_color_green_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 base_color_blue_ieee754_bits INTEGER NOT NULL,
 base_color_blue_numeric_class TEXT NOT NULL CHECK(base_color_blue_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 base_color_alpha_ieee754_bits INTEGER NOT NULL,
 base_color_alpha_numeric_class TEXT NOT NULL CHECK(base_color_alpha_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 metallic_factor_ieee754_bits INTEGER NOT NULL,
 metallic_factor_numeric_class TEXT NOT NULL CHECK(metallic_factor_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
 roughness_factor_ieee754_bits INTEGER NOT NULL,
 roughness_factor_numeric_class TEXT NOT NULL CHECK(roughness_factor_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_base_color_texture (
 id INTEGER PRIMARY KEY REFERENCES gltf_pbr_metallic_roughness(id),
 texture_index_high INTEGER NOT NULL CHECK(texture_index_high BETWEEN 0 AND 4294967295),
 texture_index_low INTEGER NOT NULL CHECK(texture_index_low BETWEEN 0 AND 4294967295),
 tex_coord_high INTEGER NOT NULL CHECK(tex_coord_high BETWEEN 0 AND 4294967295),
 tex_coord_low INTEGER NOT NULL CHECK(tex_coord_low BETWEEN 0 AND 4294967295),
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_metallic_roughness_texture (
 id INTEGER PRIMARY KEY REFERENCES gltf_pbr_metallic_roughness(id),
 texture_index_high INTEGER NOT NULL CHECK(texture_index_high BETWEEN 0 AND 4294967295),
 texture_index_low INTEGER NOT NULL CHECK(texture_index_low BETWEEN 0 AND 4294967295),
 tex_coord_high INTEGER NOT NULL CHECK(tex_coord_high BETWEEN 0 AND 4294967295),
 tex_coord_low INTEGER NOT NULL CHECK(tex_coord_low BETWEEN 0 AND 4294967295),
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_emissive_texture (
 id INTEGER PRIMARY KEY REFERENCES gltf_material(id),
 texture_index_high INTEGER NOT NULL CHECK(texture_index_high BETWEEN 0 AND 4294967295),
 texture_index_low INTEGER NOT NULL CHECK(texture_index_low BETWEEN 0 AND 4294967295),
 tex_coord_high INTEGER NOT NULL CHECK(tex_coord_high BETWEEN 0 AND 4294967295),
 tex_coord_low INTEGER NOT NULL CHECK(tex_coord_low BETWEEN 0 AND 4294967295),
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id)
);
CREATE TABLE gltf_normal_texture (
 id INTEGER PRIMARY KEY REFERENCES gltf_material(id),
 texture_index_high INTEGER NOT NULL CHECK(texture_index_high BETWEEN 0 AND 4294967295),
 texture_index_low INTEGER NOT NULL CHECK(texture_index_low BETWEEN 0 AND 4294967295),
 tex_coord_high INTEGER NOT NULL CHECK(tex_coord_high BETWEEN 0 AND 4294967295),
 tex_coord_low INTEGER NOT NULL CHECK(tex_coord_low BETWEEN 0 AND 4294967295),
 scale REAL,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 scale_ieee754_bits INTEGER NOT NULL,
 scale_numeric_class TEXT NOT NULL CHECK(scale_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
CREATE TABLE gltf_occlusion_texture (
 id INTEGER PRIMARY KEY REFERENCES gltf_material(id),
 texture_index_high INTEGER NOT NULL CHECK(texture_index_high BETWEEN 0 AND 4294967295),
 texture_index_low INTEGER NOT NULL CHECK(texture_index_low BETWEEN 0 AND 4294967295),
 tex_coord_high INTEGER NOT NULL CHECK(tex_coord_high BETWEEN 0 AND 4294967295),
 tex_coord_low INTEGER NOT NULL CHECK(tex_coord_low BETWEEN 0 AND 4294967295),
 strength REAL,
 extensions_id INTEGER REFERENCES gltf_json_value(id),
 extras_id INTEGER REFERENCES gltf_json_value(id),
 strength_ieee754_bits INTEGER NOT NULL,
 strength_numeric_class TEXT NOT NULL CHECK(strength_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan'))
);
