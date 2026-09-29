use super::*;

#[test]
fn icon_status_matches_the_shared_host_lifecycle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🖼️IconRenderHost/🧫️fixtures/🏷️status/🔣️.json")).unwrap();
    let pack = &fixture["locales"][0];
    let labels = SceneChromeLabels::english().icon_render;
    for sample in fixture["cases"].as_array().unwrap() {
        let message = if sample["request"] == false { Some(labels.empty_scene) } else { icon_render_status_message(icon_render_status(sample["resident"].as_bool().unwrap(), sample["faulted"].as_bool().unwrap() || sample["assetMiss"].as_bool().unwrap()), labels) };
        assert_eq!(message, pack["labels"][sample["state"].as_str().unwrap()].as_str(), "{} {}", pack["locale"], sample["state"]);
    }
}

#[test]
fn a_refused_icon_subject_leaves_rendering_without_faulting_the_world_snapshot() {
    let mut state = infinite_world::world::World3dState::new("icon-status-test".into(), "icon-status-controller".into());
    let url = "/mesh/refused.glb";
    let id = semio_framework_plugin::world3d_mesh_id_from_url(url);
    assert_eq!(icon_render_subject_status(&state, &id, url), IconRenderStatus::Rendering);
    infinite_world::world::mark_world3d_asset_miss(&mut state, url);
    assert_eq!(state.snapshot_fault(), None);
    assert_eq!(icon_render_subject_status(&state, &id, url), IconRenderStatus::Failed);
    assert_eq!(icon_render_subject_status(&state, "other", "/mesh/other.glb"), IconRenderStatus::Rendering);
}

#[test]
fn icon_frame_geometry_matches_the_actual_react_host() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🖼️IconRenderHost/🧫️fixtures/🖼️frame-presentation/🔣️.json")).unwrap();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::shaped_default();
    let theme = Theme::default();
    for sample in fixture["cases"].as_array().unwrap() {
        let bounds = Rect::new(0.0, 0.0, sample["container"][0].as_f64().unwrap() as f32, sample["container"][1].as_f64().unwrap() as f32);
        let footer = icon_render_footer_layout(bounds, sample["footer"].as_str(), &mut atlas, theme.font_size_small);
        let frame = icon_render_frame(bounds, sample["width"].as_f64().unwrap(), sample["height"].as_f64().unwrap(), footer.0);
        let content = icon_render_content_frame(frame);
        let badge = format!("{}×{} · {}", sample["width"].as_f64().unwrap().round() as i64, sample["height"].as_f64().unwrap().round() as i64, sample["shape"].as_str().unwrap());
        let badge_rect = icon_render_badge_layout(frame, &badge, &mut atlas);
        let lines: Vec<_> = atlas.wrap_lines_face(TextFace::Mono, &badge, badge_rect.w - 8.0, 10.0).into_iter().map(|range| badge[range].trim_end().to_owned()).collect();
        assert_eq!(serde_json::json!(lines), sample["expected"]["badge"]["lines"]);
        for (axis, value) in [badge_rect.x, badge_rect.y, badge_rect.w, badge_rect.h].into_iter().enumerate() {
            assert!((f64::from(value) - sample["expected"]["badge"]["bounds"][axis].as_f64().unwrap()).abs() < fixture["tolerance"].as_f64().unwrap(), "{} badge[{axis}]", sample["id"]);
        }
        for (field, actual) in [("frame", frame), ("content", content)] {
            for (axis, value) in [actual.x, actual.y, actual.w, actual.h].into_iter().enumerate() {
                assert!((f64::from(value) - sample["expected"][field][axis].as_f64().unwrap()).abs() < fixture["tolerance"].as_f64().unwrap(), "{} {field}[{axis}]", sample["id"]);
            }
        }
        assert_eq!(footer.0, sample["expected"]["footer"].as_array().map_or(0.0, |rect| rect[3].as_f64().unwrap() as f32));
    }
}

