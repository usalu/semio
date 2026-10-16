//! 🍱️ `shell-view-chrome` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

pub(crate) fn chrome_text_complete_step(target: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, width: f32, size: f32, color: Rgba, cursor: &mut RetainedGlyphCursor) -> Result<bool, ()> {
    match chrome_text_step(target, atlas, text, x, y, width, size, color, cursor) {
        RetainedGlyphStep::Pending => Ok(false),
        RetainedGlyphStep::Complete => {
            cursor.reset();
            Ok(true)
        }
        RetainedGlyphStep::Fault => Err(()),
    }
}

/// 🧱️ Emits at most one retained chrome item scalar, glyph, hit, or border per grant.
pub(crate) fn render_retained_chrome_group_item_step(
    group_phase: &mut u8,
    glyph: &mut RetainedGlyphCursor,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    rect: Rect,
    item: &ChromeGroupItem<'_>,
    register_hit: bool,
) -> RetainedChromeGroupStep {
    render_retained_chrome_group_item_step_with_trailing(group_phase, glyph, draw, atlas, icons, input, theme, rect, item, register_hit, CHROME_ICON_TINY, None)
}

#[allow(clippy::too_many_arguments, reason = "the retained painter's existing render context plus one optional trailing icon")]
fn render_retained_chrome_group_item_step_with_trailing(
    group_phase: &mut u8,
    glyph: &mut RetainedGlyphCursor,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    rect: Rect,
    item: &ChromeGroupItem<'_>,
    register_hit: bool,
    leading_icon_size: f32,
    trailing_icon: Option<(&str, f32)>,
) -> RetainedChromeGroupStep {
    let hovered = !item.disabled && rect.contains(input.pointer_x, input.pointer_y);
    let color = if item.disabled { theme.text_muted } else { chrome_item_text(theme, item.active, hovered) };
    match *group_phase {
        0 => {
            let background = if item.disabled { theme.overlay_shadow } else { chrome_item_bg(theme, item.active, hovered) };
            if background.a > 0.0 {
                draw.push_solid([rect.x, rect.y, rect.w, rect.h], background);
            }
            *group_phase = 1;
        }
        1 => {
            if let Some(icon_id) = item.icon_id {
                chrome_icon(draw, icons, icon_id, rect.x + theme.padding_standard, rect.y + (rect.h - leading_icon_size) * 0.5, leading_icon_size, color);
            }
            *group_phase = 2;
        }
        2 => {
            if let Some(label) = item.label {
                if label.len() > ui_wgpu::wgpu::RETAINED_NODE_TEXT_MAX_BYTES {
                    *group_phase = 0;
                    glyph.reset();
                    return RetainedChromeGroupStep::Fault;
                }
                let icon_w = item.icon_id.map(|_| leading_icon_size + theme.gap_standard).unwrap_or(0.0);
                let x = rect.x + theme.padding_standard + icon_w;
                let trailing = trailing_icon.map_or(0.0, |(_, size)| theme.gap_standard + size);
                let width = (rect.x + rect.w - theme.padding_standard - trailing - x).max(1.0);
                match chrome_text_complete_step(draw, atlas, label, x, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0, width, theme.font_size_small, color, glyph) {
                    Ok(false) => return RetainedChromeGroupStep::Pending,
                    Ok(true) => {}
                    Err(()) => {
                        *group_phase = 0;
                        glyph.reset();
                        return RetainedChromeGroupStep::Fault;
                    }
                }
            }
            *group_phase = 3;
        }
        3 => {
            draw.push_solid([rect.x, rect.y, rect.w, theme.stroke_hairline], theme.border_normal);
            *group_phase = 4;
        }
        4 => {
            draw.push_solid([rect.x, rect.y + rect.h - theme.stroke_hairline, rect.w, theme.stroke_hairline], theme.border_normal);
            *group_phase = 5;
        }
        5 => {
            draw.push_solid([rect.x, rect.y, theme.stroke_hairline, rect.h], theme.border_normal);
            *group_phase = 6;
        }
        6 => {
            draw.push_solid([rect.x + rect.w - theme.stroke_hairline, rect.y, theme.stroke_hairline, rect.h], theme.border_normal);
            *group_phase = 7;
        }
        7 => {
            if register_hit && !item.disabled {
                note_chrome_group_item(item);
                input.register_hit(HitTarget { rect, event: None, control_id: Some(item.control_id.into()), kind: item.kind.clone(), drag_axis: None, drag_data: None });
            }
            *group_phase = if trailing_icon.is_some() { 8 } else { 9 };
        }
        8 => {
            if let Some((icon_id, size)) = trailing_icon {
                chrome_icon(draw, icons, icon_id, rect.x + rect.w - theme.padding_standard - size, rect.y + (rect.h - size) * 0.5, size, color);
            }
            *group_phase = 9;
        }
        _ => {
            *group_phase = 0;
            glyph.reset();
            return RetainedChromeGroupStep::Complete;
        }
    }
    RetainedChromeGroupStep::Pending
}

