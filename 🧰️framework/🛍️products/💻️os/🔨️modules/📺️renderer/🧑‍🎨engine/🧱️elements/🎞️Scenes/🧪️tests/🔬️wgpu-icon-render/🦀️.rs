use super::*;

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
    // 🖼️ The top border strip is `[frame.x, frame.y, frame.w, hair]` — its rect's height (index 3)
    // must be exactly 2.0, matching React's `border-2`.
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
    assert_eq!(environment["material"], fixture["worldEnvironment"]["material"]);
}

#[test]
fn icon_shadow_profile_matches_react_png_material_and_svg_exclusions() {
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
    assert_eq!(icon_render_shadow_profile(&request("svg", material)), infinite_world::world::World3dShadowProfile::Unshadowed);
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
        render_icon_render_status(&mut ctx, frame, ICON_RENDER_RENDERING_MESSAGE, theme.text_muted);
    }
    let glyphs: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).collect();
    assert!(!glyphs.is_empty(), "the rendering status line must paint glyphs");
    for rect in &glyphs {
        assert!(rect[0] >= frame.x - 1.0 && rect[0] <= frame.x + frame.w, "status glyph {rect:?} left the shot frame {frame:?}");
        assert!(rect[1] >= frame.y - 1.0 && rect[1] <= frame.y + frame.h, "status glyph {rect:?} left the shot frame {frame:?}");
    }
    let _ = bounds;
}