#[test]
fn icon_camera_matches_the_shared_three_projection_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧫️fixtures/🖼️icon-render-camera/🔣️.json")).unwrap();
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    for sample in fixture["cases"].as_array().unwrap() {
        let request: IconRenderRequestFields = serde_json::from_value(sample["request"].clone()).unwrap();
        let minimum: [f32; 3] = serde_json::from_value(sample["model"]["minimum"].clone()).unwrap();
        let maximum: [f32; 3] = serde_json::from_value(sample["model"]["maximum"].clone()).unwrap();
        let width = sample["preview"]["width"].as_f64().unwrap();
        let height = sample["preview"]["height"].as_f64().unwrap();
        let actual: serde_json::Value = serde_json::from_str(&icon_render_camera_json(&request, Some((minimum, maximum)), width / request.width)).unwrap();
        let expected = &sample["expected"];
        for field in ["position", "target"] {
            for axis in 0..3 {
                assert!((actual[field][axis].as_f64().unwrap() - expected[field][axis].as_f64().unwrap()).abs() < tolerance, "{} {field}[{axis}]: {actual}", sample["id"]);
            }
        }
        assert!((actual["zoom"].as_f64().unwrap() - expected["previewZoom"].as_f64().unwrap()).abs() < tolerance, "{} zoom", sample["id"]);
        assert!((actual["fov"].as_f64().unwrap() - expected["effectiveFov"].as_f64().unwrap()).abs() < tolerance, "{} fov", sample["id"]);
        assert_eq!(actual["projectionFrame"], "preserveCamera");
        let spec: semio_framework_ui_viewport::Viewport3dProjectionSpec = serde_json::from_value(actual["projection"].clone()).unwrap();
        let vector = |field: &str| ui_wgpu::wgpu::Vec3::new(actual[field][0].as_f64().unwrap() as f32, actual[field][1].as_f64().unwrap() as f32, actual[field][2].as_f64().unwrap() as f32);
        let camera = ui_wgpu::wgpu::Camera3d { position: vector("position"), target: vector("target"), up: vector("up"), zoom: actual["zoom"].as_f64().unwrap() as f32, ..Default::default() };
        let matrix = ui_wgpu::wgpu::projection_spec_view_proj(&camera, spec, width as f32, height as f32);
        let centre = ui_wgpu::wgpu::projection_spec_project_point(matrix, spec, camera.target, width as f32, height as f32).unwrap();
        assert!((f64::from(centre[0]) - width / 2.0).abs() < 0.001 && (f64::from(centre[1]) - height / 2.0).abs() < 0.001, "{} centred target", sample["id"]);
        if sample["request"]["camera"]["projection"] == "orthographic" {
            let next = ui_wgpu::wgpu::projection_spec_project_point(matrix, spec, ui_wgpu::wgpu::Vec3::new(camera.target.x + 1.0, camera.target.y, camera.target.z), width as f32, height as f32).unwrap();
            assert!((f64::from(next[0] - centre[0]) - expected["previewZoom"].as_f64().unwrap()).abs() < 0.001, "{} preview scaling", sample["id"]);
        }
    }
}