#[allow(clippy::too_many_arguments, reason = "the retained painter's existing render context")]
pub(crate) fn render_retained_panel_chrome_item_step(
    group_phase: &mut u8,
    glyph: &mut RetainedGlyphCursor,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    rect: Rect,
    item: &ChromeGroupItem<'_>,
    register_hit: bool,
) -> RetainedChromeGroupStep {
    render_retained_chrome_group_item_step_with_trailing(group_phase, glyph, draw, atlas, icons, input, theme, rect, item, register_hit, PANEL_CHROME_ICON_TINY, Some(("grip-vertical", PANEL_CHROME_GRIP_SIZE)))
}

/// 🧭️ One anchor's floating-panel box inside the body band, the Rust twin of React's
/// `anchorPositionStyle` (`🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`): a left/right column insets by
/// `--spacing-single` (`theme.panel_inset`) from that edge, a middle column centres on the body, a
/// top/bottom row insets from that edge and the middle row centres — with React's
/// `calc(100% - spacing*2)` clamp applied to both axes.
///
/// 📐️ The one honest divergence: React's panel HUGS ITS CONTENT up to that clamp (`maxHeight`, never a
/// fixed `bottom`), which needs a measured retained-document height this renderer has no read of yet.
/// A column's visible anchors therefore share its height in equal bands instead — identical to the old
/// full-height single panel whenever a column carries exactly one open anchor, and never overlapping
/// when it carries two. See `📓️w1h-panel-anchors-and-command-dock.md`'s remaining gaps.
///
/// 📐️ Content hug: a measured document takes exactly its own height, clamped to React's
/// `calc(100% - spacing*2)` — and, when a column carries more than one open anchor, to that
/// anchor's share of the column so two hugging panels can never overlap. No measurement yet (the
/// surface's first frame) keeps the full band, which is what this renderer did before.
///
/// 📐️ One BONDED vertical edge, never a `top`+`bottom` pair (`anchorPositionStyle`): a top anchor
/// grows down from its own inset, a bottom anchor grows up from the body's bottom inset, and a
/// middle anchor centres on its band.
///
/// 📐️ The slack is computed BEFORE it is added to the origin: `slot_top + band - height` loses a
/// float step on a full-band panel (693.6 added then subtracted), which is visible as a one-thousandth
/// drift off the declared inset.
pub(crate) fn anchor_panel_rect(anchor: PanelAnchor, size: f32, content_h: Option<f32>, column_slots: usize, column_index: usize, body: Rect, theme: &Theme) -> Rect {
    let inset = theme.panel_inset;
    let width = floating_panel_width(size, body, theme);
    let available_h = (body.h - inset * 2.0).max(0.0);
    let slots = column_slots.max(1) as f32;
    let gap = if column_slots > 1 { theme.gap_standard } else { 0.0 };
    let band = ((available_h - gap * (slots - 1.0)) / slots).max(0.0);
    let height = content_h.map_or(band, |content| content.clamp(0.0, band));
    let x = match anchor.horizontal() {
        "left" => body.x + inset,
        "right" => body.x + body.w - inset - width,
        _ => body.x + (body.w - width) * 0.5,
    };
    let slot_top = body.y + inset + (band + gap) * column_index as f32;
    let slack = (band - height).max(0.0);
    let y = match anchor.vertical() {
        "top" => slot_top,
        "bottom" => slot_top + slack,
        _ => slot_top + slack * 0.5,
    };
    Rect::new(x, y, width, height)
}

/// 🎯️ The widest unoccupied band and the viewport-centered placement clamped into it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ShellChromeCenteredBand {
    pub free: ShellChromeBandSpan,
    pub centered: ShellChromeBandSpan,
}

