CREATE TABLE dwg_material (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), name TEXT NOT NULL, description TEXT NOT NULL,
 specular_gloss_class TEXT NOT NULL CHECK(specular_gloss_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), specular_gloss_ieee754_bits INTEGER, specular_gloss REAL CHECK((specular_gloss_class='finite')=(specular_gloss IS NOT NULL)),
 opacity_class TEXT NOT NULL CHECK(opacity_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), opacity_ieee754_bits INTEGER, opacity REAL CHECK((opacity_class='finite')=(opacity IS NOT NULL)),
 refraction_index_class TEXT NOT NULL CHECK(refraction_index_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), refraction_index_ieee754_bits INTEGER, refraction_index REAL CHECK((refraction_index_class='finite')=(refraction_index IS NOT NULL)),
 translucence_class TEXT NOT NULL CHECK(translucence_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), translucence_ieee754_bits INTEGER, translucence REAL CHECK((translucence_class='finite')=(translucence IS NOT NULL)),
 self_illumination_class TEXT NOT NULL CHECK(self_illumination_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), self_illumination_ieee754_bits INTEGER, self_illumination REAL CHECK((self_illumination_class='finite')=(self_illumination IS NOT NULL)),
 reflectivity_class TEXT NOT NULL CHECK(reflectivity_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), reflectivity_ieee754_bits INTEGER, reflectivity REAL CHECK((reflectivity_class='finite')=(reflectivity IS NOT NULL))
);
CREATE TABLE dwg_material_channels (
 id INTEGER PRIMARY KEY REFERENCES dwg_material(id), diffuse INTEGER NOT NULL CHECK(diffuse IN (0,1)), specular INTEGER NOT NULL CHECK(specular IN (0,1)), reflection INTEGER NOT NULL CHECK(reflection IN (0,1)), opacity INTEGER NOT NULL CHECK(opacity IN (0,1)), bump INTEGER NOT NULL CHECK(bump IN (0,1)), refraction INTEGER NOT NULL CHECK(refraction IN (0,1))
);
CREATE TABLE dwg_material_color (
 id INTEGER PRIMARY KEY, material_id INTEGER NOT NULL REFERENCES dwg_material(id), ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 2), channel TEXT NOT NULL CHECK(channel IN ('ambient','diffuse','specular')),
 factor_class TEXT NOT NULL CHECK(factor_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), factor_ieee754_bits INTEGER, factor REAL CHECK((factor_class='finite')=(factor IS NOT NULL)), override_rgb INTEGER CHECK(override_rgb BETWEEN 0 AND 4294967295),
 CHECK((ordinal=0 AND channel='ambient') OR (ordinal=1 AND channel='diffuse') OR (ordinal=2 AND channel='specular'))
);
CREATE TABLE dwg_material_map (
 id INTEGER PRIMARY KEY, material_id INTEGER NOT NULL REFERENCES dwg_material(id), ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 5), channel TEXT NOT NULL CHECK(channel IN ('diffuse','specular','reflection','opacity','bump','refraction')),
 blend_factor_class TEXT NOT NULL CHECK(blend_factor_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), blend_factor_ieee754_bits INTEGER, blend_factor REAL CHECK((blend_factor_class='finite')=(blend_factor IS NOT NULL)),
 projection TEXT NOT NULL CHECK(projection IN ('inherit','planar','box','cylinder','sphere')), tiling TEXT NOT NULL CHECK(tiling IN ('inherit','tile','crop','clamp','mirror')),
 scale_to_entity INTEGER NOT NULL CHECK(scale_to_entity IN (0,1)), use_current_block_transform INTEGER NOT NULL CHECK(use_current_block_transform IN (0,1)), source TEXT NOT NULL CHECK(source IN ('none','current_scene')),
 CHECK((ordinal=0 AND channel='diffuse') OR (ordinal=1 AND channel='specular') OR (ordinal=2 AND channel='reflection') OR (ordinal=3 AND channel='opacity') OR (ordinal=4 AND channel='bump') OR (ordinal=5 AND channel='refraction'))
);
CREATE TABLE dwg_material_map_transform_coordinate (
 id INTEGER PRIMARY KEY, material_map_id INTEGER NOT NULL REFERENCES dwg_material_map(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL CHECK((coordinate_class='finite')=(coordinate IS NOT NULL))
);
CREATE TABLE dwg_mline_style (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), name TEXT NOT NULL, description TEXT NOT NULL, fill_enabled INTEGER NOT NULL CHECK(fill_enabled IN (0,1)), display_miters INTEGER NOT NULL CHECK(display_miters IN (0,1)),
 start_square INTEGER NOT NULL CHECK(start_square IN (0,1)), start_inner_arcs INTEGER NOT NULL CHECK(start_inner_arcs IN (0,1)), start_round_outer_arcs INTEGER NOT NULL CHECK(start_round_outer_arcs IN (0,1)),
 end_square INTEGER NOT NULL CHECK(end_square IN (0,1)), end_inner_arcs INTEGER NOT NULL CHECK(end_inner_arcs IN (0,1)), end_round_outer_arcs INTEGER NOT NULL CHECK(end_round_outer_arcs IN (0,1)),
 start_angle_class TEXT NOT NULL CHECK(start_angle_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), start_angle_ieee754_bits INTEGER, start_angle REAL CHECK((start_angle_class='finite')=(start_angle IS NOT NULL)),
 end_angle_class TEXT NOT NULL CHECK(end_angle_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), end_angle_ieee754_bits INTEGER, end_angle REAL CHECK((end_angle_class='finite')=(end_angle IS NOT NULL))
);
CREATE TABLE dwg_mline_fill_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_mline_style(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
CREATE TABLE dwg_mline_style_element (
 id INTEGER PRIMARY KEY, mline_style_id INTEGER NOT NULL REFERENCES dwg_mline_style(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), offset_class TEXT NOT NULL CHECK(offset_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), offset_ieee754_bits INTEGER, offset REAL CHECK((offset_class='finite')=(offset IS NOT NULL)), linetype TEXT NOT NULL CHECK(linetype IN ('by_layer','by_block','continuous'))
);
CREATE TABLE dwg_mline_element_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_mline_style_element(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
