//! ⚖️ Law: every colour and metric the wgpu targets paint comes from the generated `ui_styling`
//! token crate — the same `🎨️styling/🔣️.json` that emits React's `🎨️palette/🎨️.css`. A hand-written
//! channel literal in a wgpu target is the only way the two renderers can silently drift apart, so
//! it fails here instead of at a screenshot diff.

use super::*;
use std::fs;
use std::path::{Path, PathBuf};

/// 📂️ Repo root, five levels above `semio-framework-ui`'s manifest
/// (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust`).
fn repo_root() -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for _ in 0..5 {
        root = root.parent().expect("manifest dir has five ancestors up to the repo root").to_path_buf();
    }
    root
}

/// 🔎️ Source trees whose `🧊️wgpu` files this law owns: the ui target itself (chrome/widgets/shell/
/// draw/theme), the per-element wgpu targets, and the os renderer's Shell/Dock/Scenes wgpu targets.
const SCAN_ROOTS: &[&str] = &["🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu", "🧰️framework/🔨️modules/🖱️ui/🧱️elements", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer"];

/// 🎨️ The two constructors that turn raw channels into paint. Every other colour must be a
/// `Rgba::from_token` lift of a generated palette entry, or derived from an existing `Rgba`.
const COLOR_CONSTRUCTORS: &[&str] = &["Rgba::new(", "Rgba::from_srgb8("];

/// 🪪️ Renderer-internal paints that are deliberately not tokens, each with the reason it stays a
/// literal. Anything not listed here must come from `ui_styling`.
const ALLOWLIST: &[(&str, &str, &str)] = &[
    (
        "🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs",
        "let white = Rgba::new(1.0, 1.0, 1.0, 1.0);",
        "🎭️ Scissor-mask identity in a `#[cfg(test)]` helper — opaque white is the stencil's \"keep\" value, not a colour anybody sees.",
    ),
    (
        "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "const CANVAS2D_SELECTION_RING",
        "🟡️ Verbatim port of React `canvas-2d-host.tsx`'s own `rgba(251, 191, 36, …)` literal; the drift lives on the React side, so tokenising only this half would create the divergence it prevents.",
    ),
    (
        "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "const CANVAS2D_SELECTION_GLOW",
        "🟡️ Same literal as `CANVAS2D_SELECTION_RING`, at the glow alpha.",
    ),
    (
        "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "fn canvas_color_channels",
        "🧮️ Per-channel default of a pure `[f64]` → `Rgba` payload decoder: what a malformed scene packet gets, not theme paint. The decoder takes no `Theme`; plumbing one in is a Scenes packet, not a token one.",
    ),
    (
        "🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
        "fn canvas_gradient_color_at",
        "🧮️ Empty-stop-list fallback of the same pure payload decoder — see `canvas_color_channels`.",
    ),
    (
        "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs",
        "const LOGO_UNTINTED",
        "⬜️ Identity TINT MULTIPLIER, not a paint: the brand mark is the one icon cell rasterised in its own four hues (`rasterize_svg(svg, tint_mask = id != \"semio-logo\")`), and React paints `<SemioLogo>` untinted. Any token here would multiply the mark down to a single hue — the black disc this const exists to prevent.",
    ),
];

/// 🔢️ True when the three COLOUR channels are plain numeric literals — i.e. the call writes a hue by
/// hand rather than reading one. Alpha is excluded on purpose: `…, 0.9 * opacity)` is still a
/// hand-written colour. `Rgba::new(tip.color.r, …)` and `Rgba::from_srgb8(r, g, b, 255)` (a parsed
/// user hex) carry derived channels and pass.
fn channels_are_hand_written(arguments: &str) -> bool {
    let channels: Vec<&str> = arguments.split(',').take(3).collect();
    if channels.len() < 3 {
        return false;
    }
    channels.iter().all(|argument| {
        let argument = argument.trim().trim_end_matches("_f32").trim_end_matches("_u8");
        !argument.is_empty() && argument.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '_' || c == '-')
    })
}