/// 🧱️ Rust twin of `navbarFreeBandV1`: normalize occupied spans, merge overlaps, and choose
/// the widest remaining interval. A fully occupied bar retains the whole width, matching the shared
/// language-neutral fixture and React helper.
pub(crate) fn shell_chrome_free_band(width: f32, occupied: &[ShellChromeBandSpan]) -> ShellChromeBandSpan {
    let width = width.max(0.0);
    let mut spans: Vec<ShellChromeBandSpan> = occupied.iter().map(|span| ShellChromeBandSpan { left: span.left.clamp(0.0, width), right: span.right.clamp(0.0, width) }).filter(|span| span.right > span.left).collect();
    spans.sort_by(|left, right| left.left.total_cmp(&right.left));
    let mut merged: Vec<ShellChromeBandSpan> = Vec::with_capacity(spans.len());
    for span in spans {
        if let Some(last) = merged.last_mut().filter(|last| span.left <= last.right) {
            last.right = last.right.max(span.right);
        } else {
            merged.push(span);
        }
    }
    let mut band = ShellChromeBandSpan::default();
    let mut cursor = 0.0;
    for span in merged.into_iter().chain(std::iter::once(ShellChromeBandSpan { left: width, right: width })) {
        if span.left - cursor > band.width() {
            band = ShellChromeBandSpan { left: cursor, right: span.left };
        }
        cursor = cursor.max(span.right);
    }
    if band.width() > 0.0 {
        band
    } else {
        ShellChromeBandSpan { left: 0.0, right: width }
    }
}

/// 🎯️ Rust twin of `navbarCenteredLeftV1`, including its pixel rounding — to the nearest whole pixel INSIDE the band, so a
/// sub-pixel band edge is never crossed (the unrounded clamp when no whole pixel fits).
pub(crate) fn shell_chrome_centered_band(width: f32, occupied: &[ShellChromeBandSpan], desired_width: f32) -> ShellChromeCenteredBand {
    let free = shell_chrome_free_band(width, occupied);
    let centered_width = desired_width.max(0.0).min(free.width());
    let latest = free.right - centered_width;
    let left = if latest <= free.left {
        free.left
    } else {
        let (first, last) = (free.left.ceil(), latest.floor());
        let left = ((width - centered_width) * 0.5).clamp(free.left, latest);
        if first > last {
            left
        } else {
            left.round().clamp(first, last)
        }
    };
    ShellChromeCenteredBand { free, centered: ShellChromeBandSpan { left, right: left + centered_width } }
}