/// 🖼️ The top border strip is `[frame.x, frame.y, frame.w, hair]` — its rect's height (index 3)
/// must be exactly 2.0, matching React's `border-2`.
#[test]
fn frame_border_is_two_px_and_badge_uses_background_token() {
    let request: IconRenderRequestFields = serde_json::from_str(r#"{"assetUrl":"mesh://x","format":"png","camera":{"position":[0,0,5],"target":[0,0,0]},"width":64.0,"height":64.0,"shape":"rectangle"}"#).unwrap();
    let bounds = Rect::new(0.0, 0.0, 200.0, 200.0);
    let frame = Rect::new(20.0, 20.0, 160.0, 160.0);

    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        paint_icon_render_chrome(&mut ctx, bounds, frame, &request, "rectangle", None);
    }
    let top_border = draw
        .layers
        .iter()
        .flat_map(|layer| layer.ui_instances.iter())
        .find(|instance| instance.rect[0] == frame.x && instance.rect[1] == frame.y && instance.rect[2] == frame.w)
        .unwrap_or_else(|| panic!("expected the top frame-border strip to be pushed"));
    assert_eq!(top_border.rect[3], 2.0, "the frame border must be 2px, matching border-2 in icon-render-host.tsx");

    let expected_badge_bg = theme.background.with_alpha(0.8);
    let stale_badge_bg = theme.panel.with_alpha(0.8);
    let colors: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|i| i.color).collect();
    assert!(colors.contains(&[expected_badge_bg.r, expected_badge_bg.g, expected_badge_bg.b, expected_badge_bg.a]), "expected the badge chip to use theme.background@0.8, got {colors:?}");
    assert!(!colors.contains(&[stale_badge_bg.r, stale_badge_bg.g, stale_badge_bg.b, stale_badge_bg.a]), "the badge chip must no longer use the stale theme.panel@0.8 token");
}

#[test]
fn an_ellipse_shot_border_follows_both_ellipse_perimeters() {
    let request: IconRenderRequestFields = serde_json::from_str(r#"{"assetUrl":"mesh://x","format":"png","camera":{"position":[0,0,5],"target":[0,0,0]},"width":256,"height":128,"shape":"ellipse"}"#).unwrap();
    let frame = Rect::new(0.0, 50.0, 400.0, 200.0);
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
    paint_icon_render_chrome(&mut ctx, Rect::new(0.0, 0.0, 400.0, 300.0), frame, &request, "ellipse", None);
    for instance in draw.layers.iter().flat_map(|layer| &layer.ui_instances) {
        assert_eq!(instance.clip_ellipse, [2.0, 52.0, 396.0, 196.0], "badge glyphs and background share the content ellipse");
    }
    let vertices: Vec<_> = draw.layers.iter().flat_map(|layer| &layer.vector_vertices).collect();
    assert_eq!(vertices.len(), 128 * 6);
    for vertex in vertices {
        let x = vertex.position[0] - 200.0;
        let y = vertex.position[1] - 150.0;
        let outer = (x / 200.0).powi(2) + (y / 100.0).powi(2);
        let inner = (x / 198.0).powi(2) + (y / 98.0).powi(2);
        assert!((outer - 1.0).abs() < 1e-5 || (inner - 1.0).abs() < 1e-5, "only the 2px elliptical ring is painted");
    }
}

#[test]
fn rectangular_icon_foreground_clips_to_its_content_and_restores_the_parent() {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let parent = Rect::new(190.0, 0.0, 15.0, 300.0);
    draw.push_scissor(parent);
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
    with_icon_render_mask(&mut ctx, Rect::new(183.25, 2.0, 33.5, 296.0), "rectangle", |ctx| ctx.draw.push_solid([0.0, 0.0, 400.0, 300.0], theme.text));
    let layer = draw.layers.iter().find(|layer| !layer.ui_instances.is_empty()).unwrap();
    let clip = layer.scissor.unwrap();
    assert_eq!((clip.x, clip.y, clip.w, clip.h), (190, 2, 15, 296));
    assert_eq!(layer.ui_instances[0].clip_ellipse, [0.0; 4]);
    let restored = draw.layers.last().unwrap().scissor.unwrap();
    assert_eq!((restored.x, restored.y, restored.w, restored.h), (190, 0, 15, 300));
}

/// ⚖️ Law: the three states React's `IconRenderHost` shows — error, ready, and `ui.host.rendering`
/// while the shot is still being produced (`🖼️IconRenderHost/🟦️.tsx:55-61`) — must all exist here.
/// The wgpu twin used to have ONE: an empty shot frame for the whole fetch, and forever on a miss.
#[test]
fn icon_render_status_reports_rendering_until_the_subject_mesh_is_resident() {
    assert_eq!(icon_render_status(false, false), IconRenderStatus::Rendering, "no lease and no fault is React's `Rendering…`");
    assert_eq!(icon_render_status(true, false), IconRenderStatus::Ready, "a published lease means the frame draws real geometry");
    assert_eq!(icon_render_status(false, true), IconRenderStatus::Failed, "a recorded fault with no lease is React's error arm");
    assert_eq!(icon_render_status(true, true), IconRenderStatus::Ready, "residency wins: the shot IS on screen whatever else faulted");
}

#[test]
fn icon_render_maps_the_shared_lighting_fixture_to_the_world_environment() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../♾️infinite/🌍️world/🧫️fixtures/🌞️scene-lighting/🔣️.json")).unwrap();
    let request: IconRenderRequestFields = serde_json::from_value(fixture["iconRenderRequest"].clone()).unwrap();
    let environment: serde_json::Value = serde_json::from_str(&icon_render_environment_json(&request)).unwrap();
    assert_eq!(environment["ambient"], fixture["worldEnvironment"]["ambient"]);
    assert_eq!(environment["sun"]["enabled"], fixture["worldEnvironment"]["sun"]["enabled"]);
    assert_eq!(environment["sun"]["color"], fixture["worldEnvironment"]["sun"]["color"]);
    for field in ["azimuth", "elevation", "intensity"] {
        assert_eq!(environment["sun"][field].as_f64(), fixture["worldEnvironment"]["sun"][field].as_f64(), "sun.{field} carries the fixture's numeric value");
    }
    assert_eq!(environment["shadow"], serde_json::json!({ "enabled": true }));
    for (field, value) in fixture["worldEnvironment"]["material"].as_object().expect("the lit material") {
        assert_eq!(&environment["material"][field], value, "material.{field} carries the fixture's lighting value");
    }
    let outline: serde_json::Value = serde_json::from_str(include_str!("../../../🖼️IconRenderHost/🧫️fixtures/📤️svg-export/🔣️.json")).unwrap();
    assert_eq!(environment["material"]["stroke"], outline["material"]["stroke"], "an icon outline defaults to React's own stroke");
}

