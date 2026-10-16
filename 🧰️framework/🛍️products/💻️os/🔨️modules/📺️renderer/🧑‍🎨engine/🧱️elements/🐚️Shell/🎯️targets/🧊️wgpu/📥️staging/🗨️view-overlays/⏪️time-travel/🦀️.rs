//! 🗨️ `shell-view-overlays` staging of `time-travel`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;
use super::time_travel::*;
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionStatus, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage};
use semio_framework_time_travel::TimeTravelLabel;

//#region ⏪️TimeTravelControls
/// 📶️ The band's replay progress node — a child of its status node, React's `<progress>` inside the band.
pub(super) const TIME_TRAVEL_BAND_PROGRESS_ID: &str = "shell.time-travel.progress";

/// 🧱️ A band's fill as an opaque surface: a translucent tone tint composited over the menu-level surface, so the content
/// the band floats over never shows through its text.
fn chrome_band_surface(fill: Rgba, theme: &Theme) -> Rgba {
    let base = theme.level_bg[Level::Menu.index()];
    let alpha = fill.a.clamp(0.0, 1.0);
    Rgba { r: base.r + (fill.r - base.r) * alpha, g: base.g + (fill.g - base.g) * alpha, b: base.b + (fill.b - base.b) * alpha, a: 1.0 }
}

//#endregion ⏪️TimeTravelControls
//#region 🗣️TimeTravelCopy
pub(super) const BAND_LABEL_INDICATOR_TARGET: &str = "ui.timeTravel.indicatorTarget";

pub(super) const BAND_LABEL_INDICATOR: &str = "ui.timeTravel.indicator";

/// 🌐️ One `TimeTravelLabel` in `locale`.
pub(crate) fn time_travel_label(label: TimeTravelLabel, locale: Locale) -> &'static str {
    label.localized(|en, de| match locale {
        Locale::En => en,
        Locale::De => de,
    })
}

/// 🏷️ What a window of the editing program says about itself — React's `timeTravelIndicatorTextV1`: the document
/// before the edited mutation while a draft is edited, else the session's stage.
pub(crate) fn time_travel_indicator_text(status: &HistoryTimeTravel, terminology: Terminology, tongue: impl Into<BandTongue>) -> String {
    let tongue: BandTongue = tongue.into();
    match (status.stage, time_travel_target_text(status, terminology, tongue.locale)) {
        (HistoryTimeTravelStage::Editing, Some(target)) => band_label(BAND_LABEL_INDICATOR_TARGET, tongue).replace("{target}", &target),
        (stage, _) => band_label(time_travel_stage_key(stage), tongue).to_string(),
    }
}

/// 🏷️ The indicator chip's visible word — React's `ui.timeTravel.indicator`; its accessible name is
/// [`time_travel_indicator_text`].
pub(crate) fn time_travel_indicator_caption(tongue: impl Into<BandTongue>) -> &'static str {
    band_label(BAND_LABEL_INDICATOR, tongue)
}

/// 🎨️ `(border, fill, text)` of the band — the shell banner severity map: a blocking report is an error; a fault, a
/// review that needs a replay or a worst outcome of warning a warning; everything else the neutral popover chrome.
pub(crate) fn time_travel_band_tone(status: &HistoryTimeTravel, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    use semio_framework::Severity;
    let severity = if status.blocking || status.review == Some(HistoryTimeTravelReview::Blocked) {
        Severity::Error
    } else if status.fault.is_some() || status.review == Some(HistoryTimeTravelReview::NeedsReplay) || status.worst == Some(Severity::Warning) {
        Severity::Warning
    } else {
        Severity::Info
    };
    let (border, fill, text) = transient_notice_tone(severity, theme);
    (border, chrome_band_surface(fill, theme), text)
}

