//! 🍱️ `shell-view-chrome` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome

#[derive(Default)]
pub struct ShellChromeChildCursor {
    pub empty_notice_lines: Vec<(std::ops::Range<usize>, Rect)>,
    pub empty_notice_text: &'static str,
    pub phase: u16,
    pub item: usize,
    pub scalar: usize,
    pub x: f32,
    pub right: f32,
    pub flag: bool,
    pub fault: bool,
    pub group_phase: u8,
    pub document: UiDocumentFrameCursor,
    pub window: Option<UiText>,
    pub rect: Option<Rect>,
    pub path: ShellUtilityPath,
    pub depth: usize,
    pub find_rejected: Option<ShellFindItem>,
    pub glyph: RetainedGlyphCursor,
}

impl<'ctx> ChromeViewCtx<'ctx> {
    /// ⏭️ Advances one bounded field or preview page and retains every remaining owner.
    ///
    /// 🏷️ React's `shouldPersistIntroductionSeen(brand)`: a `replayIntroductionOnLoad` or
    /// `ephemeral` brand answers its tour without ever writing a device-local flag, so the next
    /// load replays it. The in-memory answer still stands for THIS session (the tour does not
    /// come back on a republication) — that is React's `dismissedIntroductionAppIds` half.
    pub fn advance_chrome_maintenance_step(&mut self) -> bool {
        if !self.surfaces.retired_world3d_states.is_empty() {
            self.surfaces.advance_world3d_retirement_step();
        } else if self.chrome.chrome_present.maintenance.load_requested {
            self.docs.advance_chrome_preferences_load_step(&mut self.chrome, &mut self.prefs);
        } else if let Some(seen_key) = self.chrome.chrome_present.maintenance.introduction_read.take() {
            let key = format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}{seen_key}");
            let seen = stored_field_get(&key).as_deref() == Some("true");
            self.chrome.chrome_build.introduction_seen.insert(seen_key.clone(), seen);
            self.docs.auto_start_introduction(&seen_key, seen, &mut self.chrome, &self.tutorial);
        } else if let Some(seen_key) = self.chrome.chrome_present.maintenance.introduction_write.take() {
            if semio_framework_os_renderer_wgpu_host_base::boot_brand().persists_introduction_seen() {
                let key = format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}{seen_key}");
                stored_field_set(&key, "true");
            }
        } else if self.chrome.chrome_present.maintenance.layout_requested {
            self.dock.advance_panel_layout_persist_step(&mut self.chrome, &self.docs);
        } else if self.chrome.chrome_present.maintenance.presence_requested {
            self.advance_presence_preview_step();
        } else if self.chrome.chrome_present.maintenance.persist_requested {
            self.prefs.advance_chrome_preferences_persist_step(&mut self.chrome);
        }
        !self.surfaces.retired_world3d_states.is_empty() || self.chrome.chrome_present.maintenance.pending()
    }

    pub fn advance_presence_preview_step(&mut self) {
        self.chrome.chrome_present.maintenance.presence_requested = false;
        let Some(channel) = self.sync.sync_channel.as_ref() else { return };
        if channel.document_id.len() > SHELL_CHROME_IO_FIELD_BYTES || channel.plugin_id.len() > SHELL_CHROME_IO_FIELD_BYTES {
            return;
        }
        let actor = self.directory.current_shell_actor(channel.instance_id);
        if actor.len() > SHELL_CHROME_IO_FIELD_BYTES {
            return;
        }
        let _document_id = channel.document_id.clone();
        let connected_at_ms = channel.connected_at_ms;
        let label = Some(shell_presence_label(self.directory.identity.as_ref(), &actor)).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES);
        let user_id = self.directory.identity.as_ref().map(|identity| identity.user_id.clone()).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES);
        let (views, active_tool) = self.directory.board_presence_views(&self.surfaces);
        let ephemeral = self.plugins.plugins.iter().find(|entry| entry.plugin_id == channel.plugin_id).and_then(|plugin| plugin.ephemeral_snapshot(channel.instance_id)).unwrap_or_default();
        let (presence_pack, interaction) = (ephemeral.presence, ephemeral.interaction);
        let peer = PresencePeer { actor, label, presence_pack, connected_at_ms, user_id, role: None, drag_ghost_json: None, interaction, color: None, surface: None, views, ui: None, tool_run: ephemeral.tool_run, principal_kind: None, active_tool, history_edit: ephemeral.history_edit, typing: Vec::new() };
        self.sync.document_host.presence_heartbeat_key(&channel.document_key, chrome_now_ms() as u64, peer);
    }

    pub fn navbar_leading_tab_row_item(&self, index: usize) -> Option<(PanelAnchor, DockTabNode)> {
        self.dock.anchor_tab_row_item(PanelAnchor::TopLeft, index)
    }

    pub fn navbar_center_tab_row_item(&self, index: usize) -> Option<(PanelAnchor, DockTabNode)> {
        self.dock.anchor_tab_row_item(PanelAnchor::TopMiddle, index)
    }

    pub fn navbar_trailing_tab_row_item(&self, index: usize) -> Option<(PanelAnchor, DockTabNode)> {
        let tabs = self.dock.dock_tabs.tabs(PanelAnchor::TopRight);
        let last = tabs.len().checked_sub(1)?;
        tabs.get(last.checked_sub(index)?).cloned().map(|node| (PanelAnchor::TopRight, node))
    }

    pub fn footer_tab_row_item(&self, index: usize) -> Option<(PanelAnchor, DockTabNode)> {
        let mut seen = 0;
        for anchor in [PanelAnchor::BottomLeft, PanelAnchor::BottomMiddle, PanelAnchor::BottomRight] {
            let tabs = self.dock.dock_tabs.tabs(anchor);
            if index < seen + tabs.len() {
                return Some((anchor, tabs[index - seen].clone()));
            }
            seen += tabs.len();
        }
        None
    }

    pub fn navbar_control_band_width(&self, atlas: &mut FontAtlas, theme: &Theme, controls: &[ShellNavbarControl]) -> f32 {
        controls.iter().map(|control| self.navbar_control_width(atlas, theme, control).unwrap_or(0.0)).sum()
    }

    /// 📐️ The one width resolver used by both centred-band reservation and retained paint.
    pub fn navbar_control_width(&self, atlas: &mut FontAtlas, theme: &Theme, control: &ShellNavbarControl) -> Option<f32> {
        let label = self.chrome.chrome_control_label(&control.control_id, &control.label);
        let item = ChromeGroupItem { control_id: control.control_id.as_str(), icon_id: control.icon_id, label: Some(label.as_str()), active: control.active, disabled: false, kind: HitKind::NavbarItem };
        retained_chrome_group_item_width(atlas, theme, &item).map(|width| resolve_shell_navbar_control_width(width.max(theme.control_height), control.width_policy, theme.root_rem_pixels))
    }

    pub fn navbar_center_width(&self, atlas: &mut FontAtlas, theme: &Theme) -> f32 {
        let title = self.docs.session.as_ref().map_or_else(|| SHELL_NAVBAR_SESSIONLESS_TITLE.to_string(), |session| shell_navbar_title(&session.app, &self.prefs.terminology_id));
        let logo = theme.gap_standard * SHELL_NAVBAR_LOGO_UI_SPACING;
        let mut logo_title = logo + theme.gap_standard + atlas.measure_text(&title, theme.font_size_body).0 + theme.padding_standard * 2.0;
        if let Some(role) = self.docs.session.as_ref().map(|session| session.app.role) {
            let label = shell_surface_role_chip_text(role, self.prefs.locale_id == "de");
            let item = ChromeGroupItem { control_id: "", icon_id: None, label: Some(label), active: false, disabled: false, kind: HitKind::Button };
            logo_title += theme.gap_standard + retained_chrome_group_item_width(atlas, theme, &item).unwrap_or(0.0);
        }
        let rows = if semio_framework_os_renderer_wgpu_host_base::boot_locks().example_id.is_empty() { self.docs.session_example_rows(&self.plugins, &self.prefs) } else { Vec::new() };
        let examples: Vec<ShellNavbarControl> = shell_example_control(&rows, matches!(&self.input.overlay_state, OverlayState::Dropdown(id) if id == "example"), self.prefs.locale_id == "de").into_iter().collect();
        let modes = self.docs.session.as_ref().map(|session| shell_mode_controls(&session.app, session.view_state.active_mode_id.as_deref(), self.prefs.active_terminology(), self.prefs.active_locale())).unwrap_or_default();
        let roles = self.docs
            .session
            .as_ref()
            .and_then(|session| self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|entry| shell_role_controls(&entry.manifest.apps, &session.app, self.prefs.active_terminology(), self.prefs.active_locale(), &self.chrome.shortcut_table())))
            .unwrap_or_default();
        let widths = [
            logo_title,
            self.navbar_control_band_width(atlas, theme, &examples),
            self.navbar_control_band_width(atlas, theme, &modes),
            self.navbar_control_band_width(atlas, theme, &roles),
            self.dock.chrome_tab_band_width(atlas, theme, PanelAnchor::TopMiddle),
        ];
        let occupied = widths.into_iter().filter(|width| *width > 0.0).collect::<Vec<_>>();
        occupied.iter().sum::<f32>() + theme.gap_standard * 2.0 * occupied.len().saturating_sub(1) as f32
    }

    pub fn navbar_chrome_layout(&self, atlas: &mut FontAtlas, theme: &Theme, width: f32) -> ShellNavbarChromeLayout {
        let leading_right = theme.padding_standard + self.dock.chrome_tab_band_width(atlas, theme, PanelAnchor::TopLeft);
        let fullscreen_width = self.prefs.fullscreen_item_width(atlas, theme, &self.chrome);
        let top_right_width = self.dock.chrome_tab_band_width(atlas, theme, PanelAnchor::TopRight);
        let fullscreen_left = (width - theme.padding_standard - fullscreen_width).max(theme.padding_standard);
        let trailing_right = fullscreen_left - if top_right_width > 0.0 { theme.gap_standard } else { 0.0 };
        let trailing_left = (trailing_right - top_right_width).max(0.0);
        let occupied = [ShellChromeBandSpan { left: 0.0, right: leading_right }, ShellChromeBandSpan { left: trailing_left, right: width }];
        ShellNavbarChromeLayout { center: shell_chrome_centered_band(width, &occupied, self.navbar_center_width(atlas, theme)), trailing_right }
    }

    pub fn footer_band_width(&self, atlas: &mut FontAtlas, theme: &Theme, anchor: PanelAnchor) -> f32 {
        self.dock.chrome_tab_band_width(atlas, theme, anchor)
    }

    pub fn navbar_leading_tab_row_rect(&self, atlas: &mut FontAtlas, theme: &Theme, index: usize, lead_x: f32, btn_y: f32, btn_h: f32) -> Option<(PanelAnchor, DockTabNode, Rect)> {
        let (anchor, node) = self.navbar_leading_tab_row_item(index)?;
        let item = ChromeGroupItem { control_id: "", icon_id: Some(node.icon_id.as_str()), label: Some(node.label.as_str()), active: false, disabled: false, kind: HitKind::Toggle };
        let item_w = retained_panel_chrome_item_width(atlas, theme, &item)?;
        Some((anchor, node, Rect::new(lead_x.max(theme.padding_standard), btn_y, item_w, btn_h)))
    }

    /// 🧭️ This frame's box for one OPEN anchor: `anchor_panel_rect` plus the anchor's slot in its own
    /// column, so two open anchors in one column band it between them instead of painting over each
    /// other — and the anchor's MEASURED content height, so the panel hugs its content exactly as
    /// React's `<Panel>` does instead of filling its band.
    pub fn anchor_rect(&self, anchor: PanelAnchor, body: Rect, theme: &Theme) -> Rect {
        let body = panel_band(anchor, body, theme);
        let column: Vec<PanelAnchor> = ANCHOR_COLUMN_ORDER.into_iter().filter(|candidate| candidate.horizontal() == anchor.horizontal() && self.dock.anchor_open(*candidate)).collect();
        let index = column.iter().position(|candidate| *candidate == anchor).unwrap_or(0);
        anchor_panel_rect(anchor, self.dock.anchor_state(anchor).size, self.dock.anchor_content_height(anchor, theme), column.len().max(1), index, body, theme)
    }

    /// 🪟️ Solves this frame's dock layout into `dock_window_plan` and the three drop/pick registries
    /// (`dock_drop_bodies`, `dock_drop_tab_bars`, `window_content_rects`/`window_silhouettes`), which
    /// had no production writer at all before this call site existed. A spawned studio surface is the
    /// one non-dock case and stays a single full-bounds pane.
    ///
    /// 🈳️ A stack holding no tabs answers `stack_body_rects_with_silhouettes` with an EMPTY window id.
    /// It stays a DROP body — React's `WindowChrome` still renders for an empty stack — but it is no
    /// window: React's `activeDescriptor` is `undefined` there and its body renders nothing
    /// (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1212`). Carried into the plan it became a real window with an
    /// empty id: the engine-surface census reported `windows=["", "tool.fill", …]`, the chrome census
    /// published a `window:` surface, and the body registered a full-bounds `HitKind::ScrollRegion`
    /// with an empty control id over the whole canvas — the row that swallowed the journey's
    /// `zoom-wheel` notch (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    /// `📓️w12c-chords-and-camera-live.md` §4.3, `🗑️generated/w12c-parity-run-19/steps.json` step 18's
    /// `surfacesAdded: ["window:"]`).
    ///
    /// 📱️ React reads `UI_MOBILE_MEDIA_QUERY` off the VIEWPORT, not the dock canvas — see
    /// `crate::dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX`. One assignment per plan keeps every tab-width
    /// and hit-registration pass of this frame on the same answer.
    ///
    /// 🖼️ One derivation per frame: the mobile flat stack, the docked-out tree of a live drag, or
    /// the committed tree — everything below (plan, drop registries, chrome, hits) reads THIS.
    pub fn plan_dock_windows(&mut self, rect: Rect, theme: &Theme, atlas: &mut FontAtlas) {
        self.dock.dock_canvas_bounds = rect;
        self.dock.dock_axis_separator = theme.gap_standard;
        self.dock.dock.mobile = self.chrome.screen_w <= semio_framework_os_renderer_wgpu_dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX;
        self.dock.dock_view = self.dock.dock.render_view(self.dock.dock_drag.as_ref().map(|drag| &drag.payload));
        if self.plugins.space_mode && self.docs.spawned_ui.is_some() {
            self.dock.dock_window_plan = vec![("spawned".to_string(), rect)];
            self.dock.dock_drop_bodies = Vec::new();
            self.dock.dock_drop_tab_bars = Vec::new();
            self.dock.window_content_rects = HashMap::from([("spawned".to_string(), rect)]);
            self.dock.window_silhouettes = HashMap::new();
            return;
        }
        let (labels, _icon_ids) = self.dock.dock_chrome_maps(&self.prefs, &self.docs, &self.windows);
        for (window_id, label) in &labels {
            note_dock_control_name(window_id, semio_framework_os_renderer_wgpu_dock::DockControlName::Body(label), self.prefs.locale_id == "de");
        }
        let (bodies, silhouettes) = self.dock.dock_view.stack_body_rects_with_silhouettes(rect, theme, &labels, atlas);
        self.dock.dock_drop_tab_bars = self.dock.dock_view.stack_corner_tab_bar_rects(rect, theme, atlas, &labels);
        self.dock.dock_window_plan = bodies.iter().filter(|(_, _, window_id)| !window_id.is_empty()).map(|(_, body, window_id)| (window_id.clone(), *body)).collect();
        self.dock.window_content_rects = bodies.iter().filter(|(_, _, window_id)| !window_id.is_empty()).map(|(_, body, window_id)| (window_id.clone(), *body)).collect();
        self.dock.window_silhouettes = silhouettes;
        self.dock.dock_drop_bodies = bodies;
        let plan = self.dock.dock_window_plan.iter().map(|(id, body)| format!("{id}@{}x{}+{},{}", body.w.round(), body.h.round(), body.x.round(), body.y.round())).collect::<Vec<_>>().join(" ");
        if self.dock.dock_plan_trace.as_deref() != Some(plan.as_str()) {
            debug_log(&format!("[TRACE] wgpu-shell dock plan canvas={}x{} windows={} {plan}", rect.w.round(), rect.h.round(), self.dock.dock_window_plan.len()));
            self.dock.dock_plan_trace = Some(plan);
        }
    }

    /// 🎯️ Completes a tab drag: the anchor under the pointer wins, and a drop back on the tab's own
    /// anchor is a no-op — React's `handleTabDockDrop` over `computeTabDockDropZone`'s anchor pick.
    ///
    /// 🫳️ The index the preview pill painted, so what drops is what was shown: the chip half the
    /// pointer released over, or the row's end.
    pub fn finish_dock_tab_drag(&mut self, tab_id: &str, from_anchor: PanelAnchor, x: f32, y: f32, theme: &Theme) {
        let Some(target_anchor) = self.anchor_at_point(x, y, theme) else { return };
        if target_anchor == from_anchor {
            return;
        }
        let index = self.dock_tab_drop_index(target_anchor, x, y, theme).unwrap_or_else(|| self.dock.dock_tabs.tabs(target_anchor).len());
        if self.as_session().move_dock_tab(tab_id, &DockTabMoveTarget { anchor: target_anchor, parent_path: Vec::new(), index }) {
            self.dock.anchor_state_mut(target_anchor).visible = true;
            debug_log(&format!("[TRACE] wgpu-shell dock tab {tab_id} re-anchored {} -> {}", from_anchor.as_str(), target_anchor.as_str()));
        }
    }

    /// 🫳️ The ROOT-ROW index a drop at `(x, y)` lands at inside `anchor`, over the same chip-half rule
    /// the insert preview paints — `None` when the point is not on that anchor's own root tab row (a
    /// drop anywhere else in the anchor appends, React's own `index: buttons.length` fallback).
    pub fn dock_tab_drop_index(&mut self, anchor: PanelAnchor, x: f32, y: f32, theme: &Theme) -> Option<usize> {
        if !self.dock.anchor_open(anchor) {
            return None;
        }
        let panel = self.anchor_rect(anchor, self.as_session().body_rect(theme), theme);
        let rows = self.dock.anchor_tab_rows(anchor);
        let row_y = flowed_anchor_tab_row_y(anchor.flow().block, panel, 0, rows.len(), theme.control_height);
        let row_rect = Rect::new(panel.x, row_y, panel.w, theme.control_height);
        if !row_rect.contains(x, y) {
            return None;
        }
        let row = self.dock.dock_tabs.tabs(anchor);
        let dragged = self.dock.dock_tab_drag.as_ref().map(|(tab_id, _)| tab_id.as_str());
        let mut cursor = panel.x + theme.gap_standard;
        let mut atlas = FontAtlas::builtin();
        for (index, node) in row.iter().enumerate() {
            let item = ChromeGroupItem { control_id: node.id.as_str(), icon_id: Some(node.icon_id.as_str()), label: Some(node.label.as_str()), active: false, disabled: false, kind: HitKind::PanelTab };
            let chip_w = retained_panel_chrome_item_width(&mut atlas, theme, &item).unwrap_or(theme.control_height);
            if dragged != Some(node.id.as_str()) && x >= cursor && x < cursor + chip_w {
                let fraction = if chip_w > 0.0 { (x - cursor) / chip_w } else { 0.5 };
                return Some(if fraction >= 0.5 { index + 1 } else { index });
            }
            cursor += chip_w + theme.gap_standard;
        }
        Some(row.len())
    }

    /// 🧭️ Which anchor a screen point belongs to: an open anchor's own painted box first, then the
    /// body quadrant the point falls in, so a tab can be dropped onto a FOLDED anchor too.
    pub fn anchor_at_point(&mut self, x: f32, y: f32, theme: &Theme) -> Option<PanelAnchor> {
        let body = self.as_session().body_rect(theme);
        for anchor in self.dock.open_anchors() {
            if self.anchor_rect(anchor, body, theme).contains(x, y) {
                return Some(anchor);
            }
        }
        if !body.contains(x, y) {
            return None;
        }
        let third = |value: f32, origin: f32, extent: f32| ((value - origin) / (extent / 3.0)).floor().clamp(0.0, 2.0) as u8;
        let column = third(x, body.x, body.w);
        let row = third(y, body.y, body.h);
        Some(match (column, row) {
            (0, 0) => PanelAnchor::TopLeft,
            (0, 1) => PanelAnchor::LeftMiddle,
            (0, _) => PanelAnchor::BottomLeft,
            (1, 0) => PanelAnchor::TopMiddle,
            (1, 1) => return None,
            (1, _) => PanelAnchor::BottomMiddle,
            (_, 0) => PanelAnchor::TopRight,
            (_, 1) => PanelAnchor::RightMiddle,
            (_, _) => PanelAnchor::BottomRight,
        })
    }

    /// 📑️ Paints and hit-tests one anchor's tab bar: a branch row toggles open/shut, a leaf row selects,
    /// and every row is a `PanelTab` drag source so a tab can be carried to another anchor.
    ///
    /// 🫳️ While a tab drag is live, the row under the pointer paints the INSERT PREVIEW — React's
    /// `panelTabInsertPreviewClass` (`🧭️PanelTabBar/🟦️.tsx`), a `w-0.5` accent pill the height of the
    /// row, drawn before the chip the drop would land in front of (and after the last chip when the
    /// drop index is past the end). The index is React's own rule: the half of the chip the pointer is
    /// over (`fraction >= 0.5` → after), or the row's length when the pointer is past every chip.
    pub fn paint_anchor_tab_bar(&mut self, anchor: PanelAnchor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, panel: Rect) {
        let rows = self.dock.anchor_panel_tab_rows(anchor);
        let active: Vec<String> = self.dock.anchor_state(anchor).path.clone();
        let active_offset = usize::from(matches!(anchor.vertical(), "top" | "bottom"));
        let row_h = theme.control_height;
        let dragging = self.dock.dock_tab_drag.is_some();
        for (row_index, row) in rows.iter().enumerate() {
            let y = flowed_anchor_tab_row_y(anchor.flow().block, panel, row_index, rows.len(), row_h);
            let row_rect = Rect::new(panel.x + theme.gap_standard, y, (panel.w - theme.gap_standard * 2.0).max(0.0), row_h);
            let total_width = row
                .iter()
                .map(|node| {
                    let item = ChromeGroupItem { control_id: node.id.as_str(), icon_id: Some(node.icon_id.as_str()), label: Some(node.label.as_str()), active: false, disabled: false, kind: HitKind::PanelTab };
                    retained_panel_chrome_item_width(atlas, theme, &item).unwrap_or(theme.control_height)
                })
                .sum::<f32>()
                + theme.gap_standard * row.len().saturating_sub(1) as f32;
            let max_offset = (total_width - row_rect.w).max(0.0);
            let scroll_id = panel_tab_scroll_id(anchor, row_index);
            let offset = self.input.scroll_offsets.get(&scroll_id).copied().unwrap_or_default().clamp(0.0, max_offset);
            self.input.scroll_offsets.insert(scroll_id.clone(), offset);
            input.register_hit(HitTarget { rect: row_rect, event: None, control_id: Some(scroll_id), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
            let mut x = row_rect.x - offset;
            let preview = dragging.then(|| self.dock.dock_tab_insert_preview(row, atlas, theme, row_rect, offset, input.pointer_x, input.pointer_y)).flatten();
            draw.push_scissor(row_rect);
            for node in row {
                if preview == Some(x) {
                    draw.push_rounded([x, y + theme.gap_standard, TAB_INSERT_PREVIEW_WIDTH_PX, (row_h - theme.gap_standard * 2.0).max(0.0)], theme.accent, TAB_INSERT_PREVIEW_WIDTH_PX * 0.5);
                }
                let is_active = active.get(row_index + active_offset).is_some_and(|id| id == &node.id);
                let item = ChromeGroupItem { control_id: node.id.as_str(), icon_id: Some(node.icon_id.as_str()), label: Some(node.label.as_str()), active: is_active, disabled: false, kind: HitKind::PanelTab };
                let chip_w = retained_panel_chrome_item_width(atlas, theme, &item).unwrap_or(theme.control_height);
                let chip = Rect::new(x, y + (row_h - theme.control_height) * 0.5, chip_w, theme.control_height);
                if is_active {
                    draw.push_rounded([chip.x, chip.y, chip.w, chip.h], theme.accent, theme.border_radius);
                }
                let fg = if is_active { theme.active_foreground } else { theme.text_muted };
                let icon_id = if node.is_branch() && self.dock.dock_branch_collapsed(anchor, &node.id) { "chevron-right" } else { node.icon_id.as_str() };
                chrome_icon(draw, icons, icon_id, chip.x + theme.padding_standard, chip.y + (chip.h - PANEL_CHROME_ICON_TINY) * 0.5, PANEL_CHROME_ICON_TINY, fg);
                chrome_text(draw, atlas, input, theme, &node.label, chip.x + theme.padding_standard + PANEL_CHROME_ICON_TINY + theme.gap_standard, chip.y + (chip.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, fg);
                chrome_icon(draw, icons, "grip-vertical", chip.x + chip.w - theme.padding_standard - PANEL_CHROME_GRIP_SIZE, chip.y + (chip.h - PANEL_CHROME_GRIP_SIZE) * 0.5, PANEL_CHROME_GRIP_SIZE, fg);
                let left = chip.x.max(row_rect.x);
                let top = chip.y.max(row_rect.y);
                let right = (chip.x + chip.w).min(row_rect.x + row_rect.w);
                let bottom = (chip.y + chip.h).min(row_rect.y + row_rect.h);
                if right > left && bottom > top {
                    let visible = Rect::new(left, top, right - left, bottom - top);
                    note_chrome_group_item(&item);
                    input.register_hit(HitTarget { rect: visible, event: None, control_id: Some(node.id.clone()), kind: HitKind::PanelTab, drag_axis: Some(DragAxis::Both), drag_data: None });
                    self.chrome.chrome_build.register_element_rect(panel_tab_introduction_element_id(&node.id), visible);
                }
                x += chip_w + theme.gap_standard;
            }
            if preview == Some(x) {
                draw.push_rounded([x, y + theme.gap_standard, TAB_INSERT_PREVIEW_WIDTH_PX, (row_h - theme.gap_standard * 2.0).max(0.0)], theme.accent, TAB_INSERT_PREVIEW_WIDTH_PX * 0.5);
            }
            draw.pop_scissor();
        }
        let divider_y = flowed_anchor_tab_divider_y(anchor.flow().block, panel, rows.len() as f32 * row_h);
        let divider_y = if anchor.flow().block.is_reversed() { divider_y } else { divider_y - theme.stroke_hairline };
        draw.push_solid([panel.x, divider_y, panel.w, theme.stroke_hairline], theme.border_normal);
    }

    #[allow(clippy::too_many_arguments, reason = "one retained panel document opportunity forwards the host frame context")]
    pub fn render_panel_document_opportunity(
        &mut self,
        cursor: &mut ShellChromeChildCursor,
        window: &str,
        content: Rect,
        panel_draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        anchor: PanelAnchor,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> Option<bool> {
        let controller = self.docs.document_controller_id(&self.plugins);
        let document = self.docs.panel_documents.remove(window)?;
        semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::set_ui_document_flow(window, anchor.flow());
        let complete = {
            let scroll_offsets = &mut self.input.scroll_offsets;
            let collapsed_sections = &mut self.input.collapsed_sections;
            let open_selects = &mut self.input.open_selects;
            let widget_maps = &mut self.input.widget_maps_staging;
            let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.prefs.locale_id == "de"), world3d_states: &mut self.surfaces.world3d_states, world_resources, window_id: window };
            let viewport_height_for_widgets = panel_draw.screen_height();
            let mut ctx = framework_widget_context(panel_draw, overlay, atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
            ctx.pick_clip = Some(content);
            render_ui_document_step(&mut cursor.document, &document, content, &mut ctx, window, controller.as_str(), self.chrome.chrome_build.driver.drag, &mut hosts, cx)
        };
        self.docs.panel_documents.insert(window.to_string(), document);
        Some(complete)
    }

    /// 🧭️ One OPEN anchor's floating panel: its own tab bar (branch toggles and leaf tabs, hit-tested and
    /// draggable between anchors), then the active leaf's retained document, then its resize handle — the
    /// wgpu twin of React's `<Panel anchor=… />` from the `ANCHORS.map` loop in `📐️Layout/🟦️.tsx`.
    ///
    /// 🧭️ React wraps every anchored `Panel` in a `FlowProvider` keyed by its anchor
    /// (`🖼️Panel/🟦️.tsx:516`); this is that provider. An `End`-anchored panel therefore
    /// mirrors its overlays, its Select inline edge and its inline arrow keys, exactly as
    /// React's does — the seam `Ui::set_window_flow` was built for and never given
    /// (ticket 26/09/17 packet W15a).
    ///
    /// ↔ A corner or edge-middle anchor grows inward, so its one handle sits on the
    /// canvas-facing edge; a middle-column anchor is centred and grows both ways, so it
    /// carries one on each edge — React's `resizeSides` in `🖼️Panel/🟦️.tsx`.
    pub fn render_panel_step(
        &mut self,
        cursor: &mut ShellChromeChildCursor,
        anchor: PanelAnchor,
        panel_draw: &mut DrawList,
        overlay: Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        body: Rect,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> bool {
        match cursor.phase {
            0 => {
                cursor.rect = Some(self.anchor_rect(anchor, body, theme));
                cursor.window = self.dock.anchor_state(anchor).active_tab().and_then(UiText::try_from_str);
                cursor.flag = cursor.window.as_ref().is_none_or(|window| !self.docs.panel_documents.contains_key(window.as_str()));
                cursor.phase = 1;
            }
            1 => {
                let Some(window) = cursor.window.clone() else {
                    cursor.flag = true;
                    cursor.phase = 2;
                    return false;
                };
                if !cursor.flag {
                    let Some(panel) = cursor.rect else {
                        cursor.phase = u16::MAX;
                        return false;
                    };
                    let content = self.dock.anchor_content_rect(anchor, panel, theme);
                    let _ = self.render_panel_document_opportunity(cursor, window.as_str(), content, panel_draw, overlay, atlas, icons, input, theme, world_resources, anchor, cx);
                    if !cursor.document.layout_is_accepted() && !cursor.document.terminal_is_fault() {
                        return false;
                    }
                    if cursor.document.layout_is_accepted() {
                        let measured = self.anchor_rect(anchor, body, theme);
                        if measured != panel {
                            cursor.document.restart_viewport_after_host_reflow();
                        }
                        cursor.rect = Some(measured);
                    }
                    cursor.flag = true;
                }
                let Some(panel) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                cursor.scalar = panel_draw.push_glass([panel.x, panel.y, panel.w, panel.h], theme.border_radius, theme.glass(Level::Panel));
                cursor.phase = 2;
            }
            2 => {
                panel_draw.begin_glass_content(cursor.scalar);
                cursor.phase = 3;
            }
            3..=6 => {
                let Some(panel) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let hair = theme.stroke_hairline;
                let rect = match cursor.phase {
                    3 => [panel.x, panel.y, panel.w, hair],
                    4 => [panel.x, panel.y + panel.h - hair, panel.w, hair],
                    5 => [panel.x, panel.y, hair, panel.h],
                    _ => [panel.x + panel.w - hair, panel.y, hair, panel.h],
                };
                panel_draw.push_solid(rect, theme.border_normal);
                cursor.phase += 1;
            }
            7 => {
                let Some(panel) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                self.paint_anchor_tab_bar(anchor, panel_draw, atlas, icons, input, theme, panel);
                let content = self.dock.anchor_content_rect(anchor, panel, theme);
                panel_draw.push_scissor(content);
                cursor.phase = 8;
            }
            8 => {
                let Some(window) = cursor.window.clone() else {
                    cursor.phase = 9;
                    return false;
                };
                let Some(panel) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                if !self.docs.panel_documents.contains_key(window.as_str()) {
                    cursor.phase = 9;
                    return false;
                }
                let content = self.dock.anchor_content_rect(anchor, panel, theme);
                let complete = self.render_panel_document_opportunity(cursor, window.as_str(), content, panel_draw, overlay, atlas, icons, input, theme, world_resources, anchor, cx).unwrap_or(false);
                if !complete {
                    if !cursor.document.terminal_is_fault() {
                        return false;
                    }
                    self.as_session().record_document_paint_fault(window.as_str());
                } else {
                    self.as_session().note_retained_body_painted(window.as_str());
                }
                self.input.register_retained_panel_scroll_region(window.as_str(), content, input);
                self.input.register_retained_body_hits(window.as_str(), content, input);
                cursor.phase = 9;
            }
            9 => {
                panel_draw.pop_scissor();
                cursor.phase = 10;
            }
            10 => {
                panel_draw.end_glass_content();
                cursor.phase = 11;
            }
            11 => {
                let Some(panel) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let sides: &[(bool, &str)] = match anchor.horizontal() {
                    "left" => &[(false, "outer")],
                    "right" => &[(true, "inner")],
                    _ => &[(true, "inner"), (false, "outer")],
                };
                for &(left, suffix) in sides {
                    let id = format!("panel.resize.{}.{suffix}", anchor.as_str());
                    let handle = if left { Rect::new(panel.x, panel.y, theme.panel_inset, panel.h) } else { Rect::new(panel.x + panel.w - theme.panel_inset, panel.y, theme.panel_inset, panel.h) };
                    input.register_hit(HitTarget { rect: handle, event: None, control_id: Some(id.into()), kind: HitKind::PanelResize, drag_axis: Some(DragAxis::Horizontal), drag_data: None });
                }
                cursor.phase = 12;
            }
            12 => return true,
            _ => return false,
        }
        false
    }

    /// 📱️ The ONE mobile panel every anchor flattens into below React's breakpoint — React's
    /// `LayoutMobilePanel` (`🖱️ui/🧱️elements/📐️Layout/🟦️.tsx`): not a floating box but an in-flow pane
    /// filling the body while the canvas is hidden, carrying one tab row over
    /// {@link ShellState::mobile_panel_tabs} and the active leaf's retained document beneath it.
    /// Toggled by `ui.mobilePanel.toggle`, invisible until then, exactly as React's own
    /// `mobilePanelVisible` gate.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub fn render_mobile_panel_step(
        &mut self,
        cursor: &mut ShellChromeChildCursor,
        panel_draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        body: Rect,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> bool {
        let tabs = self.as_session().mobile_panel_tabs();
        if !self.dock.mobile_panel_visible || tabs.is_empty() {
            return true;
        }
        match cursor.phase {
            0 => {
                let path = self.dock.mobile_panel_path.clone();
                let path = mobile_panel_reconcile_path(&tabs, &path);
                self.dock.mobile_panel_path = path.clone();
                cursor.rect = Some(body);
                cursor.window = path.last().and_then(|id| UiText::try_from_str(id));
                cursor.phase = 1;
            }
            1 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                panel_draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.panel);
                self.paint_mobile_tab_bar(panel_draw, atlas, icons, input, theme, rect, &tabs);
                cursor.phase = 2;
            }
            2 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let Some(window) = cursor.window.clone() else {
                    cursor.phase = 3;
                    return false;
                };
                let content = Rect::new(rect.x, rect.y + theme.control_height, rect.w, (rect.h - theme.control_height).max(0.0)).inset(theme.gap_standard);
                let controller = self.docs.document_controller_id(&self.plugins);
                let Some(document) = self.docs.panel_documents.remove(window.as_str()) else {
                    cursor.phase = 3;
                    return false;
                };
                let complete = {
                    let scroll_offsets = &mut self.input.scroll_offsets;
                    let collapsed_sections = &mut self.input.collapsed_sections;
                    let open_selects = &mut self.input.open_selects;
                    let widget_maps = &mut self.input.widget_maps_staging;
                    let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.prefs.locale_id == "de"), world3d_states: &mut self.surfaces.world3d_states, world_resources, window_id: window.as_str() };
                    let viewport_height_for_widgets = panel_draw.screen_height();
                    let mut ctx = framework_widget_context(panel_draw, None, atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
                    ctx.pick_clip = Some(content);
                    render_ui_document_step(&mut cursor.document, &document, content, &mut ctx, window.as_str(), controller.as_str(), self.chrome.chrome_build.driver.drag, &mut hosts, cx)
                };
                self.docs.panel_documents.insert(window.as_str().to_string(), document);
                if !complete {
                    if !cursor.document.terminal_is_fault() {
                        return false;
                    }
                    self.as_session().record_document_paint_fault(window.as_str());
                } else {
                    self.as_session().note_retained_body_painted(window.as_str());
                }
                self.input.register_retained_panel_scroll_region(window.as_str(), content, input);
                self.input.register_retained_body_hits(window.as_str(), content, input);
                cursor.phase = 3;
            }
            3 => return true,
            _ => return false,
        }
        false
    }

    /// 📱️ The mobile panel's single tab row: every flattened tab as one chip, registering the same
    /// `shell.panel.tab.mobile.<tabId>` ids the mobile path reader answers.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub fn paint_mobile_tab_bar(&self, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, panel: Rect, tabs: &[DockTabNode]) {
        let mut x = panel.x + theme.gap_standard;
        let active = self.dock.mobile_panel_path.first().cloned().unwrap_or_default();
        for node in tabs {
            let label_w = atlas.measure_text(&node.label, theme.font_size_small).0;
            let chip_w = theme.padding_standard * 2.0 + CHROME_ICON_TINY + theme.gap_standard + label_w;
            if x + chip_w > panel.x + panel.w {
                break;
            }
            let chip = Rect::new(x, panel.y, chip_w, theme.control_height);
            let is_active = node.id == active;
            if is_active {
                draw.push_rounded([chip.x, chip.y, chip.w, chip.h], theme.accent, theme.border_radius);
            }
            let fg = if is_active { theme.active_foreground } else { theme.text_muted };
            chrome_icon(draw, icons, node.icon_id.as_str(), chip.x + theme.padding_standard, chip.y + (chip.h - CHROME_ICON_TINY) * 0.5, CHROME_ICON_TINY, fg);
            chrome_text(draw, atlas, input, theme, &node.label, chip.x + theme.padding_standard + CHROME_ICON_TINY + theme.gap_standard, chip.y + (chip.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, fg);
            let control_id = format!("shell.panel.tab.mobile.{}", node.id);
            note_chrome_group_item(&ChromeGroupItem { control_id: &control_id, icon_id: None, label: Some(&node.label), active: is_active, disabled: false, kind: HitKind::PanelTab });
            input.register_hit(HitTarget { rect: chip, event: None, control_id: Some(control_id), kind: HitKind::PanelTab, drag_axis: None, drag_data: None });
            x += chip_w + theme.gap_standard;
        }
        draw.push_solid([panel.x, panel.y + theme.control_height - theme.stroke_hairline, panel.w, theme.stroke_hairline], theme.border_normal);
    }
}

