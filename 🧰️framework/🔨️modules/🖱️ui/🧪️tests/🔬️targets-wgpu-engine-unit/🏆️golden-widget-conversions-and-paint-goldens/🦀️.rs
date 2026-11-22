// 🏆️ Golden widget conversions and paint goldens: the retained paint statistics helpers, the checkbox outline law and the selected-presence ring law.

//#region 🔖️GoldenHarness
/// 📊️ Total (ui_instances incl. overlay, vector_vertices incl. overlay, raster_instances) across
/// every layer of a `DrawList`. The retained shared-border finalizer tombstones each reserved edge
/// slot as a zero-area transparent instance, so only painted instances are counted.
fn stats(draw: &DrawList) -> (usize, usize, usize) {
    let painted = |instance: &&semio_ui_draw_contract::UiInstance| instance.rect[2] != 0.0 || instance.rect[3] != 0.0 || instance.color[3] != 0.0;
    let instances = draw.layers.iter().map(|layer| layer.ui_instances.iter().chain(layer.overlay_ui_instances.iter()).filter(painted).count()).sum();
    let vectors = draw.layers.iter().map(|layer| layer.vector_vertices.len() + layer.overlay_vector_vertices.len()).sum();
    let raster = draw.layers.iter().map(|layer| layer.raster_instances.len()).sum();
    (instances, vectors, raster)
}

fn retained_stats(node: &UiNode) -> (usize, usize, usize) {
    let mut ui = UiFixture::new(semio_ui_locale::Locale::En);
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree("golden", node);
    drive_layout(&mut ui, "golden", 400.0, 400.0, &mut atlas);
    let draw = ui.frame::<RecordingSceneHost>("golden", 400.0, 400.0, &mut atlas, None, None).expect("apply_tree then frame must produce a draw list");
    stats(draw)
}

/// 🧱️ Wraps a leaf `UiNode` as the sole child of a gap-less/padding-less vertical `Stack`, so the
/// retained layout grants it the full viewport bounds.
fn leaf(child: UiNode) -> UiNode {
    UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: Some("none".into()),
        padding: Some("none".into()),
        id: None,
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children: vec![child],
        menu: None,
    })
}

#[test]
fn checkbox_checked_state_does_not_paint_a_selection_outline() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧬️contract/🧪️tests/🧫️fixtures/♿️retained-toggle-semantics/🔣️.json")).unwrap();
    let paint = &fixture["checkboxPaint"];
    let width = paint["availableWidth"].as_f64().unwrap() as f32;
    let height = paint["availableHeight"].as_f64().unwrap() as f32;
    let side = paint["controlSide"].as_f64().unwrap() as f32;
    for checked in paint["checkedStates"].as_array().unwrap().iter().map(|value| value.as_bool().unwrap()) {
        let mut ui = UiFixture::new(semio_ui_locale::Locale::En);
        let mut atlas = FontAtlas::builtin();
        let toggle = UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Checkbox, id: "checkbox".into(), icon_id: IconName::CircleDot, text: None, on_change: action(), presence: UiPresence::selected(checked), menu: None });
        ui.apply_tree("checkbox-paint", &leaf(toggle));
        drive_layout(&mut ui, "checkbox-paint", width, height, &mut atlas);
        let draw = ui.frame::<RecordingSceneHost>("checkbox-paint", width, height, &mut atlas, None, None).expect("checkbox frame");
        let painted: Vec<_> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter().chain(layer.overlay_ui_instances.iter())).collect();
        assert!(!painted.is_empty());
        assert!(painted.iter().all(|instance| instance.rect[2] <= side + 0.01 && instance.rect[0] >= 0.0), "checked={checked}: a checkbox must not outline its {width}px allocation");
    }
}

/// ✨️ `presence.selected` draws its outset accent ring universally — proven here on `Toggle`, a
/// non-`Stack` variant, since `selected` used to be a `UiStackNode`-only field. Instance count
/// grows vs. the unselected fixture (the extra `push_chrome_border` edges), confirming the ring
/// is now a shared channel every element gets for free from `presence_overlay`.
#[test]
fn selected_presence_draws_an_outset_ring_on_any_element() {
    let unselected = UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::default(), menu: None });
    let selected =
        UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: "tog".into(), icon_id: IconName::CircleDot, text: Some(Label::data("On")), on_change: action(), presence: UiPresence::selected(true), menu: None });
    let (unselected_instances, _, _) = retained_stats(&leaf(unselected));
    let (selected_instances, _, _) = retained_stats(&leaf(selected));
    assert!(selected_instances > unselected_instances, "a selected element should paint more instances than an unselected one (the outset accent ring)");
}

//#endregion 🔖️GoldenHarness
