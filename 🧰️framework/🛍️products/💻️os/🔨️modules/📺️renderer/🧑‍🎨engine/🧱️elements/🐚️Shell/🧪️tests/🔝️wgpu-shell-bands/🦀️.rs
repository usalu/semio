//! 🔝️ LAW 4, 10 and 11: the navbar and footer are glass bands above the canvas (`shell-bands/v1`, `🔝️Navbar/bands.cases.json`): the canvas is the whole client, the body keeps the band height as clearance, a band wins every hit inside it and paints the lines it shares with the body.

use semio_renderer_wgpu_shell_session::{shell_band_hit, shell_band_layout, shell_footer_line_rect, shell_navbar_line_rect, ShellBandHit, ShellBandLayout};
use semio_ui_theme::{geometry::Rect, Level, Theme};

fn cases() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/bands.cases.json")).expect("shell bands cases")
}

fn rect(value: &serde_json::Value) -> Rect {
    let field = |name: &str| value[name].as_f64().expect(name) as f32;
    Rect::new(field("x"), field("y"), field("w"), field("h"))
}

fn assert_rect(actual: Rect, expected: Rect, what: &str) {
    assert!((actual.x - expected.x).abs() < 0.001 && (actual.y - expected.y).abs() < 0.001 && (actual.w - expected.w).abs() < 0.001 && (actual.h - expected.h).abs() < 0.001, "{what}: {actual:?} != {expected:?}");
}

fn layout_of(case: &serde_json::Value) -> ShellBandLayout {
    let number = |name: &str| case[name].as_f64().expect(name) as f32;
    shell_band_layout(case["client"]["w"].as_f64().expect("w") as f32, case["client"]["h"].as_f64().expect("h") as f32, number("navbar"), number("tutorial"), number("footer"), number("subfooter"), 1.0)
}

#[test]
fn the_bands_follow_the_language_neutral_cases() {
    let cases = cases();
    assert_eq!(cases["schema"], "shell-bands/v1");
    for case in cases["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let layout = layout_of(case);
        assert_rect(layout.canvas, rect(&case["canvas"]), &format!("{id} canvas runs from the top to the bottom client edge"));
        assert_rect(layout.body, rect(&case["body"]), &format!("{id} body keeps the band height as clearance"));
        assert_rect(layout.navbar, rect(&case["navbarRect"]), &format!("{id} navbar"));
        assert_rect(layout.footer, rect(&case["footerRect"]), &format!("{id} footer"));
        assert_rect(layout.navbar_line, rect(&case["navbarLine"]), &format!("{id} navbar line"));
        assert_rect(layout.footer_line, rect(&case["footerLine"]), &format!("{id} footer line"));
        for hit in case["hits"].as_array().expect("hits") {
            let expected = match hit["winner"].as_str().expect("winner") {
                "navbar" => ShellBandHit::Navbar,
                "footer" => ShellBandHit::Footer,
                _ => ShellBandHit::Body,
            };
            let (x, y) = (hit["x"].as_f64().expect("x") as f32, hit["y"].as_f64().expect("y") as f32);
            assert_eq!(shell_band_hit(&layout, x, y), Some(expected), "{id} hit at {x},{y}");
        }
    }
}

#[test]
fn lines_shared_with_a_band_lie_inside_the_band_and_the_canvas_reaches_beneath_it() {
    let theme = Theme::light();
    let layout = shell_band_layout(900.0, 600.0, theme.navbar_height, 0.0, theme.footer_height, 0.0, theme.stroke_hairline);
    let inside = |line: Rect, band: Rect| line.x >= band.x && line.y >= band.y && line.x + line.w <= band.x + band.w + 0.001 && line.y + line.h <= band.y + band.h + 0.001;
    assert!(inside(shell_navbar_line_rect(900.0, &theme), layout.navbar), "the navbar paints its own bottom line");
    assert!(inside(shell_footer_line_rect(layout.footer.y, 900.0, &theme), layout.footer), "the footer paints its own top line");
    assert!(layout.canvas.y < layout.body.y && layout.canvas.y + layout.canvas.h > layout.body.y + layout.body.h, "the canvas runs beneath both bands");
    assert!(layout.body.y >= layout.navbar.y + layout.navbar.h && layout.body.y + layout.body.h <= layout.footer.y, "nothing of the body lies under a band at rest");
}

#[test]
fn band_glass_is_panel_glass_with_the_chip_tint_and_stays_translucent() {
    let theme = Theme::light();
    let cases = cases();
    assert_eq!((cases["bandGlass"]["level"].as_str(), cases["bandGlass"]["chipTint"].as_bool()), (Some("panel"), Some(true)));
    let band = theme.band_glass();
    let panel = theme.glass(Level::Panel);
    let veil = (ui_styling::levels::CHIP_TINT_PERCENT / 100.0) as f32;
    assert!((band.alpha - (1.0 - (1.0 - panel.alpha) * (1.0 - veil))).abs() < 1e-5, "panel glass plus the veil");
    assert!(band.alpha < 1.0 && band.blur_px == panel.blur_px, "the canvas stays visible through the band");
    assert!((band.tint.r - panel.tint.r).abs() > 1e-6 || (band.tint.g - panel.tint.g).abs() > 1e-6, "the veil moves the tint away from the surface beneath");
}