/// 🔍️ Argument text of the constructor call starting at `open` (the index just past its `(`), or
/// `None` when the call spans past the end of the line.
fn call_arguments(line: &str, open: usize) -> Option<&str> {
    let rest = &line[open..];
    let mut depth = 1_i32;
    for (index, character) in rest.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&rest[..index]);
                }
            }
            _ => {}
        }
    }
    None
}

fn rust_sources(directory: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name == "node_modules" || name == "target" || name.starts_with("🗑️") || name.starts_with('.') {
                continue;
            }
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

/// 🧭️ Nearest preceding `const`/`fn`/`static` item name, so an allowlist entry can name a symbol
/// instead of a line number that every neighbouring edit invalidates.
fn enclosing_item<'a>(lines: &'a [&'a str], index: usize) -> &'a str {
    for line in lines[..=index].iter().rev() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") || trimmed.starts_with("const ") || trimmed.starts_with("pub const ") || trimmed.starts_with("pub(crate) fn ") || trimmed.starts_with("static ") {
            return trimmed;
        }
    }
    ""
}

fn is_allowlisted(relative: &str, line: &str, item: &str) -> bool {
    ALLOWLIST.iter().any(|(path, marker, _)| relative.ends_with(path) && (line.contains(marker) || item.starts_with(marker)))
}

/// 🧪️ Test case directories build expected paints by hand on purpose — that is the oracle,
/// not production paint.
#[test]
fn no_wgpu_target_paints_a_hand_written_colour_literal() {
    let root = repo_root();
    let mut violations: Vec<String> = Vec::new();
    let mut scanned = 0_usize;
    for scan_root in SCAN_ROOTS {
        let absolute = root.join(scan_root);
        assert!(absolute.is_dir(), "scan root {scan_root} is missing — the law would pass vacuously");
        let mut files = Vec::new();
        rust_sources(&absolute, &mut files);
        for file in files {
            let relative = file.strip_prefix(&root).unwrap_or(&file).to_string_lossy().to_string();
            if !relative.contains("🧊️wgpu") {
                continue;
            }
            if relative.contains("🧪️tests") {
                continue;
            }
            scanned += 1;
            let text = fs::read_to_string(&file).expect("wgpu source reads as utf8");
            let lines: Vec<&str> = text.lines().collect();
            for (index, line) in lines.iter().enumerate() {
                if line.trim_start().starts_with("//") {
                    continue;
                }
                for constructor in COLOR_CONSTRUCTORS {
                    let Some(at) = line.find(constructor) else { continue };
                    let Some(arguments) = call_arguments(line, at + constructor.len()) else { continue };
                    if !channels_are_hand_written(arguments) {
                        continue;
                    }
                    let item = enclosing_item(&lines, index);
                    if is_allowlisted(&relative, line, item) {
                        continue;
                    }
                    violations.push(format!("{relative}:{}: {}", index + 1, line.trim()));
                }
            }
        }
    }
    assert!(scanned > 0, "no wgpu sources were scanned — the walk is broken");
    assert!(
        violations.is_empty(),
        "wgpu targets must read every colour from the generated ui_styling tokens (🎨️styling/🔣️.json, the same source as React's 🎨️palette/🎨️.css).\n\
         Add the paint to 🔣️.json and read it with `Rgba::from_token`, or — only for a genuinely renderer-internal paint — extend this law's ALLOWLIST with its reason.\n\
         {} violation(s):\n{}",
        violations.len(),
        violations.join("\n")
    );
}

/// 🎨️ Reads one `--color-*` declaration out of the generated palette CSS React consumes.
fn generated_palette_hex(name: &str) -> String {
    let css = fs::read_to_string(repo_root().join("🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css")).expect("generated palette css is on disk");
    let needle = format!("--color-{name}:");
    let line = css.lines().find(|line| line.trim_start().starts_with(&needle)).unwrap_or_else(|| panic!("generated palette css declares --color-{name}"));
    line.split(':').nth(1).expect("declaration has a value").trim().trim_end_matches(';').to_string()
}