pub(crate) fn shell_chrome_clip_horizontal(rect: Rect, span: ShellChromeBandSpan) -> Option<Rect> {
    let left = rect.x.max(span.left);
    let right = (rect.x + rect.w).min(span.right);
    (right > left).then(|| Rect::new(left, rect.y, right - left, rect.h))
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ShellNavbarChromeLayout {
    pub(crate) center: ShellChromeCenteredBand,
    pub(crate) trailing_right: f32,
}

fn utility_sibling_count(roots: &[UtilityNode], path: &[u16; 64], depth: usize) -> Option<usize> {
    if depth == 0 {
        return Some(roots.len());
    }
    let parent = utility_at_path(roots, path, depth - 1)?;
    let UtilityNode::Collection { children, .. } = parent else { return None };
    Some(children.len())
}

pub(crate) fn render_utility_node_step(
    cursor: &mut ShellChromeChildCursor,
    chrome: &mut ShellChromeBuildState,
    draw: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    x: f32,
    btn_y: f32,
    btn_h: f32,
    utility: &UtilityNode,
    collection_expanded: &HashMap<String, bool>,
) -> Result<Option<(f32, bool)>, ()> {
    match utility {
        UtilityNode::Separator { .. } => Ok(Some((render_utility_section_divider(draw, theme, x, btn_y, btn_h), false))),
        UtilityNode::Button { id, icon_id, label, text, title, disabled, on_press, .. } if !disabled.unwrap_or(false) => {
            let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}button.{id}");
            let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(utility_node_label(label, text, title, id)), active: false, disabled: false, kind: HitKind::Button };
            if cursor.rect.is_none() {
                let width = retained_chrome_group_item_width(atlas, theme, &item).ok_or(())?;
                cursor.rect = Some(Rect::new(x, btn_y, width, btn_h));
            }
            let Some(rect) = cursor.rect else { return Err(()) };
            match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
                RetainedChromeGroupStep::Pending => return Ok(None),
                RetainedChromeGroupStep::Fault => return Err(()),
                RetainedChromeGroupStep::Complete => {}
            }
            note_chrome_group_item(&item);
            chrome.register_element_rect(id.clone(), rect);
            input.register_hit(HitTarget { rect, event: Some(on_press.clone()), control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
            cursor.rect = None;
            Ok(Some((x + rect.w + theme.gap_standard * 0.5, false)))
        }
        UtilityNode::Toggle { id, icon_id, label, text, title, pressed, disabled, on_change, .. } if !disabled.unwrap_or(false) => {
            let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}toggle.{id}");
            let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(utility_node_label(label, text, title, id)), active: pressed.unwrap_or(false), disabled: false, kind: HitKind::Toggle };
            if cursor.rect.is_none() {
                let width = retained_chrome_group_item_width(atlas, theme, &item).ok_or(())?;
                cursor.rect = Some(Rect::new(x, btn_y, width, btn_h));
            }
            let Some(rect) = cursor.rect else { return Err(()) };
            match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
                RetainedChromeGroupStep::Pending => return Ok(None),
                RetainedChromeGroupStep::Fault => return Err(()),
                RetainedChromeGroupStep::Complete => {}
            }
            note_chrome_group_item(&item);
            chrome.register_element_rect(id.clone(), rect);
            input.register_hit(HitTarget { rect, event: Some(on_change.clone()), control_id: Some(control_id), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
            cursor.rect = None;
            Ok(Some((x + rect.w + theme.gap_standard * 0.5, false)))
        }
        UtilityNode::Collection { id, icon_id, label, text, title, disabled, children, .. } if !disabled.unwrap_or(false) => {
            let expanded = collection_expanded.get(id).copied().unwrap_or(false);
            let control_id = format!("{WINDOW_UTILITY_RAIL_PARENT}collection.{id}");
            let item = ChromeGroupItem { control_id: &control_id, icon_id: Some(icon_id.as_str()), label: Some(utility_node_label(label, text, title, id)), active: expanded, disabled: false, kind: HitKind::Button };
            if cursor.rect.is_none() {
                let width = retained_chrome_group_item_width(atlas, theme, &item).ok_or(())?;
                cursor.rect = Some(Rect::new(x, btn_y, width, btn_h));
            }
            let Some(rect) = cursor.rect else { return Err(()) };
            match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
                RetainedChromeGroupStep::Pending => return Ok(None),
                RetainedChromeGroupStep::Fault => return Err(()),
                RetainedChromeGroupStep::Complete => {}
            }
            note_chrome_group_item(&item);
            input.register_hit(HitTarget { rect, event: None, control_id: Some(control_id), kind: HitKind::Button, drag_axis: None, drag_data: None });
            cursor.rect = None;
            Ok(Some((x + rect.w + theme.gap_standard * 0.5, expanded && !children.is_empty())))
        }
        UtilityNode::Button { .. } | UtilityNode::Toggle { .. } | UtilityNode::Collection { .. } => Ok(Some((x, false))),
    }
}

//#region ActionPanelAndUtilities
//#region 🔀️ChromeParity
/// 🎛️ One navbar control of the shell's centre cluster, resolved before anything is painted so the
/// laws below can assert "a hit target exists exactly when the app declares it" without a
/// `ShellState` (90+ fields, several without `Default`). The control ids are React's `ShellHost`
/// ids verbatim — one product across both renderers, one id for every probe.
///
/// @see `🧱️elements/🏛️ShellHost/🟦️.tsx` — `exampleSelectElement`/`modeSwitcherElement`/`roleSwitcherElement`
/// @see `🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts` — `SURFACE_ROLE_CONTROL_IDS`
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ShellNavbarControl {
    pub control_id: String,
    pub icon_id: Option<&'static str>,
    pub label: String,
    pub active: bool,
    pub width_policy: Option<ShellNavbarWidthPolicy>,
}

/// 📐️ Resolves one control's available width through its authored width policy. The caller supplies
/// logical pixels, so neither device density nor typography participates in root-rem conversion.
pub(crate) fn resolve_shell_navbar_control_width(available_width: f32, policy: Option<ShellNavbarWidthPolicy>, root_rem_pixels: f32) -> f32 {
    match policy {
        Some(ShellNavbarWidthPolicy::RootRemClamp { minimum_rem, maximum_rem }) => available_width.clamp(minimum_rem as f32 * root_rem_pixels, maximum_rem as f32 * root_rem_pixels),
        None => available_width,
    }
}

/// 📚️ The `playground.navbar.fixture` trigger, or `None` when the open dialect authored no example —
/// React's `exampleOptions.length > 0` render gate, verbatim. The label is the SELECTED row's own
/// label (a select shows its value), falling back to the localized noun when nothing is picked yet.
pub(crate) fn shell_example_control(rows: &[ShellExampleRow], open: bool, is_de: bool) -> Option<ShellNavbarControl> {
    if rows.is_empty() {
        return None;
    }
    let label = rows.iter().find(|row| row.selected).map_or_else(|| shell_chrome_string("example.picker", is_de).to_string(), |row| row.label.clone());
    Some(ShellNavbarControl { control_id: "playground.navbar.fixture".to_string(), icon_id: Some("file"), label, active: open, width_policy: Some(ShellNavbarWidthPolicy::RootRemClamp { minimum_rem: 12, maximum_rem: 28 }) })
}

