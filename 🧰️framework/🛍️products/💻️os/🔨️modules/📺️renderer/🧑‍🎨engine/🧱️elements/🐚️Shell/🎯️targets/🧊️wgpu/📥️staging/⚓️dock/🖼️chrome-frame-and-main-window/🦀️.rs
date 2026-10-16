//! ⚓️ `shell-dock` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl ShellDockState {
    /// 🗄️ Coalesces one bounded panel-layout page for the shared I/O lane, and ONLY when the dock
    /// actually moved — see [`Self::request_chrome_preferences_persist`] for why an unconditional
    /// arm is not "free".
    pub(super) fn request_panel_layout_persist(&mut self, chrome: &mut ChromeState) {
        if chrome.chrome_present.last_persisted_dock_ui.as_ref() != Some(&self.dock_ui_snapshot()) || chrome.chrome_present.last_persisted_dock_skeleton.as_ref() != Some(&self.dock_override) {
            chrome.chrome_present.maintenance.layout_requested = true;
        }
    }

    pub(super) fn advance_panel_layout_persist_step(&mut self, chrome: &mut ChromeState, docs: &DocumentState) {
        self.persist_dock_ui_if_changed(chrome, docs);
        chrome.chrome_present.maintenance.layout_requested = false;
    }

    /// 📑️ The `index`-th folded root tab of ONE anchor, in the anchor's own declared order.
    pub(super) fn anchor_tab_row_item(&self, anchor: PanelAnchor, index: usize) -> Option<(PanelAnchor, DockTabNode)> {
        self.dock_tabs.tabs(anchor).get(index).map(|node| (anchor, node.clone()))
    }

    pub(super) fn chrome_tab_band_width(&self, atlas: &mut FontAtlas, theme: &Theme, anchor: PanelAnchor) -> f32 {
        self.dock_tabs
            .tabs(anchor)
            .iter()
            .map(|node| {
                let item = ChromeGroupItem { control_id: "", icon_id: Some(node.icon_id.as_str()), label: Some(node.label.as_str()), active: false, disabled: false, kind: HitKind::Toggle };
                retained_panel_chrome_item_width(atlas, theme, &item).unwrap_or(0.0)
            })
            .sum()
    }

    /// 📐️ The height one anchor's panel WANTS: its tab bar plus the active leaf's measured document,
    /// plus the content inset the panel adds around that document. `None` while the leaf has no
    /// accepted layout yet (its very first frame), which keeps the anchor on its full column band
    /// rather than collapsing it to a hairline.
    pub(super) fn anchor_content_height(&self, anchor: PanelAnchor, theme: &Theme) -> Option<f32> {
        let leaf = self.anchor_state(anchor).active_tab()?;
        let document = crate::interpreter::retained_content_height(leaf)?;
        Some(self.anchor_tab_bar_height(anchor, theme) + document + theme.gap_standard * 2.0)
    }

    /// 🗣️ The per-INSTANCE tab label and icon the dock chrome reads, resolved off the app manifest's
    /// own `LocalizedLabel`/`IconName` through the live terminology and locale — no shell dictionary
    /// and no default language, the same rule React's `withLocalizedWindowKindLabels` follows.
    ///
    /// 🏷️ React's `windowTitlesById[instance.id] ?? instance.title` — an EXTRA instance wears the
    /// title its layout node authored (`Top`, `Perspective`), and only a bare kind falls back to
    /// the manifest label. The map is a SIDE map keyed by instance id, exactly like React's
    /// `extraWindowInstancesRef`, so a pane the user drags around keeps its authored title even
    /// though this renderer's own captured layout carries no titles on its window nodes.
    pub(crate) fn dock_chrome_maps(&self, prefs: &PrefsState, docs: &DocumentState, windows: &WindowState) -> (HashMap<String, String>, HashMap<String, String>) {
        let mut labels = HashMap::new();
        let mut icon_ids = HashMap::new();
        let Some(session) = &docs.session else {
            return (labels, icon_ids);
        };
        let terminology = prefs.active_terminology();
        let locale = prefs.active_locale();
        let mut instance_titles = session.app.default_layout.as_ref().map(crate::dock::window_layout_instance_titles).unwrap_or_default();
        instance_titles.extend(self.layout_override.as_ref().map(crate::dock::window_layout_instance_titles).unwrap_or_default());
        for (window_id, window_kind_id) in self.dock.window_instances() {
            let Some(kind) = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id) else {
                continue;
            };
            labels.insert(window_id.clone(), windows.window_title_override(&window_id, docs).map(str::to_string).or_else(|| instance_titles.get(&window_id).cloned()).unwrap_or_else(|| kind.label.resolve(terminology, locale).to_string()));
            let icon_id = dock_tab_icon_id_v1(kind.icon_id.as_str(), windows.window_icon_override(&window_id, docs), windows.world_projection_template.get(&window_id).map(String::as_str));
            if !icon_id.is_empty() {
                icon_ids.insert(window_id, icon_id);
            }
        }
        (labels, icon_ids)
    }

    /// 📑️ The rows this anchor's tab bar paints: the root row, then one row per branch the active path
    /// has drilled into and not folded shut — React's `buildPanelSelectionProps` row stack.
    pub(crate) fn anchor_tab_rows(&self, anchor: PanelAnchor) -> Vec<Vec<DockTabNode>> {
        let mut rows = vec![self.dock_tabs.tabs(anchor).to_vec()];
        let path = self.anchor_state(anchor).path.clone();
        let mut nodes = self.dock_tabs.tabs(anchor).to_vec();
        for id in &path {
            let Some(node) = nodes.iter().find(|node| &node.id == id).cloned() else { break };
            if !node.is_branch() || self.dock_branch_collapsed(anchor, &node.id) {
                break;
            }
            rows.push(node.children.clone());
            nodes = node.children;
        }
        rows
    }

    pub(crate) fn anchor_tab_bar_height(&self, anchor: PanelAnchor, theme: &Theme) -> f32 {
        self.anchor_panel_tab_rows(anchor).len() as f32 * theme.control_height
    }

    /// 📑️ Rows owned by the open panel body. A top/bottom anchor's root row stays in the bonded
    /// navbar/footer chrome, so the floating panel begins at the first selected branch row. Middle
    /// side anchors have no bonded shell row and therefore retain their root row here.
    pub(crate) fn anchor_panel_tab_rows(&self, anchor: PanelAnchor) -> Vec<Vec<DockTabNode>> {
        let rows = self.anchor_tab_rows(anchor);
        if matches!(anchor.vertical(), "top" | "bottom") {
            rows.into_iter().skip(1).collect()
        } else {
            rows
        }
    }

    /// 🧭️ The box the anchor's active leaf document paints into — the panel minus its tab bar.
    pub(crate) fn anchor_content_rect(&self, anchor: PanelAnchor, panel: Rect, theme: &Theme) -> Rect {
        let bar = self.anchor_tab_bar_height(anchor, theme);
        flowed_anchor_content_rect(anchor.flow().block, panel, bar, theme.gap_standard)
    }

    /// 🫳️ The x the insert preview paints at for one tab row, or `None` when the pointer is outside it.
    /// React's `computeTabDockDropZone` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`): the chip the pointer is over
    /// decides by its own half (`fraction >= 0.5` inserts after), and a pointer inside the row but past
    /// every chip inserts at the row's end. The dragged tab's own chip is excluded, exactly as React's
    /// `excludedIds` excludes the dragged subtree.
    #[allow(clippy::too_many_arguments, reason = "the tab row's own geometry, forwarded unchanged")]
    pub(super) fn dock_tab_insert_preview(&self, row: &[DockTabNode], atlas: &mut FontAtlas, theme: &Theme, row_rect: Rect, offset: f32, pointer_x: f32, pointer_y: f32) -> Option<f32> {
        if !row_rect.contains(pointer_x, pointer_y) {
            return None;
        }
        let dragged = self.dock_tab_drag.as_ref().map(|(tab_id, _)| tab_id.as_str());
        let mut x = row_rect.x - offset;
        for node in row {
            let item = ChromeGroupItem { control_id: node.id.as_str(), icon_id: Some(node.icon_id.as_str()), label: Some(node.label.as_str()), active: false, disabled: false, kind: HitKind::PanelTab };
            let chip_w = retained_panel_chrome_item_width(atlas, theme, &item).unwrap_or(theme.control_height);
            if dragged != Some(node.id.as_str()) && pointer_x >= x && pointer_x < x + chip_w {
                let fraction = if chip_w > 0.0 { (pointer_x - x) / chip_w } else { 0.5 };
                return Some(if fraction >= 0.5 { x + chip_w + theme.gap_standard } else { x });
            }
            x += chip_w + theme.gap_standard;
        }
        Some(x)
    }
}

//#endregion ShellChrome