fn hex_of(color: Rgba) -> String {
    let [r, g, b, _] = ui_styling::color::linear_to_rgba8(color.r, color.g, color.b, color.a);
    format!("#{r:02x}{g:02x}{b:02x}")
}

#[test]
fn outcome_paints_decode_to_the_same_hex_react_paints() {
    for (appearance, theme) in [("light", Theme::light()), ("dark", Theme::dark())] {
        for (field, value, token) in [("error", theme.error, "danger"), ("success", theme.success, "success"), ("warning", theme.warning, "warning"), ("progress", theme.progress, "secondary")] {
            assert_eq!(hex_of(value), generated_palette_hex(token), "{appearance} theme.{field} must decode to --color-{token}");
        }
    }
}

#[test]
fn semantic_panel_decodes_to_reacts_live_css_alias_and_custom_theme_input() {
    assert_eq!(hex_of(Theme::light().panel), generated_palette_hex("light-5-7"));
    assert_eq!(hex_of(Theme::dark().panel), generated_palette_hex("dark-7-9"));
    let mono = Theme::mono(false);
    assert_eq!(mono.panel, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.panel), "a premade theme owns its semantic panel token");
    assert_ne!(mono.panel, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.level_panel), "the semantic --panel alias remains distinct from the hierarchy level ramp");
}

#[test]
fn every_theme_metric_is_a_multiple_of_the_shared_ui_spacing() {
    let theme = Theme::dark();
    let step = ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX as f32;
    for (field, value) in [
        ("navbar_height", theme.navbar_height),
        ("footer_height", theme.footer_height),
        ("control_height", theme.control_height),
        ("control_height_small", theme.control_height_small),
        ("panel_header_height", theme.panel_header_height),
        ("tree_row_height", theme.tree_row_height),
        ("tree_indent_per_level", theme.tree_indent_per_level),
        ("tree_toggle_width", theme.tree_toggle_width),
        ("gap_standard", theme.gap_standard),
        ("padding_standard", theme.padding_standard),
        ("panel_inset", theme.panel_inset),
        ("panel_min_width", theme.panel_min_width),
        ("panel_max_width", theme.panel_max_width),
    ] {
        let multiple = value / step;
        assert!((multiple - multiple.round()).abs() < 1e-3 || (multiple * 1000.0).fract().abs() < 1.0, "theme.{field} = {value} is not a `--ui-spacing` multiple ({multiple})");
    }
    assert_eq!(theme.control_height_small, step * ui_styling::metrics::chrome::CONTROL_HEIGHT_SMALL_UI_SPACING as f32);
    assert_eq!(crate::wgpu::chrome::ICON_TINY, step * ui_styling::metrics::chrome::ICON_INLINE_UI_SPACING as f32);
}

#[test]
fn veil_is_the_dialog_surface_at_the_shared_veil_alpha() {
    for theme in [Theme::light(), Theme::dark()] {
        let veil = theme.veil(Level::Dialog);
        let surface = theme.surface(Level::Dialog);
        assert_eq!((veil.r, veil.g, veil.b), (surface.r, surface.g, surface.b));
        assert_eq!(veil.a, levels::VEIL_ALPHA as f32);
    }
    assert_eq!(Theme::veil_blur_px(), levels::VEIL_BLUR_PX as f32);
}

#[test]
fn spatial_axis_paints_are_the_brand_tokens() {
    for (axis, token) in [(0_u8, ui_styling::colors::PRIMARY), (1, ui_styling::colors::SECONDARY), (2, ui_styling::colors::TERTIARY)] {
        let paint = crate::wgpu::draw_types::gizmo::spatial_axis_rgba(axis, 1.0);
        assert_eq!(hex_of(paint), hex_of(Rgba::from_token(&token)), "axis {axis} must paint its brand token");
    }
}

//#region ⚫️MonoPremade
// ⚫️ W2k: the "mono" premade used to be 20 hand-written `Rgba::from_srgb8` literals in the os wgpu
// Shell, because mono's `chrome` group declared seven keys the default theme lacked and so could not
// share the generated `ChromePalette`. The key sets are reconciled and mono is now projected.