//#endregion 🗣️TimeTravelCopy
//#region 📐️TimeTravelBand
/// 🖌️ One opportunity of a bottom chrome band's frame — its fill (0), its four edges (1–4), the replay track (5) and the track's
/// filled share (6) — shared by the session band and the reprojection band.
fn paint_chrome_band_frame(scalar: usize, overlay: &mut DrawList, band: Rect, progress: Option<(Rect, f32)>, (border, fill): (Rgba, Rgba), theme: &Theme) {
    let hair = theme.stroke_hairline;
    match scalar {
        0 => overlay.push_rounded([band.x, band.y, band.w, band.h], fill, theme.border_radius),
        1 => overlay.push_solid([band.x, band.y, band.w, hair], border),
        2 => overlay.push_solid([band.x, band.y + band.h - hair, band.w, hair], border),
        3 => overlay.push_solid([band.x, band.y, hair, band.h], border),
        4 => overlay.push_solid([band.x + band.w - hair, band.y, hair, band.h], border),
        5 => {
            if let Some((track, _)) = progress {
                overlay.push_solid([track.x, track.y, track.w, track.h], theme.border_normal);
            }
        }
        _ => {
            if let Some((track, share)) = progress {
                overlay.push_solid([track.x, track.y, track.w * share, track.h], theme.progress);
            }
        }
    }
}

/// 🧮️ The opportunities [`paint_chrome_band_frame`] takes before a band's text.
pub(super) const CHROME_BAND_FRAME_STEPS: usize = 7;

//#endregion 📐️TimeTravelBand
//#region 📡️HistoryReprojectionBand
/// 🗝️ The mirror key of the polite status that announces a history change replaying before this replica adopts it, outside the
/// History panel — React's `[data-semio-history-reprojection]` (audit W1E-3).
pub(crate) const HISTORY_REPROJECTION_STATUS_ID: &str = "shell.history.reprojection";

/// 🧮️ The mirror key of that change's progress bar — React's `[data-semio-history-reprojection-progress]`.
pub(crate) const HISTORY_REPROJECTION_PROGRESS_ID: &str = "shell.history.reprojection.progress";

/// 🆔️ The reprojection band control's id: `shell.history.reprojection.<cancel-replay|rerun>`.
pub(crate) fn history_reprojection_control_id(verb: TimeTravelVerb) -> &'static str {
    match verb {
        TimeTravelVerb::Rerun => "shell.history.reprojection.rerun",
        _ => "shell.history.reprojection.cancel-replay",
    }
}

/// 🎨️ The reprojection band's border, fill and ink: a refused adoption warns, a replaying or paused change informs.
fn history_reprojection_band_tone(status: &HistoryReprojectionStatus, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    let (border, fill, text) = transient_notice_tone(if status.fault.is_some() && status.total == 0 { semio_framework::Severity::Warning } else { semio_framework::Severity::Info }, theme);
    (border, chrome_band_surface(fill, theme), text)
}

//#endregion 📡️HistoryReprojectionBand
//#region 🪟️TimeTravelIndicator
/// 🆔️ One pane's time-travel indicator — under React's `framework.window.<id>` parent like every pane chip.
pub(crate) fn time_travel_indicator_control_id(window_id: &str) -> String {
    semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "timeTravel", "indicator"])
}

//#endregion 🪟️TimeTravelIndicator
impl<'ctx> OverlaysViewCtx<'ctx> {
    /// ⏪️ Paints the persistent time-travel band, one scalar, glyph run or hit per opportunity: fill, edges, the replay
    /// track and its fill, each line of the message, then per button its fill, caption and hit. An enabled button's hit
    /// carries its verb, so a click, the keyboard ring and an assistive technology all dispatch it the same way; a
    /// disabled one paints muted, dispatches nothing and names the refusal that disables it. No button registers while a
    /// modal dialog (the finalize prompt) owns the pointer.
    pub(super) fn render_time_travel_band_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> bool {
        let Some(status) = self.history.history_time_travel.clone() else { return true };
        let plan = self.prefs.time_travel_band_plan_for(&status, theme, &self.chrome);
        let (border, fill, text_color) = time_travel_band_tone(&status, theme);
        let baseline = |rect: Rect| rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0;
        let buttons_from = CHROME_BAND_FRAME_STEPS + plan.lines.len();
        match cursor.scalar {
            scalar if scalar < CHROME_BAND_FRAME_STEPS => paint_chrome_band_frame(scalar, overlay, plan.band, plan.progress, (border, fill), theme),
            scalar if scalar < buttons_from => {
                let line = &plan.lines[scalar - CHROME_BAND_FRAME_STEPS];
                match chrome_text_complete_step(overlay, atlas, &line.text, line.rect.x, baseline(line.rect), line.rect.w, theme.font_size_small, text_color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.chrome.error = Some("Shell time-travel band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            scalar => {
                let Some(button) = plan.buttons.get((scalar - buttons_from) / 3) else { return true };
                let enabled = button.control.disabled_by.is_none();
                match (scalar - buttons_from) % 3 {
                    0 => overlay.push_rounded([button.rect.x, button.rect.y, button.rect.w, button.rect.h], if enabled { theme.button } else { theme.button.with_alpha(theme.button.a * 0.5) }, theme.border_radius),
                    1 => {
                        let ink = if enabled { theme.text } else { theme.text_muted };
                        match chrome_text_complete_step(overlay, atlas, &button.label, button.rect.x + theme.padding_standard, baseline(button.rect), (button.rect.w - theme.padding_standard).max(1.0), theme.font_size_small, ink, &mut cursor.glyph) {
                            Ok(false) => return false,
                            Ok(true) => {}
                            Err(()) => {
                                self.chrome.error = Some("Shell time-travel band button text exceeded the retained glyph boundary".to_string());
                                cursor.glyph.reset();
                            }
                        }
                    }
                    _ => {
                        if let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()).filter(|_| !self.chrome.chrome_build.dialog_open()) {
                            let locale = self.prefs.active_locale();
                            let control_id = button.control.verb.control_id();
                            note_chrome_control_name(control_id, Some(button.control.verb.label(self.prefs.band_tongue(&self.chrome))));
                            note_chrome_control_description(control_id, button.control.disabled_by.map(|label| time_travel_label(label, locale)));
                            note_chrome_control_disabled(control_id, !enabled);
                            input.register_hit(HitTarget { rect: button.rect, event: enabled.then(|| button.control.verb.action(&controller_id, &status)), control_id: Some(control_id.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                        }
                    }
                }
            }
        }
        cursor.scalar += 1;
        false
    }

    /// 🔊️ The band's live node: ONE polite status in every stage — its role never changes — named by the band's own message
    /// so a reader hears exactly what is painted, busy while the runtime replays or finalizes. It takes focus programmatically
    /// (never by Tab) — React's band region — so a replay or a review moves the reader onto the band. A running replay's
    /// numbers are [`Self::time_travel_progress_accessibility_node`].
    pub(crate) fn time_travel_status_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let status = self.history.history_time_travel.as_ref()?;
        let mut node = chrome_status_accessibility_node(node_id, TIME_TRAVEL_BAND_STATUS_ID, time_travel_band_lines(status, self.prefs.active_terminology(), self.prefs.band_tongue(&self.chrome)).message());
        node.focusable = true;
        node.focused = self.input.accessibility_focused_control_id.as_deref() == Some(TIME_TRAVEL_BAND_STATUS_ID);
        node.busy = matches!(status.stage, HistoryTimeTravelStage::Replaying | HistoryTimeTravelStage::Finalizing);
        Some(node)
    }

    /// 📶️ Preparation or replay progress as its own node under the band's status — React's `<progress>` inside the band: a progress
    /// bar named and valued by the progress line while known-total work runs, `None` otherwise.
    pub(crate) fn time_travel_progress_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let status = self.history.history_time_travel.as_ref()?;
        let total = status.total.filter(|_| time_travel_progress_key(status.stage).is_some())?;
        let line = time_travel_band_lines(status, self.prefs.active_terminology(), self.prefs.band_tongue(&self.chrome)).progress?;
        let mut node = chrome_status_accessibility_node(node_id, TIME_TRAVEL_BAND_PROGRESS_ID, line);
        node.role = "progressbar".into();
        node.depth = 1;
        node.live = ui_contract::liveness_name(ui_contract::Liveness::Off).to_string();
        node.value_min = Some(0.0);
        node.value_max = Some(f64::from(total));
        node.value_now = Some(f64::from(status.done.unwrap_or(0)));
        node.value_text = node.label.clone();
        Some(node)
    }

    /// 📡️ Paints the band of a history change replaying before adoption, one scalar, glyph run or hit per opportunity — the shared
    /// band frame with its replay track, each line of its message, then its control's fill, caption and hit (Cancel replay or Replay
    /// again on the session controller, no generation) — stacked above the session band while one is open, which then owns the
    /// controls. No control registers while a modal dialog owns the pointer.
    pub(super) fn render_history_reprojection_band_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme) -> bool {
        let Some((status, plan)) = self.prefs.history_reprojection_band_plan_for(theme, &self.chrome, &self.history) else { return true };
        let (border, fill, text_color) = history_reprojection_band_tone(&status, theme);
        let baseline = |rect: Rect| rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0;
        let control_from = CHROME_BAND_FRAME_STEPS + plan.lines.len();
        match cursor.scalar {
            scalar if scalar < CHROME_BAND_FRAME_STEPS => paint_chrome_band_frame(scalar, overlay, plan.band, plan.progress, (border, fill), theme),
            scalar if scalar < control_from => {
                let line = &plan.lines[scalar - CHROME_BAND_FRAME_STEPS];
                match chrome_text_complete_step(overlay, atlas, &line.text, line.rect.x, baseline(line.rect), line.rect.w, theme.font_size_small, text_color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.chrome.error = Some("Shell history reprojection band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            scalar => {
                let Some((verb, rect)) = plan.control.filter(|_| scalar < control_from + 3) else { return true };
                let tongue = self.prefs.band_tongue(&self.chrome);
                match scalar - control_from {
                    0 => overlay.push_rounded([rect.x, rect.y, rect.w, rect.h], theme.button, theme.border_radius),
                    1 => match chrome_text_complete_step(overlay, atlas, verb.label(tongue), rect.x + theme.padding_standard, baseline(rect), (rect.w - theme.padding_standard).max(1.0), theme.font_size_small, theme.text, &mut cursor.glyph) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => {
                            self.chrome.error = Some("Shell history reprojection band button text exceeded the retained glyph boundary".to_string());
                            cursor.glyph.reset();
                        }
                    },
                    _ => {
                        if let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()).filter(|_| !self.chrome.chrome_build.dialog_open()) {
                            let control_id = history_reprojection_control_id(verb);
                            note_chrome_control_name(control_id, Some(verb.label(tongue)));
                            input.register_hit(HitTarget { rect, event: Some(ActionDescriptor { controller_id, action: verb.action_id().to_string(), args: None }), control_id: Some(control_id.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                        }
                    }
                }
            }
        }
        cursor.scalar += 1;
        false
    }

    /// 🔊️ The reprojection band's live node (audit W1E-3): ONE polite status whether the change replays, is paused or was
    /// refused — its role never changes — named by exactly the painted, localized message `<title>: <text>` (a refusal's code is
    /// never announced) and busy while it replays, so a reader hears an interior Undo, a peer's history change or a document load
    /// replaying without opening the History panel. Its numbers are [`Self::history_reprojection_progress_accessibility_node`].
    pub(crate) fn history_reprojection_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let status = self.prefs.history_reprojection_status(&self.history)?;
        let mut node = chrome_status_accessibility_node(node_id, HISTORY_REPROJECTION_STATUS_ID, history_reprojection_message(&status));
        node.busy = history_reprojection_running(&status);
        Some(node)
    }

    /// 🧭️ A replaying history change's progress as its own node beside the status — React's `<progress>` of the reprojection
    /// status: named by the title, valued by done of total, its value text the status line; `None` while nothing replays.
    pub(crate) fn history_reprojection_progress_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let status = self.prefs.history_reprojection_status(&self.history).filter(history_reprojection_running)?;
        let mut node = chrome_status_accessibility_node(node_id, HISTORY_REPROJECTION_PROGRESS_ID, status.title.clone());
        node.role = "progressbar".into();
        node.live = ui_contract::liveness_name(ui_contract::Liveness::Off).to_string();
        node.value_min = Some(0.0);
        node.value_max = Some(f64::from(status.total));
        node.value_now = Some(f64::from(status.done));
        node.value_text = Some(status.text.clone());
        Some(node)
    }

    /// 📣️ Both bands' live nodes in reading order — each status followed by its progress — numbered from `first_id`.
    pub(crate) fn band_accessibility_nodes(&self, first_id: u64) -> Vec<ui_contract::AccessibilityProjectionNode> {
        let mut nodes = Vec::new();
        nodes.extend(self.time_travel_status_accessibility_node(first_id));
        nodes.extend(self.time_travel_progress_accessibility_node(first_id + nodes.len() as u64));
        nodes.extend(self.history_reprojection_accessibility_node(first_id + nodes.len() as u64));
        nodes.extend(self.history_reprojection_progress_accessibility_node(first_id + nodes.len() as u64));
        nodes
    }

    /// 🪟️ One paint opportunity of ONE pane's time-travel indicator, painted after the pane's own chips (`index` is
    /// their count) at the bottom-middle anchor none of them uses — React's `TimeTravelWindowIndicator`: a clock and the
    /// word "Time travel", announced as a note named by what the window shows. It dispatches nothing.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub(super) fn paint_window_time_travel_indicator_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, window_id: &str, window_rect: Rect, index: usize) -> bool {
        if cursor.scalar != index {
            return true;
        }
        let Some(status) = self.history.history_time_travel.as_ref() else { return true };
        let tongue = self.prefs.band_tongue(&self.chrome);
        let description = time_travel_indicator_text(status, self.prefs.active_terminology(), tongue);
        let control_id = time_travel_indicator_control_id(window_id);
        let item = ChromeGroupItem { control_id: control_id.as_str(), icon_id: Some("clock"), label: Some(time_travel_indicator_caption(tongue)), active: true, disabled: false, kind: HitKind::Generic };
        if cursor.rect.is_none() {
            let Some(width) = retained_chrome_group_item_width(atlas, theme, &item) else {
                self.chrome.error = Some("Shell time-travel indicator exceeded the retained chrome boundary".to_string());
                return true;
            };
            cursor.rect = Some(window_pane_chip_rect(theme, window_rect, PanelAnchor::BottomMiddle, width));
        }
        let Some(rect) = cursor.rect else { return true };
        if cursor.depth == 0 {
            cursor.depth = draw.push_glass([rect.x, rect.y, rect.w, rect.h], 0.0, theme.glass(Level::Window)).saturating_add(1);
        }
        draw.begin_glass_content(cursor.depth - 1);
        let step = render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false);
        draw.end_glass_content();
        match step {
            RetainedChromeGroupStep::Pending => return false,
            RetainedChromeGroupStep::Complete => {}
            RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell time-travel indicator exceeded the retained glyph boundary".to_string()),
        }
        note_chrome_control_name(&control_id, Some(&description));
        note_chrome_control_semantics(&control_id, &ChromeDialogSemantics { role: Some("note"), ..ChromeDialogSemantics::default() });
        self.input.pane_overlay_hits.push(HitTarget { rect, event: None, control_id: Some(control_id), kind: HitKind::Generic, drag_axis: None, drag_data: None });
        cursor.rect = None;
        cursor.depth = 0;
        cursor.scalar += 1;
        false
    }
}

