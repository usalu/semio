CREATE TABLE dwg_mleader_style (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), content_type TEXT NOT NULL CHECK(content_type IN ('none','block','mtext')), draw_order TEXT NOT NULL CHECK(draw_order IN ('leader_first','content_first')), leader_order TEXT NOT NULL CHECK(leader_order IN ('head_first','tail_first')), maximum_segment_points INTEGER NOT NULL CHECK(maximum_segment_points BETWEEN 0 AND 4294967295),
 first_segment_angle_class TEXT NOT NULL CHECK(first_segment_angle_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), first_segment_angle_ieee754_bits INTEGER, first_segment_angle REAL CHECK((first_segment_angle_class='finite')=(first_segment_angle IS NOT NULL)),
 second_segment_angle_class TEXT NOT NULL CHECK(second_segment_angle_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), second_segment_angle_ieee754_bits INTEGER, second_segment_angle REAL CHECK((second_segment_angle_class='finite')=(second_segment_angle IS NOT NULL)),
 description TEXT NOT NULL, overall_scale_class TEXT NOT NULL CHECK(overall_scale_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), overall_scale_ieee754_bits INTEGER, overall_scale REAL CHECK((overall_scale_class='finite')=(overall_scale IS NOT NULL)),
 property_overrides_changed INTEGER NOT NULL CHECK(property_overrides_changed IN (0,1)), annotative INTEGER NOT NULL CHECK(annotative IN (0,1)),
 break_size_class TEXT NOT NULL CHECK(break_size_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), break_size_ieee754_bits INTEGER, break_size REAL CHECK((break_size_class='finite')=(break_size IS NOT NULL))
);
CREATE TABLE dwg_mleader_leader_style (
 id INTEGER PRIMARY KEY REFERENCES dwg_mleader_style(id), kind TEXT NOT NULL CHECK(kind IN ('invisible','straight','spline')), linetype_style_handle_high INTEGER NOT NULL CHECK(linetype_style_handle_high BETWEEN 0 AND 4294967295), linetype_style_handle_low INTEGER NOT NULL CHECK(linetype_style_handle_low BETWEEN 0 AND 4294967295), lineweight INTEGER NOT NULL CHECK(lineweight BETWEEN -2147483648 AND 2147483647)
);
CREATE TABLE dwg_mleader_landing (
 id INTEGER PRIMARY KEY REFERENCES dwg_mleader_style(id), enabled INTEGER NOT NULL CHECK(enabled IN (0,1)), gap_class TEXT NOT NULL CHECK(gap_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), gap_ieee754_bits INTEGER, gap REAL CHECK((gap_class='finite')=(gap IS NOT NULL))
);
CREATE TABLE dwg_mleader_dogleg (
 id INTEGER PRIMARY KEY REFERENCES dwg_mleader_style(id), enabled INTEGER NOT NULL CHECK(enabled IN (0,1)), length_class TEXT NOT NULL CHECK(length_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), length_ieee754_bits INTEGER, length REAL CHECK((length_class='finite')=(length IS NOT NULL))
);
CREATE TABLE dwg_mleader_arrow (
 id INTEGER PRIMARY KEY REFERENCES dwg_mleader_style(id), symbol_handle_high INTEGER CHECK(symbol_handle_high BETWEEN 0 AND 4294967295), symbol_handle_low INTEGER CHECK(symbol_handle_low BETWEEN 0 AND 4294967295), size_class TEXT NOT NULL CHECK(size_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), size_ieee754_bits INTEGER, size REAL CHECK((size_class='finite')=(size IS NOT NULL)), CHECK((symbol_handle_high IS NULL)=(symbol_handle_low IS NULL))
);
CREATE TABLE dwg_mleader_text_style (
 id INTEGER PRIMARY KEY REFERENCES dwg_mleader_style(id), default_content TEXT NOT NULL, style_handle_high INTEGER NOT NULL CHECK(style_handle_high BETWEEN 0 AND 4294967295), style_handle_low INTEGER NOT NULL CHECK(style_handle_low BETWEEN 0 AND 4294967295),
 left_attachment TEXT NOT NULL CHECK(left_attachment IN ('top_of_top','middle_of_top','middle','middle_of_bottom','bottom_of_bottom','bottom_line','bottom_of_top','bottom_of_top_underline','bottom_of_top_no_underline','center')),
 right_attachment TEXT NOT NULL CHECK(right_attachment IN ('top_of_top','middle_of_top','middle','middle_of_bottom','bottom_of_bottom','bottom_line','bottom_of_top','bottom_of_top_underline','bottom_of_top_no_underline','center')),
 angle TEXT NOT NULL CHECK(angle IN ('horizontal','aligned','always_right_reading')), alignment TEXT NOT NULL CHECK(alignment IN ('left','center','right')),
 height_class TEXT NOT NULL CHECK(height_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), height_ieee754_bits INTEGER, height REAL CHECK((height_class='finite')=(height IS NOT NULL)), frame INTEGER NOT NULL CHECK(frame IN (0,1)), always_left INTEGER NOT NULL CHECK(always_left IN (0,1)),
 alignment_space_class TEXT NOT NULL CHECK(alignment_space_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), alignment_space_ieee754_bits INTEGER, alignment_space REAL CHECK((alignment_space_class='finite')=(alignment_space IS NOT NULL)),
 attachment_direction TEXT NOT NULL CHECK(attachment_direction IN ('horizontal','vertical')),
 top_attachment TEXT NOT NULL CHECK(top_attachment IN ('top_of_top','middle_of_top','middle','middle_of_bottom','bottom_of_bottom','bottom_line','bottom_of_top','bottom_of_top_underline','bottom_of_top_no_underline','center')),
 bottom_attachment TEXT NOT NULL CHECK(bottom_attachment IN ('top_of_top','middle_of_top','middle','middle_of_bottom','bottom_of_bottom','bottom_line','bottom_of_top','bottom_of_top_underline','bottom_of_top_no_underline','center'))
);
CREATE TABLE dwg_mleader_block_style (
 id INTEGER PRIMARY KEY REFERENCES dwg_mleader_style(id), content_handle_high INTEGER CHECK(content_handle_high BETWEEN 0 AND 4294967295), content_handle_low INTEGER CHECK(content_handle_low BETWEEN 0 AND 4294967295), use_scale INTEGER NOT NULL CHECK(use_scale IN (0,1)),
 rotation_class TEXT NOT NULL CHECK(rotation_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), rotation_ieee754_bits INTEGER, rotation REAL CHECK((rotation_class='finite')=(rotation IS NOT NULL)), use_rotation INTEGER NOT NULL CHECK(use_rotation IN (0,1)), connection TEXT NOT NULL CHECK(connection IN ('extents','base_point')), CHECK((content_handle_high IS NULL)=(content_handle_low IS NULL))
);
CREATE TABLE dwg_mleader_block_scale_coordinate (
 id INTEGER PRIMARY KEY, mleader_block_style_id INTEGER NOT NULL REFERENCES dwg_mleader_block_style(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL CHECK((coordinate_class='finite')=(coordinate IS NOT NULL))
);
CREATE TABLE dwg_mleader_leader_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_mleader_leader_style(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
CREATE TABLE dwg_mleader_text_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_mleader_text_style(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
CREATE TABLE dwg_mleader_block_color (
 id INTEGER PRIMARY KEY REFERENCES dwg_mleader_block_style(id), color_index INTEGER NOT NULL CHECK(color_index BETWEEN 0 AND 65535), kind TEXT NOT NULL CHECK(kind IN ('none','by_layer','by_block','by_color','by_aci','by_pen','foreground','layer_off','layer_frozen')), red INTEGER CHECK(red BETWEEN 0 AND 255), green INTEGER CHECK(green BETWEEN 0 AND 255), blue INTEGER CHECK(blue BETWEEN 0 AND 255), value_index INTEGER CHECK(value_index BETWEEN 0 AND 65535), name TEXT, book_name TEXT,
 CHECK((kind='by_color')=(red IS NOT NULL)),CHECK((red IS NULL)=(green IS NULL)),CHECK((red IS NULL)=(blue IS NULL)),CHECK((kind IN ('by_aci','by_pen'))=(value_index IS NOT NULL)),CHECK(kind!='by_pen' OR value_index<=255)
);
