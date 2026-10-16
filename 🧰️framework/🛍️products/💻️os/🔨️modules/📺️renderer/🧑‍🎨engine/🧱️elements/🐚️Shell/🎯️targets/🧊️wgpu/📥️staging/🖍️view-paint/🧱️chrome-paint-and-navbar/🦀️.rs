//! 🖍 `shell-view-paint` staging of `chrome-paint-and-navbar`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

/// ✍️ Immediate-mode Shell chrome text — the non-retained sibling of {@link chrome_text_step}, used by
/// the eleven overlay/dropdown/status painters below that draw a whole scalar in one pass. It was
/// `#[cfg(test)]`-gated, which made every one of those production call sites `E0425` on every target
/// (the reason this crate compiled on neither `wasm32-unknown-unknown` nor natively).
pub(crate) fn chrome_text(target: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = framework_widget_context(target, None, atlas, None, input, theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
    draw_text(&mut ctx, text, x, y, size, color);
}

/// ✍️ Advances one Shell text scalar and at most one pre-admitted glyph. Shell chrome text flows
/// [`RetainedTextFlow::Clip`] — React's chrome labels are all `whitespace-nowrap truncate`
/// (`🎛️chrome-control-presentation/🟦️.ts`, `🏛️ShellHost/🟦️.tsx`'s chip/tab/pill labels), so a label
/// that outgrows its box loses its tail rather than dropping a second line inside a 22px chip.
pub(super) fn chrome_text_step(target: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, width: f32, size: f32, color: Rgba, cursor: &mut RetainedGlyphCursor) -> RetainedGlyphStep {
    paint_retained_glyph_step_flowed(text, Rect::new(x, y - size, width.max(1.0), size), size, color, RetainedTextFlow::Clip, atlas, target, cursor)
}

pub(crate) fn chrome_icon(draw: &mut DrawList, icons: &IconAtlas, icon_id: &str, x: f32, y: f32, size: f32, color: Rgba) {
    if let Some(uv) = icons.icon_uv(icon_id) {
        draw.push_textured([x, y, size, size], uv, color);
    }
}

/// 🪧️ The identity tint for a FULL-COLOUR atlas cell. `UI_SHADER`'s textured branch is
/// `icon.rgb * in.color.rgb` (`🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎨️shaders/🦀️.rs:144`), which is what makes a
/// Lucide glyph wear `theme.text`: every Lucide cell is rasterised as a WHITE tint mask
/// (`rasterize_svg(svg, tint_mask = id != "semio-logo", …)`, `🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs:111`).
///
/// 🩸️ The brand mark is the one cell rasterised in its real colours — navy disc, orange/red/teal
/// bars — and the navbar multiplied it by `theme.text` anyway, which collapsed all four hues into the
/// text colour and painted the BLACK DISC of `📓️audit-visual-parity-puzzle3d.md` §2/§4. React paints
/// `<SemioLogo>` as an untinted SVG, so the identity is what parity means here.
pub(crate) const LOGO_UNTINTED: Rgba = Rgba::new(1.0, 1.0, 1.0, 1.0);

/// 🪧️ React's `size-workbench` on the navbar brand mark (`🏛️ShellHost/🟦️.tsx:10056`), in `--ui-spacing`
/// units: `--size-workbench` is `1.5 × --size-small` and `--size-small` is `5 × --ui-spacing`
/// (`🖱️ui/🎨️styling/🖌️ui/🎨️.css:736-737`), so the mark is 7.5 spacings — TALLER than a 7-spacing chip,
/// which is why it used to be sized off `control_height` and read visibly small.
pub(crate) const SHELL_NAVBAR_LOGO_UI_SPACING: f32 = 7.5;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RetainedChromeGroupStep {
    Pending,
    Complete,
    Fault,
}

pub(crate) fn floating_panel_width(width: f32, body: Rect, theme: &Theme) -> f32 {
    width.clamp(theme.panel_min_width, floating_panel_max_width(body, theme))
}

/// 🧭️ Top-to-bottom order the anchors of one column band themselves in — the row order React's
/// `anchorVertical` names, used to slot an open anchor within its column.
pub(crate) const ANCHOR_COLUMN_ORDER: [PanelAnchor; 8] = [PanelAnchor::TopLeft, PanelAnchor::LeftMiddle, PanelAnchor::BottomLeft, PanelAnchor::TopMiddle, PanelAnchor::BottomMiddle, PanelAnchor::TopRight, PanelAnchor::RightMiddle, PanelAnchor::BottomRight];

pub(super) fn utility_node_label<'a>(label: &'a Option<String>, text: &'a Option<String>, title: &'a Option<String>, id: &'a str) -> &'a str {
    title.as_deref().or(label.as_deref()).or(text.as_deref()).unwrap_or(id)
}

/// 📏️ One horizontal interval in a shell chrome band.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ShellChromeBandSpan {
    pub left: f32,
    pub right: f32,
}