/// 🎛️ The `playground.navbar.modes.<id>` group — empty for a single-mode surface, exactly as React
/// renders no mode switcher for one (the viewer of generation3d has one mode and shows none).
pub(crate) fn shell_mode_controls(app: &AppDefinition, active_mode_id: Option<&str>, terminology: Terminology, locale: Locale) -> Vec<ShellNavbarControl> {
    if app.modes.len() < 2 {
        return Vec::new();
    }
    app.modes
        .iter()
        .map(|mode| ShellNavbarControl { control_id: format!("playground.navbar.modes.{}", mode.id), icon_id: None, label: mode.label.resolve(terminology, locale).to_string(), active: active_mode_id == Some(mode.id.as_str()), width_policy: None })
        .collect()
}

/// 👁️✏️ The `playground.navbar.roles.{editor,viewer}` group. Icons are fixed per role (`pencil`/`eye`),
/// deliberately NOT each app's own `icon_id`: both surfaces of one artifact carry the same artifact
/// icon, which would make the two buttons indistinguishable. Labels are each target
/// `AppDefinition`'s own `LocalizedLabel` — no shell dictionary, no default language.
///
/// ⌨️ Each label carries its chord INLINE (`Editor ⌘️⌥️E`), because React's `ButtonGroupItem` paints a
/// `ControlHotkeyBadge` beside its own inline label whenever the driver's `hotkeys` axis is `"inline"`
/// — which the default driver sets — and `playground.navbar.roles.*` are the two `SHELL_KEYBINDINGS`
/// rows that can be per-button at all (`🔨️modules/🕹️control-keybinding-context/🟦️.tsx:162`). The chord
/// is read off this session's own remappable table, never off the frozen defaults.
pub(crate) fn shell_role_controls(apps: &[AppDefinition], app: &AppDefinition, terminology: Terminology, locale: Locale, shortcuts: &[(String, String)]) -> Vec<ShellNavbarControl> {
    let Some((editor, viewer)) = surface_role_apps(apps, &app.dialect) else {
        return Vec::new();
    };
    [(editor, semio_framework::manifest::AppRole::Editor, "pencil"), (viewer, semio_framework::manifest::AppRole::Viewer, "eye")]
        .into_iter()
        .map(|(target, role, icon_id)| {
            let control_id = format!("playground.navbar.roles.{}", role.as_str());
            let label = target.label.resolve(terminology, locale).to_string();
            let label = shell_control_hotkey_badge(shortcuts, &control_id).map_or_else(|| label.clone(), |badge| format!("{label} {badge}"));
            ShellNavbarControl { control_id, icon_id: Some(icon_id), label, active: app.role == role, width_policy: None }
        })
        .collect()
}

/// ⏳️🖼️ The pill one surface's published status asks for, or `None` when the status is settled.
pub(crate) fn world3d_status_pill_for(surface_id: &str, status: &World3dComputeStatus, is_de: bool) -> Option<World3dStatusPill> {
    if !world3d_status_is_visible(status) {
        return None;
    }
    let phase_text = match (status.phase_label.as_ref(), is_de) {
        (Some((_, de)), true) => de.clone(),
        (Some((en, _)), false) => en.clone(),
        (None, _) => shell_chrome_string("common.loading", is_de).to_string(),
    };
    let (progress_text, ratio) = if status.units_total > 0.0 {
        let percent = (status.ratio * 100.0).round() as i64;
        (Some(format!("{}/{} ({percent}%)", status.units_done as i64, status.units_total as i64)), Some(status.ratio as f32))
    } else {
        (None, None)
    };
    Some(World3dStatusPill { surface_id: surface_id.to_string(), phase: status.phase.clone(), phase_text, progress_text, ratio, computing: status.computing })
}

/// 📏️ How wide a pill's own row is, on the same metric `retained_chrome_group_item_width` prices a
/// chrome control's label at — so the cancel control that follows it on the same row can be anchored
/// without measuring a glyph.
pub(crate) fn world3d_status_pill_width(theme: &Theme, pill: &World3dStatusPill) -> f32 {
    theme.padding_standard * 2.0 + pill.label().len() as f32 * theme.font_size_small * 0.6
}