#[test]
fn the_mono_premade_is_a_real_theme_off_its_own_generated_palettes() {
    let light = Theme::mono(false);
    let dark = Theme::mono(true);
    assert_ne!(light.background, dark.background, "mono resolves both appearances");
    assert_ne!(light.background, Theme::light().background, "mono is not an alias of semio");
    assert_ne!(dark.background, Theme::dark().background);
    assert_eq!(light.background, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.base), "mono's floor is its OWN chrome.base, not a hand-resolved canvas");
    assert_eq!(dark.background, Rgba::from_token(&ui_styling::CHROME_MONO_DARK.base));
}

#[test]
fn the_mono_premade_shares_every_derived_surface_rule_with_the_default_theme() {
    let mono = Theme::mono(false);
    assert_eq!(mono.panel, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.panel), "panel follows the premade theme's semantic alias");
    assert_eq!(mono.navbar, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.level_window));
    assert_eq!(mono.temporary, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.level_menu));
    assert_eq!(mono.text_muted, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.muted_foreground), "the hand-port read hover_interactive_fill here — a wrong source");
    assert_eq!(mono.text_element, Rgba::from_token(&ui_styling::CHROME_MONO_LIGHT.border_element));
    let semio = Theme::light();
    assert_eq!(mono.control_height, semio.control_height, "metrics are shared: a premade only recolors");
    assert_eq!(mono.font_size_body, semio.font_size_body);
    assert_eq!(mono.border_radius, semio.border_radius);
}

#[test]
fn every_appearance_group_carries_a_mono_twin_with_the_default_theme_s_own_shape() {
    assert_ne!(ui_styling::OUTCOME_MONO_LIGHT.error, ui_styling::OUTCOME_LIGHT.error, "mono recolors the outcome palette off its own grayscale tokens");
    assert_eq!(ui_styling::DIAGRAM_MONO_LIGHT.shape_outline.len(), 4, "a premade shares the generated struct, so even the four keys mono used to omit exist");
    assert_ne!(ui_styling::BOARD_MONO_LIGHT.node_stroke_computing, ui_styling::BOARD_LIGHT.node_stroke_computing, "the four board keys mono used to omit now resolve off mono's own palette");
    let muted = Theme::mono(false).muted;
    assert_ne!(muted, Theme::mono(false).separator, "`--muted` is its own token, not the separator stroke");
}
//#endregion ⚫️MonoPremade

//#region 🏠️ShellFloor
// 🏠️ W2k: React's `shellFloorPaints` (`🔨️modules/🏠️shell-floor-presentation/🟦️.ts:14`) — a base-level
// floor nested in a scope that is already base-and-painting drops to `bg-transparent`. The wgpu Shell
// painted the base colour twice per frame (frame setup + the main window) with no such predicate.

#[test]
fn a_base_floor_suppresses_its_fill_only_inside_an_already_painted_base_scope() {
    assert!(shell_floor_paints(None), "no enclosing scope at all still paints");
    assert!(!shell_floor_paints(Some(SurfaceScope { level: Level::Base, fill: SurfaceFill::Surface })), "base-and-painting is the one suppressed case");
    assert!(!shell_floor_paints(Some(SurfaceScope { level: Level::Base, fill: SurfaceFill::Glass })));
    assert!(!shell_floor_paints(Some(SurfaceScope { level: Level::Base, fill: SurfaceFill::Veil })));
    assert!(shell_floor_paints(Some(SurfaceScope { level: Level::Base, fill: SurfaceFill::None })), "a base scope that paints nothing leaves the floor to its descendant");
    for level in [Level::Window, Level::Pane, Level::Panel, Level::Dialog, Level::Menu] {
        assert!(shell_floor_paints(Some(SurfaceScope { level, fill: SurfaceFill::Surface })), "a deeper level is a different colour, so the floor still paints");
    }
}
//#endregion 🏠️ShellFloor