impl ShellChromeBandSpan {
    pub(crate) fn width(self) -> f32 {
        (self.right - self.left).max(0.0)
    }
}

pub(super) fn render_utility_section_divider(draw: &mut DrawList, theme: &Theme, x: f32, btn_y: f32, btn_h: f32) -> f32 {
    draw.push_solid([x + theme.gap_standard * 0.5, btn_y + 4.0, theme.stroke_hairline, btn_h - 8.0], theme.border_normal);
    x + theme.gap_standard
}

pub(super) fn utility_at_path<'utility>(roots: &'utility [UtilityNode], path: &[u16; 64], depth: usize) -> Option<&'utility UtilityNode> {
    let mut siblings = roots;
    for level in 0..=depth {
        let node = siblings.get(path[level] as usize)?;
        if level == depth {
            return Some(node);
        }
        let UtilityNode::Collection { children, .. } = node else { return None };
        siblings = children;
    }
    None
}

//#region ActionPanelAndUtilities
//#region 🔀️ChromeParity
/// 📐️ A navbar control's authored responsive width contract. The bounds are expressed in root-em
/// units and resolved in logical pixels before both centred-band reservation and paint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellNavbarWidthPolicy {
    RootRemClamp { minimum_rem: u16, maximum_rem: u16 },
}

/// 🗺️ What the navbar centre reads with no session open. React's `navbarItems` answers `[]` in that
/// case and paints no cluster at all; this renderer's chrome walk has no "paint nothing" arm, so it
/// keeps the product name it has always shown.
pub(crate) const SHELL_NAVBAR_SESSIONLESS_TITLE: &str = "semio · os";

/// 👁️✏️ React's `surfaceRoleChipText` (`🛠️ShellHelpers/🟦️.tsx:2422`) — the frozen `{en, de}` pair of the
/// navbar's read-only role badge, term for term.
pub(crate) fn shell_surface_role_chip_text(role: semio_framework::manifest::AppRole, is_de: bool) -> &'static str {
    match (role, is_de) {
        (semio_framework::manifest::AppRole::Viewer, false) => "Viewer",
        (semio_framework::manifest::AppRole::Viewer, true) => "Betrachter",
        (semio_framework::manifest::AppRole::Editor, _) => "Editor",
    }
}

/// ⏳️🖼️ What the World3d compute-status pill says on ONE surface: the phase in the reader's own
/// language, the producer's own `unitsDone/unitsTotal` count, and the fraction its bar fills to.
/// `ratio` is `None` for an indeterminate producer (one reporting no `unitsTotal`), which paints a
/// bare track rather than a lie about how far along the work is.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct World3dStatusPill {
    pub surface_id: String,
    pub phase: String,
    pub phase_text: String,
    pub progress_text: Option<String>,
    pub ratio: Option<f32>,
    pub computing: bool,
}

impl World3dStatusPill {
    /// 🏷️ The one text run the pill paints — React renders the same two pieces as two adjacent
    /// spans, this target has one glyph run per label, so they join with a separator.
    pub fn label(&self) -> String {
        match self.progress_text.as_deref() {
            Some(progress) => format!("{} · {progress}", self.phase_text),
            None => self.phase_text.clone(),
        }
    }
}

/// ⏳️ Whether a status is worth annotating a viewport with at all — byte-for-byte React's own
/// `WorldComputeStatusPane` gate (`if (!computing && !cancellable && phase !== "cancelled") return
/// null`): a settled producer paints nothing, so an idle preview is never covered by chrome.
pub(crate) fn world3d_status_is_visible(status: &World3dComputeStatus) -> bool {
    status.computing || status.cancellable || status.phase == "cancelled"
}

/// 🖼️ Whether a surface is big enough to carry an overlay row at all — a collapsed dock pane paints
/// no chrome over its whole body.
pub(crate) fn surface_fits_overlay(theme: &Theme, bounds: Rect) -> bool {
    bounds.w >= theme.control_height * 4.0 && bounds.h >= theme.control_height * 2.0
}

/// 🛟️ Which axis an in-surface affordance is free to yield on — the Rust twin of React's `SafeAreaYield`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SafeAreaYield {
    Inline,
    Block,
    Either,
}

