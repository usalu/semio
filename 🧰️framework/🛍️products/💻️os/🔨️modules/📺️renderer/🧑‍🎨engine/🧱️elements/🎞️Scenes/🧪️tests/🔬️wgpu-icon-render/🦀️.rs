
use super::*;

#[test]
fn frame_border_is_two_px_and_badge_uses_background_token() {
    let request: IconRenderRequestFields = serde_json::from_str(r#"{"assetUrl":"mesh://x","camera":{"position":[0,0,5],"target":[0,0,0]},"width":64.0,"height":64.0,"shape":"rectangle"}"#).unwrap();
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
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None);
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