/// 🛟️ Clear every shell panel along one authored anchor axis, including cascaded obstacles. Preserve the
/// axis gap, round outward, and remain entirely inside the host. The least viable displacement wins;
/// block wins ties. No overlap or no fit leaves placement unchanged. React consumes the same corpus.
pub(crate) fn chrome_panel_safe_area(affordance: Rect, host: Rect, anchor: PanelAnchor, panels: &[Rect], yield_axis: SafeAreaYield, gap: f32) -> ChromePanelSafeArea {
    if !panels.iter().any(|panel| rects_overlap(*panel, affordance)) { return ChromePanelSafeArea::default(); }
    let block = if yield_axis == SafeAreaYield::Inline { 0.0 } else {
        chrome_panel_axis_reserve(affordance, host, panels, false, match anchor.vertical() { "top" => 1.0, "bottom" => -1.0, _ => 0.0 }, gap)
    };
    let inline = if yield_axis == SafeAreaYield::Block { 0.0 } else {
        chrome_panel_axis_reserve(affordance, host, panels, true, match anchor.horizontal() { "left" => 1.0, "right" => -1.0, _ => 0.0 }, gap)
    };
    if block > 0.0 && (inline == 0.0 || block <= inline) { return ChromePanelSafeArea { inline: 0.0, block }; }
    if inline > 0.0 { return ChromePanelSafeArea { inline, block: 0.0 }; }
    ChromePanelSafeArea::default()
}

/// 🛟️ Where a live surface's overlay row actually starts once the open floating panels over it are
/// reserved — the wgpu twin of the React world pane's `useChromePanelSafeArea` rail offset. The status
/// pill and the cancel control that follows it share this one origin, so the row moves as one affordance.
pub(crate) fn surface_overlay_row_origin(theme: &Theme, bounds: Rect, panels: &[Rect]) -> [f32; 2] {
    let row = surface_overlay_row_box(theme, bounds);
    let safe_area = chrome_panel_safe_area(row, bounds, PanelAnchor::TopLeft, panels, SafeAreaYield::Either, theme.gap_standard);
    [row.x + safe_area.inline, row.y + safe_area.block]
}

/// ⏳️🖼️ The compute-status pills a set of live World3d surfaces asks for, each with the rect it is
/// painted in — pure over the surfaces' own bounds and status, so the laws drive it without a
/// `ShellState`. The pill leads the surface's overlay row; `surface_overlay_controls_for` anchors
/// the cancel control after it, which is the same left-to-right order React's pane lays out.
pub(crate) fn surface_status_pills_for(worlds: &[(&str, Rect, Option<&str>)], panels: &[Rect], theme: &Theme, is_de: bool) -> Vec<(World3dStatusPill, Rect)> {
    worlds
        .iter()
        .filter(|(_, bounds, _)| surface_fits_overlay(theme, *bounds))
        .filter_map(|(surface_id, bounds, status_json)| {
            let pill = world3d_status_pill_for(surface_id, &world3d_compute_status(*status_json), is_de)?;
            let origin = surface_overlay_row_origin(theme, *bounds, panels);
            let room = (bounds.x + bounds.w - theme.gap_standard - origin[0]).max(theme.control_height);
            let width = world3d_status_pill_width(theme, &pill).min(room);
            Some((pill, Rect::new(origin[0], origin[1], width, theme.control_height)))
        })
        .collect()
}

/// 🛑️🖼️ The overlay controls a set of live engine surfaces offers, with the anchor each is painted
/// at — pure over the surfaces' own bounds and status, so the laws drive it without a `ShellState`.
/// A surface too small to carry a control offers none, which is what keeps a collapsed dock pane from
/// painting chrome over its whole body.
pub(crate) fn surface_overlay_controls_for(_graphs: &[(&str, Rect)], worlds: &[(&str, Rect, Option<&str>)], panels: &[Rect], theme: &Theme, is_de: bool) -> Vec<(ShellNavbarControl, [f32; 2])> {
    let mut controls = Vec::new();
    for (surface_id, bounds, status_json) in worlds.iter().filter(|(_, bounds, _)| surface_fits_overlay(theme, *bounds)) {
        let status = world3d_compute_status(*status_json);
        if !status.cancellable {
            continue;
        }
        let lead = world3d_status_pill_for(surface_id, &status, is_de).map(|pill| world3d_status_pill_width(theme, &pill) + theme.gap_standard).unwrap_or(0.0);
        let control = ShellNavbarControl { control_id: format!("shell.world3d.cancel::{surface_id}"), icon_id: Some("x"), label: shell_chrome_string("common.cancel", is_de).to_string(), active: false, width_policy: None };
        let origin = surface_overlay_row_origin(theme, *bounds, panels);
        controls.push((control, [origin[0] + lead, origin[1]]));
    }
    controls
}

//#endregion 🔀️ChromeParity
//#region 🪟️WindowPaneChrome

impl<'ctx> ChromeViewCtx<'ctx> {
    /// 🧰️ One paint opportunity of ONE pane's UNFOLDED Utilities rail — the app's own utility tree,
    /// walked with the very stepper the footer rail used, laid out to the RIGHT of the `Utilities`
    /// chip on the pane's bottom-left row. This is where React puts them (`utilityBarNode(utilities,
    /// instance.id, …)` handed to the `bottom-left` `Pane`, `🏛️ShellHost/🟦️.tsx:10263`).
    ///
    /// 🖼️ The rail's own surface, pushed ONCE — React's unfolded `Pane` body carries a panel
    /// background, and a rail painted straight onto a 3D scene is unreadable.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub(crate) fn paint_window_utilities_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, window_id: &str, window_rect: Rect) -> bool {
        if self.windows.utility_bar_folded(window_id) || !surface_fits_overlay(theme, window_rect) {
            return true;
        }
        let Some(session) = self.docs.session.clone() else { return true };
        let utilities = self.dock.derive_window_utility_nodes(&session, window_id, &self.prefs, &self.windows);
        if utilities.is_empty() {
            return true;
        }
        let btn_y = window_rect.y + window_rect.h - theme.panel_inset - theme.control_height;
        if cursor.x <= 0.0 {
            let chip = ChromeGroupItem {
                control_id: "",
                icon_id: Some(WindowPaneChip::Utilities.icon_id()),
                label: Some(shell_chrome_string(WindowPaneChip::Utilities.label_key(), self.prefs.locale_id == "de")),
                active: true,
                disabled: false,
                kind: HitKind::Toggle,
            };
            let chip_w = retained_chrome_group_item_width(atlas, theme, &chip).unwrap_or(theme.control_height);
            cursor.x = window_pane_chip_rect(theme, window_rect, PanelAnchor::BottomLeft, chip_w).x + chip_w + theme.gap_standard;
            draw.push_solid([cursor.x, btn_y, (window_rect.x + window_rect.w - theme.panel_inset - cursor.x).max(0.0), theme.control_height], theme.panel);
        }
        if cursor.flag {
            let Some(next_depth) = cursor.depth.checked_add(1).filter(|depth| *depth < cursor.path.0.len()) else { return true };
            cursor.depth = next_depth;
            cursor.path.0[next_depth] = 0;
            cursor.flag = false;
            return false;
        }
        let Some(utility) = utility_at_path(&utilities, &cursor.path.0, cursor.depth) else {
            let siblings = utility_sibling_count(&utilities, &cursor.path.0, cursor.depth).unwrap_or(0);
            let next = cursor.path.0[cursor.depth] as usize + 1;
            if next < siblings {
                cursor.path.0[cursor.depth] = next as u16;
                return false;
            }
            if cursor.depth > 0 {
                cursor.depth -= 1;
                return false;
            }
            return true;
        };
        let x = cursor.x;
        let result = render_utility_node_step(cursor, &mut self.chrome.chrome_build, draw, atlas, icons, input, theme, x, btn_y, theme.control_height, utility, &self.windows.utility_collection_expanded);
        let advanced = match result {
            Ok(result) => result,
            Err(()) => {
                self.chrome.error = Some("Shell pane utility exceeded the retained chrome boundary".to_string());
                return true;
            }
        };
        let Some((next_x, descend)) = advanced else { return false };
        cursor.x = next_x;
        cursor.flag = descend;
        if descend {
            return false;
        }
        let siblings = utility_sibling_count(&utilities, &cursor.path.0, cursor.depth).unwrap_or(0);
        let next = cursor.path.0[cursor.depth] as usize + 1;
        if next < siblings {
            cursor.path.0[cursor.depth] = next as u16;
            return false;
        }
        if cursor.depth > 0 {
            cursor.depth -= 1;
            return false;
        }
        true
    }

    /// 🔀️ One paint opportunity of ONE pane's UNFOLDED retained Projection Tree.
    ///
    /// 🧭️ Selecting a row is VIEW STATE and dispatches nothing, which is React's own rule for this
    /// pane verbatim: `handleProjectionKindChange` only sets `externalPendingProjectionSpec`, because
    /// the single `setProjection` consumer takes granular `{field, value}` pairs and has no lossless
    /// mapping from a whole spec (`🌐️World3dHost/🟦️.tsx:6574`). Here the selection is observable in
    /// the chip's own icon; making the ORBIT follow it needs an orthographic frustum this renderer's
    /// `OrbitController` does not carry yet (`♾️infinite/🌍️world/🦀️.rs`'s
    /// `WORLD3D_PERSPECTIVE_CAMERA_ZOOM`) — the World3d lane's gap, not this one's.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub(crate) fn paint_window_projection_step(
        &mut self,
        cursor: &mut ShellChromeChildCursor,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        window_id: &str,
        window_rect: Rect,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> bool {
        if self.windows.projection_pane_folded(window_id) || self.surfaces.world3d_host_id_for_window(window_id).is_none() || !surface_fits_overlay(theme, window_rect) {
            return true;
        }
        if !self.docs.window_projection_documents.contains_key(window_id) {
            if let Err(error) = self.windows.refresh_window_projection_document(window_id, &self.surfaces, &mut self.docs) {
                self.chrome.error = Some(error);
                return true;
            }
        }
        let Some(document) = self.docs.window_projection_documents.remove(window_id) else { return true };
        let surface = window_projection_surface_id(window_id);
        crate::interpreter::set_ui_document_flow(&surface, ui_contract::UiFlow { inline: ui_contract::FlowInline::Rtl, block: ui_contract::FlowBlock::Down });
        if cursor.document.phase_name() == "viewport" {
            let selected_key = format!(
                "{surface}/{WORLD_PROJECTION_PANE_PARENT}.{}.{}",
                semio_framework::element_id_segment(window_id),
                semio_framework::element_id_segment(self.windows.world_projection_template_id(window_id, &self.surfaces))
            );
            if let Ok(header) = document.header() {
                let _ = crate::interpreter::stamp_ui_document_tree_selected_item(&surface, header.generation, &selected_key);
            }
        }
        let rect = window_projection_rect(window_id, window_rect, theme);
        if !cursor.flag && cursor.document.layout_is_accepted() && cursor.rect.is_some_and(|previous| previous != rect) {
            cursor.document.restart_viewport_after_host_reflow();
        }
        cursor.rect = Some(rect);
        if !cursor.flag && cursor.document.phase_name() == "paint" {
            draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.panel);
            cursor.flag = true;
        }
        let controller = self.docs.document_controller_id(&self.plugins);
        let complete = {
            let scroll_offsets = &mut self.input.scroll_offsets;
            let collapsed_sections = &mut self.input.collapsed_sections;
            let open_selects = &mut self.input.open_selects;
            let widget_maps = &mut self.input.widget_maps_staging;
            let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.prefs.locale_id == "de"), world3d_states: &mut self.surfaces.world3d_states, world_resources, window_id: surface.as_str() };
            let viewport_height_for_widgets = draw.screen_height();
            let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
            ctx.pick_clip = Some(rect);
            render_ui_document_step(&mut cursor.document, &document, rect, &mut ctx, surface.as_str(), controller.as_str(), self.chrome.chrome_build.driver.drag, &mut hosts, cx)
        };
        self.docs.window_projection_documents.insert(window_id.to_string(), document);
        if complete {
            self.as_session().note_retained_body_painted(&surface);
        } else {
            if document_opportunity_remains(cursor, WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES) {
                return false;
            }
            debug_log(&format!(
                "[TRACE] wgpu-shell projection paint {surface} exhausted {WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES} opportunities parked-in={}",
                cursor.document.phase_name()
            ));
            self.as_session().record_document_paint_fault(&surface);
        }
        push_chrome_group_border(draw, rect, theme);
        self.input.register_retained_body_hits(&surface, rect, input);
        true
    }
}

/// 📐️ The Projection pane's 300px bottom-right body, ending above its own chip and growing up.
pub(super) fn window_projection_rect(window_id: &str, body: Rect, theme: &Theme) -> Rect {
    let width = WINDOW_PANE_BODY_WIDTH_PX.min((body.w - theme.panel_inset * 2.0).max(0.0));
    let bottom = body.y + body.h - theme.panel_inset - theme.control_height - theme.gap_standard;
    let available = (bottom - body.y - theme.panel_inset).max(0.0);
    let height = crate::interpreter::retained_content_height(&window_projection_surface_id(window_id)).unwrap_or(available).min(available).max(0.0);
    Rect::new(body.x + body.w - theme.panel_inset - width, bottom - height, width, height)
}

//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities

