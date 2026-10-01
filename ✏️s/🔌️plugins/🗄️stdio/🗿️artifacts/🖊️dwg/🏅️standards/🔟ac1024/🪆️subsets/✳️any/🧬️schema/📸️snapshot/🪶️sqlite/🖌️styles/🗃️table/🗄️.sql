CREATE TABLE dwg_table_style (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), description TEXT NOT NULL, bit_flags INTEGER NOT NULL CHECK(bit_flags BETWEEN 0 AND 4294967295), template_style_handle_high INTEGER CHECK(template_style_handle_high BETWEEN 0 AND 4294967295), template_style_handle_low INTEGER CHECK(template_style_handle_low BETWEEN 0 AND 4294967295), CHECK((template_style_handle_high IS NULL)=(template_style_handle_low IS NULL))
);
CREATE TABLE dwg_cell_style (
 id INTEGER PRIMARY KEY, table_style_id INTEGER NOT NULL REFERENCES dwg_table_style(id), ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 3), role TEXT NOT NULL CHECK(role IN ('table','title','header','data')),
 property_override_flags INTEGER NOT NULL CHECK(property_override_flags BETWEEN 0 AND 4294967295), merge_flags INTEGER NOT NULL CHECK(merge_flags BETWEEN 0 AND 4294967295), content_layout INTEGER NOT NULL CHECK(content_layout BETWEEN 0 AND 4294967295),
 CHECK((ordinal=0 AND role='table') OR (ordinal=1 AND role='title') OR (ordinal=2 AND role='header') OR (ordinal=3 AND role='data'))
);
CREATE TABLE dwg_cell_content_format (
 id INTEGER PRIMARY KEY REFERENCES dwg_cell_style(id), property_override_flags INTEGER NOT NULL CHECK(property_override_flags BETWEEN 0 AND 4294967295), property_flags INTEGER NOT NULL CHECK(property_flags BETWEEN 0 AND 4294967295),
 value_data_type INTEGER NOT NULL CHECK(value_data_type BETWEEN 0 AND 4294967295), value_unit_type INTEGER NOT NULL CHECK(value_unit_type BETWEEN 0 AND 4294967295), value_format_string TEXT NOT NULL,
 rotation_class TEXT NOT NULL CHECK(rotation_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), rotation_ieee754_bits INTEGER, rotation REAL CHECK((rotation_class='finite')=(rotation IS NOT NULL)),
 block_scale_class TEXT NOT NULL CHECK(block_scale_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), block_scale_ieee754_bits INTEGER, block_scale REAL CHECK((block_scale_class='finite')=(block_scale IS NOT NULL)),
 alignment INTEGER NOT NULL CHECK(alignment BETWEEN 0 AND 4294967295), text_style_handle_high INTEGER CHECK(text_style_handle_high BETWEEN 0 AND 4294967295), text_style_handle_low INTEGER CHECK(text_style_handle_low BETWEEN 0 AND 4294967295),
 text_height_class TEXT NOT NULL CHECK(text_height_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), text_height_ieee754_bits INTEGER, text_height REAL CHECK((text_height_class='finite')=(text_height IS NOT NULL)), CHECK((text_style_handle_high IS NULL)=(text_style_handle_low IS NULL))
);
CREATE TABLE dwg_cell_margins (
 id INTEGER PRIMARY KEY REFERENCES dwg_cell_style(id), vertical_class TEXT NOT NULL CHECK(vertical_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), vertical_ieee754_bits INTEGER, vertical REAL CHECK((vertical_class='finite')=(vertical IS NOT NULL)),
 horizontal_class TEXT NOT NULL CHECK(horizontal_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), horizontal_ieee754_bits INTEGER, horizontal REAL CHECK((horizontal_class='finite')=(horizontal IS NOT NULL)),
 bottom_class TEXT NOT NULL CHECK(bottom_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), bottom_ieee754_bits INTEGER, bottom REAL CHECK((bottom_class='finite')=(bottom IS NOT NULL)),
 right_class TEXT NOT NULL CHECK(right_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), right_ieee754_bits INTEGER, right REAL CHECK((right_class='finite')=(right IS NOT NULL)),
 horizontal_spacing_class TEXT NOT NULL CHECK(horizontal_spacing_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), horizontal_spacing_ieee754_bits INTEGER, horizontal_spacing REAL CHECK((horizontal_spacing_class='finite')=(horizontal_spacing IS NOT NULL)),
 vertical_spacing_class TEXT NOT NULL CHECK(vertical_spacing_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), vertical_spacing_ieee754_bits INTEGER, vertical_spacing REAL CHECK((vertical_spacing_class='finite')=(vertical_spacing IS NOT NULL))
);
CREATE TABLE dwg_cell_border (
 id INTEGER PRIMARY KEY, cell_style_id INTEGER NOT NULL REFERENCES dwg_cell_style(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), edge TEXT NOT NULL CHECK(edge IN ('top','horizontal_inside','bottom','left','vertical_inside','right')),
 override_flags INTEGER NOT NULL CHECK(override_flags BETWEEN 0 AND 4294967295), border_type INTEGER NOT NULL CHECK(border_type BETWEEN 0 AND 4294967295), lineweight INTEGER NOT NULL CHECK(lineweight BETWEEN -2147483648 AND 2147483647),
 linetype_handle_high INTEGER CHECK(linetype_handle_high BETWEEN 0 AND 4294967295), linetype_handle_low INTEGER CHECK(linetype_handle_low BETWEEN 0 AND 4294967295), visible INTEGER NOT NULL CHECK(visible BETWEEN 0 AND 4294967295),
 double_line_spacing_class TEXT NOT NULL CHECK(double_line_spacing_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), double_line_spacing_ieee754_bits INTEGER, double_line_spacing REAL CHECK((double_line_spacing_class='finite')=(double_line_spacing IS NOT NULL)), CHECK((linetype_handle_high IS NULL)=(linetype_handle_low IS NULL))
);
CREATE TABLE dwg_cell_background_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_cell_style(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
CREATE TABLE dwg_cell_content_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_cell_content_format(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
CREATE TABLE dwg_cell_border_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_cell_border(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
