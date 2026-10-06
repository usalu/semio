CREATE TABLE dwg_layout (
 id INTEGER PRIMARY KEY REFERENCES dwg_object(id), page_setup_name TEXT NOT NULL, printer_configuration TEXT NOT NULL, canonical_media_name TEXT NOT NULL, stylesheet TEXT NOT NULL, name TEXT NOT NULL,
 paper_unit TEXT NOT NULL CHECK(paper_unit='inches'), rotation TEXT NOT NULL CHECK(rotation='quarter_turn'), plot_area TEXT NOT NULL CHECK(plot_area IN ('display','layout')),
 paper_units_class TEXT NOT NULL CHECK(paper_units_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), paper_units_ieee754_bits INTEGER, paper_units REAL CHECK((paper_units_class='finite')=(paper_units IS NOT NULL)),
 drawing_units_class TEXT NOT NULL CHECK(drawing_units_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), drawing_units_ieee754_bits INTEGER, drawing_units REAL CHECK((drawing_units_class='finite')=(drawing_units IS NOT NULL)),
 standard_scale TEXT NOT NULL CHECK(standard_scale IN ('custom','one_to_one')), standard_scale_factor_class TEXT NOT NULL CHECK(standard_scale_factor_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), standard_scale_factor_ieee754_bits INTEGER, standard_scale_factor REAL CHECK((standard_scale_factor_class='finite')=(standard_scale_factor IS NOT NULL)),
 shade_plot TEXT NOT NULL CHECK(shade_plot='as_displayed'), shade_plot_resolution TEXT NOT NULL CHECK(shade_plot_resolution='normal'), shade_plot_dpi INTEGER NOT NULL CHECK(shade_plot_dpi BETWEEN 0 AND 65535), tab_order INTEGER NOT NULL CHECK(tab_order BETWEEN 0 AND 65535),
 paper_space_linetype_scaling INTEGER NOT NULL CHECK(paper_space_linetype_scaling IN (0,1)), ucs_elevation_class TEXT NOT NULL CHECK(ucs_elevation_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), ucs_elevation_ieee754_bits INTEGER, ucs_elevation REAL CHECK((ucs_elevation_class='finite')=(ucs_elevation IS NOT NULL)),
 orthographic_view TEXT NOT NULL CHECK(orthographic_view IN ('none','top','bottom','front','back','left','right')),
 plot_view_handle_high INTEGER CHECK(plot_view_handle_high BETWEEN 0 AND 4294967295), plot_view_handle_low INTEGER CHECK(plot_view_handle_low BETWEEN 0 AND 4294967295),
 visual_style_handle_high INTEGER CHECK(visual_style_handle_high BETWEEN 0 AND 4294967295), visual_style_handle_low INTEGER CHECK(visual_style_handle_low BETWEEN 0 AND 4294967295),
 block_header_handle_high INTEGER NOT NULL CHECK(block_header_handle_high BETWEEN 0 AND 4294967295), block_header_handle_low INTEGER NOT NULL CHECK(block_header_handle_low BETWEEN 0 AND 4294967295),
 active_viewport_handle_high INTEGER CHECK(active_viewport_handle_high BETWEEN 0 AND 4294967295), active_viewport_handle_low INTEGER CHECK(active_viewport_handle_low BETWEEN 0 AND 4294967295),
 base_ucs_handle_high INTEGER CHECK(base_ucs_handle_high BETWEEN 0 AND 4294967295), base_ucs_handle_low INTEGER CHECK(base_ucs_handle_low BETWEEN 0 AND 4294967295),
 named_ucs_handle_high INTEGER CHECK(named_ucs_handle_high BETWEEN 0 AND 4294967295), named_ucs_handle_low INTEGER CHECK(named_ucs_handle_low BETWEEN 0 AND 4294967295),
 CHECK((plot_view_handle_high IS NULL)=(plot_view_handle_low IS NULL)),CHECK((visual_style_handle_high IS NULL)=(visual_style_handle_low IS NULL)),CHECK((active_viewport_handle_high IS NULL)=(active_viewport_handle_low IS NULL)),CHECK((base_ucs_handle_high IS NULL)=(base_ucs_handle_low IS NULL)),CHECK((named_ucs_handle_high IS NULL)=(named_ucs_handle_low IS NULL))
);
CREATE TABLE dwg_plot_options (
 id INTEGER PRIMARY KEY REFERENCES dwg_layout(id), use_standard_scale INTEGER NOT NULL CHECK(use_standard_scale IN (0,1)), plot_viewport_borders INTEGER NOT NULL CHECK(plot_viewport_borders IN (0,1)), plot_with_lineweights INTEGER NOT NULL CHECK(plot_with_lineweights IN (0,1)), draw_viewports_first INTEGER NOT NULL CHECK(draw_viewports_first IN (0,1)), model_type INTEGER NOT NULL CHECK(model_type IN (0,1)), update_paper INTEGER NOT NULL CHECK(update_paper IN (0,1)), initializing INTEGER NOT NULL CHECK(initializing IN (0,1))
);
CREATE TABLE dwg_layout_coordinate (
 id INTEGER PRIMARY KEY, layout_id INTEGER NOT NULL REFERENCES dwg_layout(id), vector TEXT NOT NULL CHECK(vector IN ('margins','paper_size','plot_origin','plot_window_lower_left','plot_window_upper_right','paper_image_origin','insertion_base','limits_minimum','limits_maximum','ucs_origin','ucs_x_axis','ucs_y_axis','extents_minimum','extents_maximum')),
 ordinal INTEGER NOT NULL CHECK(ordinal>=0), component_ordinal INTEGER NOT NULL CHECK(component_ordinal>=0), coordinate_class TEXT NOT NULL CHECK(coordinate_class IN ('finite','negative_zero','nan','positive_infinity','negative_infinity')), coordinate_ieee754_bits INTEGER, coordinate REAL CHECK((coordinate_class='finite')=(coordinate IS NOT NULL))
);
CREATE TABLE dwg_layout_viewport_handle (
 id INTEGER PRIMARY KEY, layout_id INTEGER NOT NULL REFERENCES dwg_layout(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), handle_high INTEGER NOT NULL CHECK(handle_high BETWEEN 0 AND 4294967295), handle_low INTEGER NOT NULL CHECK(handle_low BETWEEN 0 AND 4294967295)
);
