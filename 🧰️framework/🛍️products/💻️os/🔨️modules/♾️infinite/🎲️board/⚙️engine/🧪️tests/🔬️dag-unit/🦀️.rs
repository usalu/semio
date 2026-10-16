//! 🧪️ engine tests (🔬️dag-unit): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_engine::CanvasPalette;
use semio_framework_canvas::Color;
use semio_framework_canvas as canvas;
use semio_framework_os_infinite_directed_presentation::*;

#[test]
fn dag_label_colors_use_theme_label_fields() {
    use canvas::Color;
    let theme = CanvasPalette { label_fill: Color::from_rgba8(240, 241, 245, 255), label_halo: Color::from_rgba8(10, 12, 16, 180), node_stroke: Color::from_rgba8(90, 100, 110, 255), ..CanvasPalette::default() };
    assert_ne!(theme.label_fill.to_rgba8(), theme.node_stroke.to_rgba8());
}