#[test]
fn icon_render_profile_matches_react_png_shadows_and_svg_face_lighting() {
    let request = |format: &str, material: serde_json::Value| {
        serde_json::from_value::<IconRenderRequestFields>(serde_json::json!({
            "assetUrl": "mesh://x",
            "format": format,
            "camera": { "position": [0, 0, 5], "target": [0, 0, 0] },
            "width": 64,
            "height": 64,
            "material": material,
        }))
        .unwrap()
    };
    let material = serde_json::json!({ "color": "#ffffff" });
    assert_eq!(icon_render_shadow_profile(&request("png", material.clone())), infinite_world::world::World3dShadowProfile::IconPng);
    assert_eq!(icon_render_shadow_profile(&request("png", serde_json::Value::Null)), infinite_world::world::World3dShadowProfile::Unshadowed);
    assert_eq!(icon_render_shadow_profile(&request("svg", material)), infinite_world::world::World3dShadowProfile::IconSvg);
}

/// 🖼️ The status line is centred INSIDE the shot frame, like React's text inside `IconShotFrame`.
#[test]
fn icon_render_status_line_paints_inside_the_frame() {
    let bounds = Rect::new(0.0, 0.0, 200.0, 200.0);
    let frame = Rect::new(20.0, 20.0, 160.0, 160.0);
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_icon_render_status(&mut ctx, frame, SceneChromeLabels::english().icon_render.rendering, theme.text_muted);
    }
    let glyphs: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).collect();
    assert!(!glyphs.is_empty(), "the rendering status line must paint glyphs");
    for rect in &glyphs {
        assert!(rect[0] >= frame.x - 1.0 && rect[0] <= frame.x + frame.w, "status glyph {rect:?} left the shot frame {frame:?}");
        assert!(rect[1] >= frame.y - 1.0 && rect[1] <= frame.y + frame.h, "status glyph {rect:?} left the shot frame {frame:?}");
    }
    let _ = bounds;
}
