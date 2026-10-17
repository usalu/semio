/// 🧰️ Scene fixtures and drains: the empty, raster, paint2d and text editor scenes, the layer hit point and the action drains.

fn empty_scene(surface_id: &str, controller_id: &str, kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        host_id: surface_id.into(),
        surface_id: surface_id.into(),
        controller_id: controller_id.into(),
        component_kind: kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        menu: None,
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
    }
}

/// 🖼️ A two-pixel-layer raster document in the wire shape `RasterHost::sync_document_json` reads —
/// the same `RasterSnapshot.layers` projection the raster plugin publishes.
///
/// 🧾️ `schema` and per-layer `mask` are REQUIRED keys of that wire, not omittable ones:
/// `parse_document` refuses anything whose `schema` is not `"raster.document"`, and
/// `LayerNodeJson::Pixel` declares `mask` as an `Option<MaskJson>` with no serde default. The attach
/// path swallows the refusal (`let _ = host.sync_document_json(…)`,
/// `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🦀️.rs`, `sync_raster_engine`), so a fixture missing either key leaves the host with
/// NO layers — which reads downstream as "nothing is pickable anywhere in the viewport" rather than
/// as a parse fault. The raster plugin's own snapshots carry both
/// (`🖨️raster/…/📸️snapshot/⬅️before/🔣️.json`); this fixture carried neither
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration).
fn raster_document_json() -> String {
    json!({
        "schema": "raster.document",
        "layers": [
            { "kind": "pixel", "id": "base", "visible": true, "opacity": 1.0, "blendMode": "normal", "transform": {"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":-0.0,"d":1.0}, "mask": null, "width": 64, "height": 64 },
            { "kind": "pixel", "id": "overlay", "visible": true, "opacity": 0.5, "blendMode": "normal", "transform": {"x":0.0,"y":0.0,"a":1.0,"b":0.0,"c":-0.0,"d":1.0}, "mask": null, "width": 64, "height": 64 }
        ]
    })
    .to_string()
}

fn paint2d_scene(surface_id: &str, active_utility: &str, selection: &[&str]) -> UiComponentSceneNode {
    let mut scene = empty_scene(surface_id, "raster", SurfaceKind::Paint2d);
    scene.paint_2d = Some(Paint2dScene {
        document_sync_json: raster_document_json(),
        assets_json: "{}".into(),
        camera_json: json!({ "x": 0.0, "y": 0.0, "zoom": 1.0 }).to_string(),
        selection_json: serde_json::to_string(selection).expect("selection encodes"),
        hovered_id: None,
        active_utility: active_utility.into(),
        brush_size: 12.0,
        brush_opacity: 0.8,
        brush_color: "#e07020".into(),
        brush_hardness: 0.25,
        paint_target:"pixels".into(),mask_value:255,fill_tolerance:24,pixel_selection_json:None,
        view_mode: "composite".into(),
        composite_viewport_json: None,
        lanes: Vec::new(),
    });
    scene
}

/// ✏️ A text-editor surface whose declared caret sits at the END of `buffer`, the way a host that has
/// just loaded a document and focused it publishes one. The caret is part of the SCENE
/// (`selectionJson`), not an editor default — React's `TextEditorHost` restores it from the same
/// field — so a law about typing has to declare where the caret is instead of assuming zero
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration: the fixture said `{0,0}` while
/// its own assertions read `"alpha!"` and a caret of 6).
fn text_editor_scene(surface_id: &str, buffer: &str) -> UiComponentSceneNode {
    let mut scene = empty_scene(surface_id, "note", SurfaceKind::TextEditor);
    let caret = buffer.chars().count();
    scene.text_editor = Some(TextEditorScene::base(buffer.into(), Some("markdown".into()), Some(json!({ "start": caret, "end": caret }).to_string())));
    scene
}

/// 🎯️ A surface-local point the attached `RasterHost` reports a pixel layer under — found by
/// scanning, because the host owns the camera and this test pins the ACTION payload, not the
/// projection.
fn layer_hit_point(surface_id: &str, bounds: Rect) -> Option<(f32, f32)> {
    let mut y = 0.0f32;
    while y < bounds.h {
        let mut x = 0.0f32;
        while x < bounds.w {
            if paint2d_pick_layer(surface_id, f64::from(x), f64::from(y)).is_some() {
                return Some((bounds.x + x, bounds.y + y));
            }
            x += 8.0;
        }
        y += 8.0;
    }
    None
}

fn drain(input: &mut InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(input)
}

fn drain_editor_actions_accepted(input: &mut InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let _ = drive_text_editor_outbox_step(input).expect("editor outbox drive");
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority") {
        let queued = action.into_envelope().expect("action envelope");
        if let Some(receipt) = queued.receipt {
            settle_text_editor_action_receipt(receipt, TextEditorActionOutcome::Accepted);
        }
        actions.push(queued.descriptor);
    }
    actions
}
