//! 🖌️ Product renderer paints obey the shared general palette rule.

use std::path::PathBuf;

#[path = "../../../../../../../🔨️modules/🖱️ui/🧪️testing/🎨️paint-policy/🦀️.rs"]
mod paint_policy;

const SCAN_ROOTS: &[&str] = &["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer"];

const ALLOWLIST: &[(&str, &str, &str)] = &[
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

#[test]
fn no_wgpu_target_paints_a_hand_written_colour_literal() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for _ in 0..10 { root = root.parent().expect("renderer manifest has ten ancestors to the repository").to_path_buf(); }
    let (scanned, violations) = paint_policy::scan(&root, SCAN_ROOTS, ALLOWLIST);
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
