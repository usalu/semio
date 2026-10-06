CREATE TABLE dwg_visual_style (id INTEGER PRIMARY KEY REFERENCES dwg_object(id), description TEXT NOT NULL, style_type INTEGER NOT NULL CHECK(style_type BETWEEN 0 AND 4294967295), extension_lighting_model INTEGER NOT NULL CHECK(extension_lighting_model BETWEEN 0 AND 65535), internal_only INTEGER NOT NULL CHECK(internal_only IN (0,1)));
CREATE TABLE dwg_visual_style_face (
 id INTEGER PRIMARY KEY REFERENCES dwg_visual_style(id),
 lighting_model INTEGER NOT NULL CHECK(lighting_model BETWEEN 0 AND 4294967295), lighting_model_operation TEXT NOT NULL CHECK(lighting_model_operation IN ('inherit','set','disable','enable')),
 lighting_quality INTEGER NOT NULL CHECK(lighting_quality BETWEEN 0 AND 4294967295), lighting_quality_operation TEXT NOT NULL CHECK(lighting_quality_operation IN ('inherit','set','disable','enable')),
 color_mode INTEGER NOT NULL CHECK(color_mode BETWEEN 0 AND 4294967295), color_mode_operation TEXT NOT NULL CHECK(color_mode_operation IN ('inherit','set','disable','enable')),
 modifiers INTEGER NOT NULL CHECK(modifiers BETWEEN 0 AND 65535), modifiers_operation TEXT NOT NULL CHECK(modifiers_operation IN ('inherit','set','disable','enable')),
 opacity_class TEXT NOT NULL CHECK(opacity_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), opacity_ieee754_bits INTEGER, opacity REAL CHECK((opacity_class='finite')=(opacity IS NOT NULL)), opacity_operation TEXT NOT NULL CHECK(opacity_operation IN ('inherit','set','disable','enable')),
 specular_amount_class TEXT NOT NULL CHECK(specular_amount_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), specular_amount_ieee754_bits INTEGER, specular_amount REAL CHECK((specular_amount_class='finite')=(specular_amount IS NOT NULL)), specular_amount_operation TEXT NOT NULL CHECK(specular_amount_operation IN ('inherit','set','disable','enable')),
 monochrome_color_operation TEXT NOT NULL CHECK(monochrome_color_operation IN ('inherit','set','disable','enable'))
);
CREATE TABLE dwg_visual_style_edge (
 id INTEGER PRIMARY KEY REFERENCES dwg_visual_style(id),
 model INTEGER NOT NULL CHECK(model BETWEEN 0 AND 4294967295), model_operation TEXT NOT NULL CHECK(model_operation IN ('inherit','set','disable','enable')),
 styles INTEGER NOT NULL CHECK(styles BETWEEN 0 AND 4294967295), styles_operation TEXT NOT NULL CHECK(styles_operation IN ('inherit','set','disable','enable')),
 intersection_color_operation TEXT NOT NULL CHECK(intersection_color_operation IN ('inherit','set','disable','enable')), obscured_color_operation TEXT NOT NULL CHECK(obscured_color_operation IN ('inherit','set','disable','enable')),
 obscured_line_pattern INTEGER NOT NULL CHECK(obscured_line_pattern BETWEEN 0 AND 4294967295), obscured_line_pattern_operation TEXT NOT NULL CHECK(obscured_line_pattern_operation IN ('inherit','set','disable','enable')),
 intersection_line_pattern INTEGER NOT NULL CHECK(intersection_line_pattern BETWEEN 0 AND 4294967295), intersection_line_pattern_operation TEXT NOT NULL CHECK(intersection_line_pattern_operation IN ('inherit','set','disable','enable')),
 crease_angle_class TEXT NOT NULL CHECK(crease_angle_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), crease_angle_ieee754_bits INTEGER, crease_angle REAL CHECK((crease_angle_class='finite')=(crease_angle IS NOT NULL)), crease_angle_operation TEXT NOT NULL CHECK(crease_angle_operation IN ('inherit','set','disable','enable')),
 modifiers INTEGER NOT NULL CHECK(modifiers BETWEEN 0 AND 4294967295), modifiers_operation TEXT NOT NULL CHECK(modifiers_operation IN ('inherit','set','disable','enable')),
 color_operation TEXT NOT NULL CHECK(color_operation IN ('inherit','set','disable','enable')),
 opacity_class TEXT NOT NULL CHECK(opacity_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), opacity_ieee754_bits INTEGER, opacity REAL CHECK((opacity_class='finite')=(opacity IS NOT NULL)), opacity_operation TEXT NOT NULL CHECK(opacity_operation IN ('inherit','set','disable','enable')),
 width INTEGER NOT NULL CHECK(width BETWEEN 0 AND 4294967295), width_operation TEXT NOT NULL CHECK(width_operation IN ('inherit','set','disable','enable')),
 overhang INTEGER NOT NULL CHECK(overhang BETWEEN 0 AND 4294967295), overhang_operation TEXT NOT NULL CHECK(overhang_operation IN ('inherit','set','disable','enable')),
 jitter INTEGER NOT NULL CHECK(jitter BETWEEN 0 AND 4294967295), jitter_operation TEXT NOT NULL CHECK(jitter_operation IN ('inherit','set','disable','enable')),
 silhouette_color_operation TEXT NOT NULL CHECK(silhouette_color_operation IN ('inherit','set','disable','enable')),
 silhouette_width INTEGER NOT NULL CHECK(silhouette_width BETWEEN 0 AND 4294967295), silhouette_width_operation TEXT NOT NULL CHECK(silhouette_width_operation IN ('inherit','set','disable','enable')),
 halo_gap INTEGER NOT NULL CHECK(halo_gap BETWEEN 0 AND 4294967295), halo_gap_operation TEXT NOT NULL CHECK(halo_gap_operation IN ('inherit','set','disable','enable')),
 isolines INTEGER NOT NULL CHECK(isolines BETWEEN 0 AND 4294967295), isolines_operation TEXT NOT NULL CHECK(isolines_operation IN ('inherit','set','disable','enable')),
 hidden_edge_precision INTEGER NOT NULL CHECK(hidden_edge_precision IN (0,1)), hidden_edge_precision_operation TEXT NOT NULL CHECK(hidden_edge_precision_operation IN ('inherit','set','disable','enable'))
);
CREATE TABLE dwg_visual_style_display (
 id INTEGER PRIMARY KEY REFERENCES dwg_visual_style(id),
 settings INTEGER NOT NULL CHECK(settings BETWEEN 0 AND 4294967295), settings_operation TEXT NOT NULL CHECK(settings_operation IN ('inherit','set','disable','enable')),
 brightness_class TEXT NOT NULL CHECK(brightness_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), brightness_ieee754_bits INTEGER, brightness REAL CHECK((brightness_class='finite')=(brightness IS NOT NULL)), brightness_operation TEXT NOT NULL CHECK(brightness_operation IN ('inherit','set','disable','enable')),
 shadow_type INTEGER NOT NULL CHECK(shadow_type BETWEEN 0 AND 4294967295), shadow_type_operation TEXT NOT NULL CHECK(shadow_type_operation IN ('inherit','set','disable','enable'))
);
CREATE TABLE dwg_visual_face_monochrome_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_visual_style_face(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
CREATE TABLE dwg_visual_edge_intersection_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_visual_style_edge(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
CREATE TABLE dwg_visual_edge_obscured_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_visual_style_edge(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
CREATE TABLE dwg_visual_edge_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_visual_style_edge(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
CREATE TABLE dwg_visual_edge_silhouette_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_visual_style_edge(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