/// 🛟️ How far an affordance must move off its anchor's own two edges to clear the chrome panels painted
/// over it — at most one axis is ever non-zero. The Rust twin of React's `ChromePanelSafeArea`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ChromePanelSafeArea {
    pub inline: f32,
    pub block: f32,
}

pub(super) fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.w && a.x + a.w > b.x && a.y < b.y + b.h && a.y + a.h > b.y
}

/// 🧭️ Sweep forbidden displacement intervals while preserving the axis gap and actual host bounds.
pub(super) fn chrome_panel_axis_reserve(affordance: Rect, host: Rect, panels: &[Rect], inline: bool, direction: f32, gap: f32) -> f32 {
    if direction == 0.0 { return 0.0; }
    let axes = |rect: Rect| if inline { [rect.x, rect.x + rect.w, rect.y, rect.y + rect.h] } else { [rect.y, rect.y + rect.h, rect.x, rect.x + rect.w] };
    let a = axes(affordance);
    let h = axes(host);
    if a[2] < h[2] || a[3] > h[3] { return 0.0; }
    let mut intervals: Vec<(f32, f32)> = panels.iter().filter_map(|panel| {
        let p = axes(*panel);
        if a[2] >= p[3] || a[3] <= p[2] { return None; }
        Some(if direction > 0.0 { (p[0] - a[1] - gap, p[1] - a[0] + gap) } else { (a[0] - p[1] - gap, a[1] - p[0] + gap) })
    }).collect();
    intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut offset = 0.0;
    for (start, end) in intervals {
        if start >= offset { break; }
        if end > offset { offset = end.ceil(); }
    }
    if offset > 0.0 && a[0] + direction * offset >= h[0] && a[1] + direction * offset <= h[1] { offset } else { 0.0 }
}

/// 🛟️ The box a live surface's overlay row claims at its own top-left corner — the smallest row
/// `surface_fits_overlay` ever admits (`surfaceControlMinimum` in `🛑️surface-controls/🔣️.json`), so the
/// claim is stated before a single label has been measured. A chrome panel covering THAT box reserves.
pub(crate) fn surface_overlay_row_box(theme: &Theme, bounds: Rect) -> Rect {
    Rect::new(bounds.x + theme.gap_standard, bounds.y + theme.gap_standard, theme.control_height * 4.0, theme.control_height)
}
//#endregion 🔀️ChromeParity
//#endregion ActionPanelAndUtilities

//#region ActionPanelAndUtilities
//#region 🪟️WindowPaneChrome
/// ⏱️ The ceiling one unfolded projection body may spend inside the chrome walk — its own paint opportunities, not the whole
/// walk's, and only its own WORK (`document_opportunity_remains`): polls of the layout worker pool and steps of other surfaces'
/// layouts are not the pane's. The pane's content is React's fixed taxonomy (18 retained nodes), and its own work is priced per
/// grant: two layout passes (the pane re-solves once it hugs its measured content) of ~240 own steps each — every worker job
/// step costs two opportunities on this thread (its outcome is taken, then applied), plus node collection and session close —
/// and ~800 paint grants of one glyph or node phase each: 1 405 in WG11's overlay build 7, where a 1 024 ceiling faulted the
/// converging pane at paint progress 418. Nearly three times that fits; a body that reaches it has stopped converging, which is
/// a fault, not a slow frame.
pub(crate) const WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES: usize = 4096;

/// 📐️ The box one pane overlay chip occupies inside a window body — React's `anchorPositionStyle`
/// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6864`) on this target's own token: a corner anchor insets
/// `--spacing-single` (`PANEL_INSET_UI_SPACING`, the same 1 × ui-spacing `theme.panel_inset` carries)
/// on BOTH its edges, a middle anchor centres on that axis. One chip per anchor, so a band is one box.
pub(crate) fn window_pane_chip_rect(theme: &Theme, body: Rect, anchor: PanelAnchor, width: f32) -> Rect {
    let x = match anchor.horizontal() {
        "left" => body.x + theme.panel_inset,
        "right" => body.x + body.w - theme.panel_inset - width,
        _ => body.x + (body.w - width) * 0.5,
    };
    let y = match anchor.vertical() {
        "top" => body.y + theme.panel_inset,
        "bottom" => body.y + body.h - theme.panel_inset - theme.control_height,
        _ => body.y + (body.h - theme.control_height) * 0.5,
    };
    Rect::new(x, y, width, theme.control_height)
}
//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities
