//! 🗨️ `shell-view-overlays` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
/// ✅️ One step of the approvals modal's flat paint program — the retained chrome step walks this
/// list by index, so a glyph run that needs another opportunity simply does not advance the index.
pub(crate) enum AgentApprovalPaintOp {
    Scrim,
    Modal(Rect),
    Fill { rect: Rect, color: Rgba },
    Text { value: String, x: f32, y: f32, max_w: f32, size: f32, color: Rgba },
    Hit { rect: Rect, control_id: String },
    Clicks,
}

//#endregion 🧯️TransientNoticeAndAgentOverlays
//#region 🗨️ChromeDialog
/// 📒️ Records what a reader hears for one painted dialog control — its name, consequence text, role, pressed state,
/// value and whether it is gated — in the registry the chrome accessibility projection reads. Both render paths go
/// through here, so the published tree always describes the frame on screen.
pub(crate) fn note_chrome_dialog_control(op: &ChromeDialogPaintOp) {
    match op {
        ChromeDialogPaintOp::Hit { control_id, label, description, semantics, disabled, .. } => {
            note_chrome_control_name(control_id, Some(label));
            note_chrome_control_description(control_id, description.as_deref());
            note_chrome_control_semantics(control_id, semantics);
            note_chrome_control_disabled(control_id, *disabled);
        }
        ChromeDialogPaintOp::Segment { control_id, label, selected, .. } => {
            note_chrome_control_name(control_id, Some(label));
            note_chrome_control_semantics(control_id, &ChromeDialogSemantics { role: Some("button"), pressed: Some(*selected), value_text: None, value_range: None });
        }
        ChromeDialogPaintOp::Nudge { control_id, label, .. } => note_chrome_control_name(control_id, Some(label)),
        _ => {}
    }
}

//#endregion 🗨️ChromeDialog
/// 🎓️ Punches `hole` out of `band`, returning up to four remaining rectangles (or the original band when
/// they don't overlap). The React shell renders one fullscreen `ui-veil` div and raises the
/// introduced/shown element's chrome unit above it via z-index; wgpu keeps the geometric subtraction
/// because 3D window content lives in separate `scene_passes` that cannot be repainted above an
/// overlay at all, so a cutout is the only way to leave the introduced element unveiled.
pub(crate) fn punch_introduction_cutout(band: Rect, hole: Rect) -> Vec<Rect> {
    let top = band.y.max(hole.y);
    let left = band.x.max(hole.x);
    let bottom = (band.y + band.h).min(hole.y + hole.h);
    let right = (band.x + band.w).min(hole.x + hole.w);
    if right <= left || bottom <= top {
        return vec![band];
    }
    [Rect::new(band.x, band.y, band.w, top - band.y), Rect::new(band.x, bottom, band.w, band.y + band.h - bottom), Rect::new(band.x, top, left - band.x, bottom - top), Rect::new(right, top, band.x + band.w - right, bottom - top)]
        .into_iter()
        .filter(|piece| piece.w > 0.0 && piece.h > 0.0)
        .collect()
}

/// 🎓️ Splits the viewport into bands tiling the space around every cutout — an empty cutout list
/// returns one full-viewport band. Each band is pushed as a REAL `ui-veil` glass region
/// (`Theme::veil_glass`), so the shell behind it is blurred exactly as React's `backdrop-filter` blurs
/// the page.
///
/// 🩸️ This docstring used to argue that a real glass veil was infeasible, reading the batch renderer's
/// `composite_to_swapchain` — where overlay glass composites BEFORE the overlay's own instance pass,
/// i.e. beneath the chrome it should frost. That path is `#[cfg(test)]`. The production ladder is
/// `Commands → BlurScene → EncodeComposite → GlassCommands → ForegroundCommands` (`🧊️gpu/🦀️.rs`),
/// which encodes every non-glass-content layer of BOTH lists into the scene the blur chain mips, so a
/// glass region composites the frosted navbar, footer and panes — not blurred bare canvas. The
/// argument was true of the batch path and false of the one that actually presents
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W8a).
///
/// ⚠️ One divergence remains: `ForegroundCommands` re-encodes EVERY glass-content layer after the
/// glass pass, so a window cap's own chips (`🛰️Dock`'s `begin_glass_content`) stay crisp over the
/// veil where React blurs them. Ordering foreground content against a LATER glass region is a
/// ladder-level change — see the packet report's hand-offs.
pub(crate) fn introduction_veil_bands(width: f32, height: f32, cutouts: &[Rect]) -> Vec<Rect> {
    let mut bands = vec![Rect::new(0.0, 0.0, width, height)];
    for cutout in cutouts {
        let x = cutout.x.clamp(0.0, width);
        let y = cutout.y.clamp(0.0, height);
        let w = (cutout.x + cutout.w).clamp(0.0, width) - x;
        let h = (cutout.y + cutout.h).clamp(0.0, height) - y;
        if w <= 0.0 || h <= 0.0 {
            continue;
        }
        let clamped = Rect::new(x, y, w, h);
        bands = bands.into_iter().flat_map(|band| punch_introduction_cutout(band, clamped)).collect();
    }
    bands
}

pub(crate) const INTRODUCTION_INFO_BOX_GAP: f32 = 16.0;

/// 🎓️ Where the info box sits relative to `anchor` — byte-for-byte port of `resolveIntroductionPlacement`
/// (`ui/js/react/index.tsx`). `auto` picks the side with the most free viewport space; `center` (and any
/// anchor-less step) centers the box.
pub(crate) fn resolve_introduction_placement(placement: semio_framework::onboarding::IntroductionPlacement, anchor: Option<Rect>, box_size: (f32, f32), viewport: (f32, f32)) -> (f32, f32) {
    use semio_framework::onboarding::IntroductionPlacement;
    let (box_w, box_h) = box_size;
    let (vw, vh) = viewport;
    let centered = ((vw - box_w) / 2.0, (vh - box_h) / 2.0);
    let Some(anchor) = anchor else {
        return centered;
    };
    if matches!(placement, IntroductionPlacement::Center) {
        return centered;
    }
    let gap = INTRODUCTION_INFO_BOX_GAP;
    let clamp_left = |left: f32| left.max(gap).min((vw - box_w - gap).max(gap));
    let clamp_top = |top: f32| top.max(gap).min((vh - box_h - gap).max(gap));
    let space_top = anchor.y;
    let space_bottom = vh - (anchor.y + anchor.h);
    let space_left = anchor.x;
    let space_right = vw - (anchor.x + anchor.w);
    let side = if matches!(placement, IntroductionPlacement::Auto) {
        [("top", space_top), ("bottom", space_bottom), ("left", space_left), ("right", space_right)].into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)).map(|(side, _)| side).unwrap_or("bottom")
    } else {
        match placement {
            IntroductionPlacement::Top => "top",
            IntroductionPlacement::Bottom => "bottom",
            IntroductionPlacement::Left => "left",
            IntroductionPlacement::Right => "right",
            _ => "bottom",
        }
    };
    match side {
        "top" => (clamp_left(anchor.x + anchor.w / 2.0 - box_w / 2.0), clamp_top(anchor.y - box_h - gap)),
        "bottom" => (clamp_left(anchor.x + anchor.w / 2.0 - box_w / 2.0), clamp_top(anchor.y + anchor.h + gap)),
        "left" => (clamp_left(anchor.x - box_w - gap), clamp_top(anchor.y + anchor.h / 2.0 - box_h / 2.0)),
        "right" => (clamp_left(anchor.x + anchor.w + gap), clamp_top(anchor.y + anchor.h / 2.0 - box_h / 2.0)),
        _ => centered,
    }
}

/// 🎓️ React's card is `w-fit max-w-sm` (`UIIntroduction`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`) — 24rem of
/// content at the widest, and never so narrow that a two-word step reads as a label.
const INTRODUCTION_BOX_MAX_WIDTH: f32 = 384.0;

const INTRODUCTION_BOX_MIN_WIDTH: f32 = 240.0;

/// 🎓️ Port of `splitIntroductionBodyParagraphs` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:4900`): a blank line
/// starts a new `[data-slot="introduction-body-paragraph"]`, empties are dropped.
fn split_introduction_body_paragraphs(body: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    for raw in body.split("\n\n") {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            paragraphs.push(trimmed.to_string());
        }
    }
    paragraphs
}

/// 🎓️ Where the element `step.introduce`/`step.show` names actually IS on this frame's screen — the
/// wgpu twin of React's `useIntroductionAnchorRect`/`useIntroductionElevation`, which resolve the id
/// through `elementIdSelector` and read the DOM rect. Three registries answer here, in React's own
/// order of specificity: the per-frame logical rect registry (`register_element_rect` — utility bar
/// buttons, panel tab chips, window bodies), then the dock's own window content rects for a
/// `framework.window.<kind>` id, then nothing.
fn chrome_tour_anchor_rect(state: &OverlaysViewCtx<'_>, element_id: &str) -> Option<Rect> {
    if let Some(rect) = state.chrome.chrome_build.resolve_element_rect(element_id) {
        return Some(rect);
    }
    let rest = element_id.strip_prefix("framework.window.")?;
    let segment = rest.split_once(".action.").map_or(rest, |(segment, _)| segment);
    let session = state.docs.session.as_ref()?;
    let kind = session.app.window_kinds.iter().find(|kind| semio_framework::element_id_segment(&kind.id) == segment)?;
    state.dock.dock.window_instances().into_iter().filter(|(_, kind_id)| kind_id == &kind.id).find_map(|(window_id, _)| state.dock.window_content_rects.get(&window_id).copied())
}

/// ⌨️ The introduction's own chords, the three rows of React's `SHELL_KEYBINDINGS`
/// (`🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:163-166`) that [`SHELL_SHORTCUT_ROWS`] does not
/// carry: that table is the ACCELERATOR table, every row of which must resolve to a [`ShellShortcut`]
/// verb, and these three are answered by the tour's own ladder instead. They are listed here so the
/// card's buttons can wear their chord the way React's `ControlHotkeyBadge` does.
const INTRODUCTION_SHORTCUT_ROWS: &[(&str, &str)] = &[(UI_INTRODUCTION_SKIP_CONTROL_ID, "escape"), (UI_INTRODUCTION_NEXT_CONTROL_ID, "enter,arrowright"), (UI_INTRODUCTION_BACK_CONTROL_ID, "arrowleft")];

/// ⌨️ The chord badge one introduction control wears — its user override first, then its React
/// default. `Next ↵`, exactly as `🗑️generated/react-6313/final.png` reads it.
fn introduction_control_hotkey(overrides: &HashMap<String, String>, control_id: &str) -> Option<String> {
    let keys = overrides.get(control_id).filter(|keys| keys.split(',').any(|chord| !chord.trim().is_empty())).map(String::as_str).or_else(|| INTRODUCTION_SHORTCUT_ROWS.iter().find(|(id, _)| *id == control_id).map(|(_, keys)| *keys))?;
    let badge = format_keybinding_shortcut(keys);
    (!badge.is_empty()).then_some(badge)
}

/// 🎓️ One chip of the tour card, in React's own reading order: the inline label, then the
/// `ControlHotkeyBadge` chord, then the icon — `ButtonGroupItem` renders exactly that sequence
/// (`🖱️ui/🧱️elements/🔳️ButtonGroup/🟦️.tsx:110-118`), which is why React's footer reads `Next ↵ ›`
/// with the chevron LAST. `leading_icon` is the one inversion React itself makes: `WindowChrome`'s
/// `close` control paints its `CloseIcon` before the label, so Skip reads `✕ Skip`.
struct ChromeTourChip {
    pub(crate) rect: Rect,
    pub(crate) label: &'static str,
    pub(crate) hotkey: Option<String>,
    pub(crate) icon_id: &'static str,
    pub(crate) leading_icon: bool,
    filled: bool,
}

/// 🎓️ Where one tour chip's leading or trailing icon sits — React's `ButtonGroupItem` puts the icon
/// LAST, after the label and the chord badge, and only `WindowChrome`'s `close` control leads with it.
pub(crate) fn chrome_tour_chip_icon_x(chip: &ChromeTourChip, theme: &Theme) -> f32 {
    if chip.leading_icon {
        chip.rect.x + theme.padding_standard * 2.0
    } else {
        chip.rect.x + chip.rect.w - theme.padding_standard * 2.0 - CHROME_ICON_TINY
    }
}

/// 🎓️ The `(x, baseline, width)` one tour chip's inline label flows in — `px-double` from the chip's
/// own edge, after a leading icon when it carries one.
pub(crate) fn chrome_tour_chip_label_box(chip: &ChromeTourChip, atlas: &mut FontAtlas, theme: &Theme) -> (f32, f32, f32) {
    let leading = if chip.leading_icon { CHROME_ICON_TINY + theme.gap_standard } else { 0.0 };
    let x = chip.rect.x + theme.padding_standard * 2.0 + leading;
    let width = atlas.measure_text(chip.label, theme.font_size_small).0.max(1.0);
    (x, chip.rect.y + (chip.rect.h + theme.font_size_small) * 0.5 - 1.0, width)
}

/// ⌨️ The chord badge box beside one tour chip's label — React's `ControlHotkeyBadge`, which sits
/// BETWEEN the inline label and the icon (`🖱️ui/🧱️elements/🔳️ButtonGroup/🟦️.tsx:110-118`).
pub(crate) fn chrome_tour_chip_hotkey_box(chip: &ChromeTourChip, atlas: &mut FontAtlas, theme: &Theme) -> Option<(String, (f32, f32, f32))> {
    let badge = chip.hotkey.clone()?;
    let (label_x, baseline, label_w) = chrome_tour_chip_label_box(chip, atlas, theme);
    let width = atlas.measure_text(&badge, theme.font_size_small).0.max(1.0);
    Some((badge, (label_x + label_w + theme.gap_standard, baseline, width)))
}

/// 🎓️ The hairline outline every tour chip and the card's own title chip wear — React's
/// `windowChromeTitleChipClass` box, four `--border-normal` strokes, the same idiom
/// [`render_retained_chrome_group_item_step`] uses for a navbar chip.
pub(crate) fn push_chrome_tour_chip_border(overlay: &mut DrawList, rect: Rect, theme: &Theme) {
    overlay.push_solid([rect.x, rect.y, rect.w, theme.stroke_hairline], theme.border_normal);
    overlay.push_solid([rect.x, rect.y + rect.h - theme.stroke_hairline, rect.w, theme.stroke_hairline], theme.border_normal);
    overlay.push_solid([rect.x, rect.y, theme.stroke_hairline, rect.h], theme.border_normal);
    overlay.push_solid([rect.x + rect.w - theme.stroke_hairline, rect.y, theme.stroke_hairline, rect.h], theme.border_normal);
}

/// 🎓️ One tour step's whole geometry, measured once per walk step so the card, the veil holes and the
/// hit targets can never disagree about where anything is.
pub(crate) struct ChromeTourLayout {
    pub(crate) card: Rect,
    pub(crate) title: String,
    pub(crate) title_chip: Rect,
    pub(crate) title_rect: Rect,
    pub(crate) grip: Rect,
    pub(crate) skip: ChromeTourChip,
    pub(crate) back: Option<ChromeTourChip>,
    pub(crate) next: Option<ChromeTourChip>,
    pub(crate) counter: Rect,
    pub(crate) counter_text: String,
    pub(crate) footer_rule: Rect,
    pub(crate) paragraphs: Vec<(String, Rect)>,
    pub(crate) checklist: Vec<(String, bool, Rect)>,
    pub(crate) cutouts: Vec<Rect>,
    pub(crate) veil_blocks_pointer: bool,
}