/// 📐️ Extra vertical space a CHROME-HOSTED open panel overhangs its band by, the Rust twin of
/// React's `chromeHostedOpenPanelPositionStyle` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6894`):
/// `calc(-1 * (var(--size-large) + var(--size-medium)) / 2)` — from the body's edge back to the
/// centred `h-medium` row inside the `h-large` navbar/footer band, where `PanelChromeTabBar`
/// parked the folded toggle. So an open panel's own cap sits IN the chrome band, not below it.
pub fn chrome_hosted_panel_overhang(theme: &Theme) -> f32 {
    (theme.navbar_height + theme.control_height) * 0.5
}

/// 🧭️ The band an anchored panel may occupy: React's mode body, with the anchored vertical edge
/// pulled `chrome_hosted_panel_overhang` INTO the navbar (a top anchor) or the footer (a bottom
/// anchor) and its height grown by exactly that overhang — React's `maxHeight:
/// calc(100% + (var(--size-large) + var(--size-medium)) / 2)`. Measured on React's own DOM
/// (`🗑️generated/w14d-cap-occlusion.json`, 1600×1000): mode body `y=29`, open Catalogue panel
/// `y=3 h=168`, its `panel-body-stack` from `y=26`.
///
/// 🩸️ This renderer used to open a panel at `body.y` and then, when that put the panel's own tab
/// row on the dock's cap strip, to push the whole panel BELOW the dock's stack tab bar
/// (`📓️w9c-behaviour-parity-run-2.md` steps 16–17). Both are divergences: React solves the same
/// collision by pulling the panel UP, so its cap lands in the navbar band (`y=3..26`) and only its
/// BODY covers the dock caps (`y=32..61`) — which is why React's window-cap steps journal nothing
/// at all, they press the panel's body (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w13a`§4).
/// Pushing the panel down instead made the two renderers lay out two different applications and
/// the cap steps compare them.
///
/// 📐️ `anchor_panel_rect` re-applies `inset` inside this band, so the band carries `-inset` on
/// the grown edge to land the panel on React's exact `-overhang` and `available_h` on its exact
/// `100% + overhang`. A middle anchor has no bonded edge and keeps the body it centres on.
pub fn panel_band(anchor: PanelAnchor, body: Rect, theme: &Theme) -> Rect {
    let overhang = chrome_hosted_panel_overhang(theme);
    let inset = theme.panel_inset;
    let root_row = theme.control_height;
    match anchor.vertical() {
        "top" => Rect::new(body.x, body.y - overhang - inset + root_row, body.w, (body.h + overhang + inset * 2.0 - root_row).max(0.0)),
        "bottom" => Rect::new(body.x, body.y - inset, body.w, (body.h + overhang + inset * 2.0 - root_row).max(0.0)),
        _ => body,
    }
}

//#endregion ShellChrome

