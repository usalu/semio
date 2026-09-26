//! 📐️ LAW: every chrome geometry the wgpu shell produces — navbar band, body rect, dock tab rects,
//! split resize-handle hit rects — is a function of the LOGICAL (CSS) surface extent and the theme
//! alone. The surface scale factor reaches the GPU surface size, the projection divisor and the
//! glyph/icon atlas raster, and NOTHING else.
//!
//! The bug this pins: the content root used to be sized `css * dpr` (physical pixels) while every
//! chrome constant (`chrome_px`, `navbar_height`, `footer_height`, `control_height`, the
//! resize-handle hit widths) stayed a logical/CSS value taken straight from the same generated
//! tokens React's DOM consumes. At `scale_factor == 1.0` that is invisible; on any HiDPI display
//! every fixed-size chrome element rendered and hit-tested at roughly `1/dpr` of its intended size
//! against a full-size canvas — "ui elements are placed totally different, the window system doesn't
//! work". Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g.
//!
//! Each law drives the REAL ingress seam: `WindowMetrics::logical_size` (what `handle_metrics` hands
//! `RuntimeApply::Resize`) feeds `ShellState::screen_w`/`screen_h` exactly as
//! `AppInteractionState::resize` does, and the assertions compare production chrome functions.

use super::*;
use crate::dock::DockNode;
use ui_host::WindowMetrics;
use ui_render::PhysicalSize;

/// 📏️ One CSS viewport, observed at four densities: 1× (any non-Retina panel), 1.5× (a common
/// Windows/Linux fractional scale), 2× (this environment's own macOS Retina host) and 3× (a phone).
const SCALE_FACTORS: [f32; 4] = [1.0, 1.5, 2.0, 3.0];
const LOGICAL_WIDTH: f32 = 1280.0;
const LOGICAL_HEIGHT: f32 = 800.0;

fn metrics_for(scale_factor: f32) -> WindowMetrics {
    WindowMetrics { physical: PhysicalSize::new((LOGICAL_WIDTH * scale_factor).round() as u32, (LOGICAL_HEIGHT * scale_factor).round() as u32), scale_factor }
}

/// 🪟️ A two-pane row with a tabbed right stack — the smallest tree that has both corner tab bars
/// and one split resize handle.
fn split_dock() -> DockState {
    let mut dock = DockState::default();
    dock.root =
        DockNode::Row(vec![(DockNode::Stack { windows: vec![DockStackTab::new("flow")], active: "flow".into() }, 0.6), (DockNode::Stack { windows: vec![DockStackTab::new("preview"), DockStackTab::new("details")], active: "preview".into() }, 0.4)]);
    dock.active_window_id = Some("flow".into());
    dock
}

fn dock_labels() -> HashMap<String, String> {
    HashMap::from([("flow".into(), "Flow".into()), ("preview".into(), "Preview".into()), ("details".into(), "Details".into())])
}

/// 📐️ Every rect this frame's chrome pass would place, rounded to a printable form so a mismatch
/// names the offending geometry instead of a bare `false`.
#[derive(Debug, PartialEq)]
struct ChromeGeometry {
    logical_extent: (f32, f32),
    navbar_height: f32,
    footer_height: f32,
    control_height: f32,
    body_rect: [f32; 4],
    tab_bars: Vec<[f32; 4]>,
    tab_chip_widths: Vec<Vec<f32>>,
    resize_handles: Vec<[f32; 4]>,
}

/// 🎬️ Runs the production chrome pipeline for one scale factor: metrics → logical extent → shell
/// content root → `body_rect` → dock tab-bar rects and registered split hits.
fn chrome_geometry(scale_factor: f32) -> ChromeGeometry {
    let metrics = metrics_for(scale_factor);
    let (logical_width, logical_height) = metrics.logical_size();
    let mut shell = ShellState::new(vec![], "dpi-law".into());
    shell.screen_w = logical_width.max(1.0);
    shell.screen_h = logical_height.max(1.0);

    let theme = Theme::default();
    let body = shell.body_rect(&theme);
    let dock = split_dock();
    let labels = dock_labels();
    let icon_ids = HashMap::new();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();

    let tab_bars = dock.stack_corner_tab_bar_rects(body, &theme, &mut atlas, &labels);
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
    dock.register_resize_hits(&mut ctx, body);
    let resize_handles = input.staged_hits().iter().filter(|hit| hit.kind == HitKind::DockSplit).map(|hit| [hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h]).collect();

    ChromeGeometry {
        logical_extent: (logical_width, logical_height),
        navbar_height: theme.navbar_height,
        footer_height: theme.footer_height,
        control_height: theme.control_height,
        body_rect: [body.x, body.y, body.w, body.h],
        tab_bars: tab_bars.iter().map(|(_, _, rect, _)| [rect.x, rect.y, rect.w, rect.h]).collect(),
        tab_chip_widths: tab_bars.iter().map(|(_, _, _, widths)| widths.clone()).collect(),
        resize_handles,
    }
}

/// 📐️ The metrics seam itself: one CSS viewport measured at four densities always answers the same
/// logical extent. This is the value `OsHost::handle_metrics` forwards as `RuntimeApply::Resize`.
#[test]
fn window_metrics_answer_one_logical_extent_at_every_density() {
    for scale_factor in SCALE_FACTORS {
        let metrics = metrics_for(scale_factor);
        let (width, height) = metrics.logical_size();
        assert!((width - LOGICAL_WIDTH).abs() <= 1.0, "scale {scale_factor}: logical width {width} is not the CSS viewport {LOGICAL_WIDTH}");
        assert!((height - LOGICAL_HEIGHT).abs() <= 1.0, "scale {scale_factor}: logical height {height} is not the CSS viewport {LOGICAL_HEIGHT}");
        assert!(metrics.physical.width as f32 >= LOGICAL_WIDTH, "scale {scale_factor}: the PHYSICAL extent is what grows with density");
    }
}

/// 📐️ The law. One CSS viewport at four densities produces byte-identical navbar band, body rect,
/// dock tab-bar rects, per-tab chip widths and split resize-handle hit rects.
#[test]
fn chrome_layout_is_independent_of_the_scale_factor() {
    let baseline = chrome_geometry(SCALE_FACTORS[0]);
    assert!(baseline.navbar_height > 0.0 && baseline.footer_height > 0.0 && baseline.control_height > 0.0, "the theme must carry real chrome constants");
    assert!(!baseline.tab_bars.is_empty(), "the sample dock must place tab bars");
    assert!(!baseline.resize_handles.is_empty(), "the sample dock must register a split resize handle");
    assert!(baseline.body_rect[1] >= baseline.navbar_height, "the body starts under the navbar band");
    assert!(baseline.body_rect[2] == LOGICAL_WIDTH, "the body spans the CSS viewport, not the device one");
    for scale_factor in SCALE_FACTORS.into_iter().skip(1) {
        assert_eq!(chrome_geometry(scale_factor), baseline, "scale {scale_factor} moved chrome geometry that only the CSS viewport may move");
    }
}

/// ✂️ The resize-handle HIT width is a logical constant: a 2× display must not make the grab target
/// half as wide in CSS terms (the pre-fix symptom — handles that were nearly impossible to grab).
#[test]
fn resize_handle_hit_width_stays_a_logical_constant() {
    for scale_factor in SCALE_FACTORS {
        let geometry = chrome_geometry(scale_factor);
        for handle in &geometry.resize_handles {
            let thickness = handle[2].min(handle[3]);
            assert!(thickness >= 8.0, "scale {scale_factor}: a {thickness} logical-px grab target is not pointable");
        }
    }
}