/// 🎓️ Measures one step into a [`ChromeTourLayout`] — React's own card, term for term: Skip in the cap
/// row beside the title, the body's blank-line paragraphs, the interaction checklist with its
/// check/circle bullets, and a footer of Back (from step 2 on) / `i / n` / Next-or-Done. The box is
/// sized to its copy and placed by `resolve_introduction_placement` against the step's own anchor.
///
/// 🩸️ What this replaces: a `Rect::new((width - 320.0) * 0.5, (height - 168.0) * 0.5, 320.0, 168.0)`
/// — a fixed box, always centred, which clipped any step whose copy outgrew 168px, ignored
/// `step.placement` entirely (`resolve_introduction_placement` was `#[cfg(test)]`, i.e. production-dead)
/// and painted neither the checklist nor the spotlight.
///
/// 🧯️ React's `veilBlocksPointer`: only a screen-style step (`introduce == null`) or one whose
/// target actually resolved owns every pointer — an unresolved target would otherwise trap the
/// user behind an opaque veil with no way to reveal it (`UIIntroduction`).
pub(crate) fn chrome_tour_layout(state: &OverlaysViewCtx<'_>, atlas: &mut FontAtlas, theme: &Theme, step: &semio_framework::onboarding::IntroductionStepDefinition, step_index: usize, step_count: usize, width: f32, height: f32) -> ChromeTourLayout {
    let is_de = state.prefs.locale_id == "de";
    let is_last = step_index + 1 == step_count;
    let advances_by_button = step.interactions.is_empty();
    let shows_back = step_index > 0;
    let pad = theme.padding_standard * 2.0;
    let gap = theme.gap_standard;
    let body_gap = theme.gap_standard * 2.0;
    let cap_h = theme.control_height;
    let line_h = theme.font_size_small * 1.6;
    let overrides = &state.chrome.chrome_build.preferences.keybinding_overrides;
    let title = step.title.resolve(state.prefs.active_terminology(), state.prefs.active_locale()).to_string();
    let paragraph_texts = split_introduction_body_paragraphs(&step.body.resolve(state.prefs.active_terminology(), state.prefs.active_locale()));
    let checklist_texts: Vec<(String, bool)> = step
        .interactions
        .iter()
        .enumerate()
        .map(|(index, interaction)| {
            let done = state.chrome.chrome_build.tour_state.as_ref().is_some_and(|tour| tour.completed_interactions.contains(&index));
            let label = if step.ordered { format!("{}. {}", index + 1, interaction.label) } else { interaction.label.clone() };
            (label, done)
        })
        .collect();
    let skip_label = shell_chrome_string("introduction.skip", is_de);
    let back_label = shell_chrome_string("introduction.back", is_de);
    let next_label = shell_chrome_string(if is_last { "introduction.done" } else { "introduction.next" }, is_de);
    let next_hotkey = introduction_control_hotkey(overrides, UI_INTRODUCTION_NEXT_CONTROL_ID);
    let back_hotkey = introduction_control_hotkey(overrides, UI_INTRODUCTION_BACK_CONTROL_ID);
    let counter_text = format!("{} / {step_count}", step_index + 1);
    let chip_w = |atlas: &mut FontAtlas, label: &str, hotkey: Option<&String>, icons: usize| {
        let hotkey_w = hotkey.map_or(0.0, |badge| gap + atlas.measure_text(badge, theme.font_size_small).0);
        theme.padding_standard * 4.0 + atlas.measure_text(label, theme.font_size_small).0 + hotkey_w + icons as f32 * (gap + CHROME_ICON_TINY)
    };
    let skip_w = chip_w(atlas, skip_label, None, 1);
    let back_w = chip_w(atlas, back_label, back_hotkey.as_ref(), 1);
    let next_w = chip_w(atlas, next_label, next_hotkey.as_ref(), 1);
    let counter_w = chip_w(atlas, &counter_text, None, 0);
    let title_chip_w = theme.padding_standard * 4.0 + atlas.measure_text(&title, theme.font_size_body).0 + gap + CHROME_ICON_TINY;
    let footer_w = if shows_back { back_w + gap } else { 0.0 } + counter_w + gap + if advances_by_button { next_w } else { 0.0 };
    let mut content_w = (title_chip_w + gap + skip_w).max(footer_w);
    for paragraph in &paragraph_texts {
        content_w = content_w.max(atlas.measure_text(paragraph, theme.font_size_small).0);
    }
    for (label, _) in &checklist_texts {
        content_w = content_w.max(theme.font_size_small + gap + atlas.measure_text(label, theme.font_size_small).0);
    }
    let content_w = content_w.clamp(INTRODUCTION_BOX_MIN_WIDTH, (INTRODUCTION_BOX_MAX_WIDTH - pad * 2.0).max(INTRODUCTION_BOX_MIN_WIDTH));
    let paragraph_heights: Vec<f32> = paragraph_texts.iter().map(|paragraph| atlas.measure_text_wrapped(paragraph, content_w, theme.font_size_small).1.max(line_h)).collect();
    let body_h = paragraph_heights.iter().sum::<f32>() + body_gap * paragraph_heights.len().saturating_sub(1) as f32;
    let checklist_h = line_h * checklist_texts.len() as f32;
    let card_w = content_w + pad * 2.0;
    let card_h = pad * 2.0 + cap_h + if body_h > 0.0 { body_gap + body_h } else { 0.0 } + if checklist_h > 0.0 { body_gap + checklist_h } else { 0.0 } + body_gap + theme.control_height;
    let mut cutouts: Vec<Rect> = Vec::new();
    let anchor = step.introduce.as_deref().and_then(|id| chrome_tour_anchor_rect(state, id));
    if let Some(anchor) = anchor {
        cutouts.push(anchor);
    }
    for id in &step.show {
        if let Some(rect) = chrome_tour_anchor_rect(state, id) {
            cutouts.push(rect);
        }
    }
    let veil_blocks_pointer = step.introduce.is_none() || anchor.is_some() || !cutouts.is_empty();
    let (left, top) = resolve_introduction_placement(step.placement, anchor, (card_w, card_h), (width, height));
    let card = Rect::new(left, top, card_w, card_h);
    let cap_y = card.y + pad;
    let skip_rect = Rect::new(card.x + card.w - pad - skip_w, cap_y, skip_w, cap_h);
    let title_chip = Rect::new(card.x + pad, cap_y, title_chip_w.min((skip_rect.x - gap - card.x - pad).max(1.0)), cap_h);
    let grip = Rect::new(title_chip.x + title_chip.w - theme.padding_standard * 2.0 - CHROME_ICON_TINY, title_chip.y + (title_chip.h - CHROME_ICON_TINY) * 0.5, CHROME_ICON_TINY, CHROME_ICON_TINY);
    let title_rect = Rect::new(title_chip.x + theme.padding_standard * 2.0, cap_y, (grip.x - gap - title_chip.x - theme.padding_standard * 2.0).max(1.0), cap_h);
    let mut flow_y = cap_y + cap_h + if body_h > 0.0 { body_gap } else { 0.0 };
    let mut paragraphs = Vec::new();
    for (paragraph, paragraph_h) in paragraph_texts.into_iter().zip(paragraph_heights) {
        paragraphs.push((paragraph, Rect::new(card.x + pad, flow_y, content_w, paragraph_h)));
        flow_y += paragraph_h + body_gap;
    }
    if !paragraphs.is_empty() {
        flow_y -= body_gap;
    }
    if checklist_h > 0.0 {
        flow_y += body_gap;
    }
    let mut checklist = Vec::new();
    for (label, done) in checklist_texts {
        checklist.push((label, done, Rect::new(card.x + pad, flow_y, content_w, line_h)));
        flow_y += line_h;
    }
    let footer_y = card.y + card.h - pad - theme.control_height;
    ChromeTourLayout {
        card,
        title,
        title_chip,
        title_rect,
        grip,
        skip: ChromeTourChip { rect: skip_rect, label: skip_label, hotkey: None, icon_id: "x", leading_icon: true, filled: false },
        back: shows_back.then(|| ChromeTourChip { rect: Rect::new(card.x + pad, footer_y, back_w, theme.control_height), label: back_label, hotkey: back_hotkey, icon_id: "chevron-left", leading_icon: false, filled: false }),
        next: advances_by_button.then(|| ChromeTourChip {
            rect: Rect::new(card.x + card.w - pad - next_w, footer_y, next_w, theme.control_height),
            label: next_label,
            hotkey: next_hotkey,
            icon_id: if is_last { "check" } else { "chevron-right" },
            leading_icon: false,
            filled: true,
        }),
        counter: Rect::new(card.x + (card.w - counter_w) * 0.5, footer_y, counter_w, theme.control_height),
        counter_text,
        footer_rule: Rect::new(card.x, footer_y - theme.padding_standard, card.w, theme.stroke_hairline),
        paragraphs,
        checklist,
        cutouts,
        veil_blocks_pointer,
    }
}

//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome


