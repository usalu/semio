// 🖱️ Widget interaction and render laws: the interaction maps, the widget render registrations for inputs, selects, sliders, steppers, rings, fields, sections and trees, the scroll regions and the draw text channels.

//#region 🧩️WidgetsInternalsTests
#[test]
fn widget_interaction_maps_clear_frame_empties_every_map() {
    let mut maps = WidgetInteractionMaps::<ActionDescriptor>::default();
    maps.input_metas.insert("i".into(), InputMeta { on_change: action(), commit: None, value: "v".into(), input_kind: "text".into(), min: None, max: None, step: None, accept: None });
    maps.select_metas.insert("s".into(), action());
    maps.toggle_metas.insert("t".into(), (true, action()));
    maps.slider_metas.insert("sl".into(), SliderMeta { on_change: action(), min: 0.0, max: 1.0, step: 0.1, value: 0.5, bounds_x: 0.0, bounds_w: 10.0 });
    maps.stepper_metas.insert("st".into(), StepperMeta { on_absolute: action(), on_delta: action(), step: 1.0, value: 1.0 });
    maps.ring_metas.insert("r".into(), RingMeta { on_change: action(), disabled: false, center_x: 0.0, center_y: 0.0, radius: 10.0 });
    maps.slider_live_values.insert("sl".into(), 0.5);
    maps.ring_live_values.insert("r".into(), 0.5);
    maps.tree_hover_commands.insert("h".into(), action());
    maps.tree_unhover_commands.insert("u".into(), action());
    maps.tree_selection_change = Some(action());

    maps.clear_frame();

    assert!(maps.input_metas.is_empty());
    assert!(maps.select_metas.is_empty());
    assert!(maps.toggle_metas.is_empty());
    assert!(maps.slider_metas.is_empty());
    assert!(maps.stepper_metas.is_empty());
    assert!(maps.ring_metas.is_empty());
    assert!(maps.slider_live_values.is_empty());
    assert!(maps.ring_live_values.is_empty());
    assert!(maps.tree_hover_commands.is_empty());
    assert!(maps.tree_unhover_commands.is_empty());
    assert!(maps.tree_selection_change.is_none());
}

struct WidgetHarness {
    draw: DrawList,
    atlas: FontAtlas,
    theme: Theme,
    input: semio_ui_wgpu_input::InputState<ActionDescriptor>,
    scroll_offsets: std::collections::HashMap<String, f32>,
    collapsed_sections: std::collections::HashMap<String, bool>,
    open_selects: std::collections::HashMap<String, bool>,
    maps: WidgetInteractionMaps<ActionDescriptor>,
}

impl WidgetHarness {
    fn new() -> Self {
        Self { draw: DrawList::default(), atlas: FontAtlas::builtin(), theme: Theme::default(), input: Default::default(), scroll_offsets: Default::default(), collapsed_sections: Default::default(), open_selects: Default::default(), maps: Default::default() }
    }

    fn ctx(&mut self) -> semio_ui_wgpu_widget_kit::WidgetContext<'_, ActionDescriptor> {
        semio_ui_wgpu_widget_kit::WidgetContext {
            draw: &mut self.draw,
            overlay: None,
            atlas: &mut self.atlas,
            icons: None,
            input: &mut self.input,
            theme: &self.theme,
            scroll_offsets: &mut self.scroll_offsets,
            collapsed_sections: &mut self.collapsed_sections,
            open_selects: &mut self.open_selects,
            interaction_maps: Some(&mut self.maps),
            pick_clip: None,
            viewport_height: 0.0,
        }
    }
}

#[test]
fn render_scroll_region_clamps_stale_offset_to_new_max_scroll() {
    let mut h = WidgetHarness::new();
    h.scroll_offsets.insert("scroll".into(), 500.0);
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    {
        let mut ctx = h.ctx();
        render_scroll_region("scroll", bounds, 150.0, &mut ctx, |_content, _ctx| {});
    }
    assert_eq!(h.scroll_offsets.get("scroll"), Some(&50.0), "offset must clamp to max_scroll (content_height - bounds.h) even if a stale value was larger");
}

#[test]
fn render_scroll_region_registers_a_scroll_region_hit_target() {
    let mut h = WidgetHarness::new();
    let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
    {
        let mut ctx = h.ctx();
        render_scroll_region("myscroll", bounds, 400.0, &mut ctx, |_content, _ctx| {});
    }
    assert!(h.input.staged_hits().iter().any(|t| t.control_id.as_deref() == Some("myscroll")));
}

#[test]
fn draw_text_on_emits_one_glyph_instance_per_character() {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    draw_text_on(&mut draw, &mut atlas, "abc", 0.0, 0.0, 16.0, Theme::default().text);
    let total: usize = draw.layers.iter().map(|l| l.ui_instances.len()).sum();
    assert_eq!(total, 3);
}

#[test]
fn draw_text_overlay_on_writes_to_the_overlay_channel_not_the_main_one() {
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    draw_text_overlay_on(&mut draw, &mut atlas, "hi", 0.0, 0.0, 16.0, Theme::default().text);
    let main: usize = draw.layers.iter().map(|l| l.ui_instances.len()).sum();
    let overlay: usize = draw.layers.iter().map(|l| l.overlay_ui_instances.len()).sum();
    assert_eq!(main, 0, "overlay glyphs must not land in the main ui_instances channel");
    assert_eq!(overlay, 2, "one overlay glyph instance per character");
}
//#endregion 🧩️WidgetsInternalsTests
