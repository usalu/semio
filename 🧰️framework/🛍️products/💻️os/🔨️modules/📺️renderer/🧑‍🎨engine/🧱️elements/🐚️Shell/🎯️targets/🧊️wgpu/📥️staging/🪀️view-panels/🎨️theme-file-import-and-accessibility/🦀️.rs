//! 🪀️ `shell-view-panels` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ♿️ChromeAccessibleNames
/// ♿️ The ARIA role a chrome hit announces — React's own element choice per control kind.
fn chrome_accessibility_role(kind: &HitKind) -> &'static str {
    match kind {
        HitKind::Toggle => "switch",
        HitKind::Select => "combobox",
        HitKind::DropdownItem => "option",
        HitKind::PanelTab => "button",
        HitKind::Slider => "slider",
        HitKind::Input => "textbox",
        HitKind::ContextMenu => "menuitem",
        _ => "button",
    }
}

impl<'ctx> PanelsViewCtx<'ctx> {
    pub fn palette_accessibility_nodes(&mut self, hits: &[HitTarget<ActionDescriptor>]) -> Option<Vec<ui_contract::AccessibilityProjectionNode>> {
        let kind = match self.input.overlay_state {
            OverlayState::Search => ShellPaletteKind::Search,
            OverlayState::Find => ShellPaletteKind::Find,
            _ => return None,
        };
        let is_de = self.prefs.locale_id == "de";
        let title = shell_chrome_string(if kind == ShellPaletteKind::Search { "search.title" } else { "find.title" }, is_de).to_string();
        let description = shell_chrome_string(if kind == ShellPaletteKind::Search { "search.description" } else { "find.description" }, is_de).to_string();
        let empty = shell_chrome_string(if kind == ShellPaletteKind::Search { "search.empty" } else { "find.empty" }, is_de).to_string();
        let query = self.commands.palette_query(kind).to_string();
        let selected_index = match kind {
            ShellPaletteKind::Search => self.commands.search_selected,
            ShellPaletteKind::Find => self.commands.find_selected,
        };
        let rows: Vec<(usize, String, String, Option<String>, String)> = match kind {
            ShellPaletteKind::Search => self.as_session().filtered_search_items().into_iter().enumerate().map(|(index, item)| (index, item.id, item.label, item.description, item.group)).collect(),
            ShellPaletteKind::Find => self.commands.filtered_find_items().into_iter().enumerate().map(|(index, item)| (index, item.id, item.label, item.description, item.category.unwrap_or_default())).collect(),
        };
        let rect_for = |id: &str| hits.iter().find(|hit| hit.control_id.as_deref() == Some(id)).map(|hit| hit.rect);
        let dialog_rect = rect_for(kind.dialog_id());
        let input_rect = rect_for(kind.input_id());
        let list_rect = dialog_rect.zip(input_rect).map(|(dialog, input)| Rect::new(dialog.x, input.y + input.h, dialog.w, (dialog.y + dialog.h - input.y - input.h).max(0.0)));
        let basic = |key: String, role: &str, depth: usize, label: Option<String>, description: Option<String>, rect: Option<Rect>| ui_contract::AccessibilityProjectionNode {
            value_step: None,
            invalid: false,
            set_size: None,
            pos_in_set: None,
            tone: None,
            node_id: 0,
            key,
            role: role.to_string(),
            depth,
            label,
            description,
            live: ui_contract::liveness_name(ui_contract::Liveness::Off).to_string(),
            shortcut: None,
            hidden: false,
            disabled: false,
            focusable: false,
            tabbable: false,
            actionable: false,
            focused: false,
            checked: None,
            pressed: None,
            selected: None,
            expanded: None,
            editable: false,
            multiline: false,
            controls: None,
            active_descendant: None,
            level: None,
            rect: rect.map(|rect| [rect.x, rect.y, rect.w, rect.h]),
            value_min: None,
            value_max: None,
            value_now: None,
            value_text: None,
            busy: false,
        };
        let mut nodes = Vec::with_capacity(SHELL_PALETTE_PLAN_ENTRY_CAPACITY + 4);
        nodes.push(basic(kind.dialog_id().to_string(), "dialog", 0, Some(title.clone()), Some(description.clone()), dialog_rect));
        let mut close = basic(kind.close_id().to_string(), "button", 1, Some(shell_chrome_string("common.close", is_de).to_string()), None, rect_for(kind.close_id()));
        close.focusable = true;
        close.tabbable = true;
        close.actionable = true;
        nodes.push(close);
        let mut input = basic(kind.input_id().to_string(), "combobox", 1, Some(title.clone()), Some(description), input_rect);
        input.shortcut = shell_control_hotkey_badge(&self.chrome.shortcut_table(), kind.input_id());
        input.focusable = true;
        input.tabbable = true;
        input.actionable = true;
        input.focused = self.input.accessibility_focused_control_id.as_deref() == Some(kind.input_id());
        input.expanded = Some(true);
        input.editable = true;
        input.controls = Some(kind.list_id().to_string());
        input.active_descendant = rows.get(selected_index).map(|(_, id, _, _, _)| id.clone());
        input.value_text = Some(query);
        nodes.push(input);
        nodes.push(basic(kind.list_id().to_string(), "listbox", 1, Some(title), None, list_rect));
        if rows.is_empty() {
            let mut status = basic(format!("{}.empty", kind.dialog_id()), "status", 2, Some(empty), None, list_rect);
            status.live = ui_contract::liveness_name(ui_contract::Liveness::Polite).to_string();
            nodes.push(status);
        } else {
            let mut groups = Vec::<String>::new();
            for (_, _, _, _, group) in &rows {
                if !groups.iter().any(|existing| existing == group) {
                    groups.push(group.clone());
                }
            }
            for (group_index, group) in groups.into_iter().enumerate() {
                nodes.push(basic(format!("{}.group.{group_index}", kind.dialog_id()), "group", 2, (!group.is_empty()).then_some(group.clone()), None, None));
                for (index, id, label, row_description, _) in rows.iter().filter(|(_, _, _, _, row_group)| row_group == &group) {
                    let mut option = basic(id.clone(), "option", 3, Some(label.clone()), row_description.clone(), rect_for(&kind.control_id(*index)));
                    option.actionable = true;
                    option.selected = Some(*index == selected_index);
                    nodes.push(option);
                }
            }
        }
        for (index, node) in nodes.iter_mut().enumerate() {
            node.node_id = index as u64 + 1;
        }
        Some(nodes)
    }

    /// 🌴️ The open declared dialog as a modal accessibility tree — the twin of React's `UIDialog` (`role="dialog"`
    /// named by its title and described by its body): the dialog node, then every control it painted this frame one
    /// level below it in paint order, named, described, valued and pressed from the painted registry, a gated submit
    /// or choice `disabled`, each focus stop tabbable, and the dialog's own keyboard focus as the focused node. While
    /// it is open nothing behind the veil is published, as React's `aria-modal` makes the page inert.
    pub fn dialog_accessibility_nodes(&self, hits: &[HitTarget<ActionDescriptor>]) -> Option<Vec<ui_contract::AccessibilityProjectionNode>> {
        let request = self.chrome.chrome_build.dialog_stack.last()?;
        let key = request.accessibility_key();
        let prefix = format!("{key}.");
        let stops: Vec<String> = request.stops().into_iter().map(|stop| request.control_id(stop)).collect();
        let focused = request.control_id(request.focused());
        let controls: Vec<(&str, &HitTarget<ActionDescriptor>)> = hits.iter().filter_map(|hit| hit.control_id.as_deref().filter(|id| id.starts_with(&prefix)).map(|id| (id, hit))).collect();
        let bounds = controls.iter().map(|(_, hit)| hit.rect).reduce(|lhs, rhs| {
            let (x, y) = (lhs.x.min(rhs.x), lhs.y.min(rhs.y));
            Rect::new(x, y, (lhs.x + lhs.w).max(rhs.x + rhs.w) - x, (lhs.y + lhs.h).max(rhs.y + rhs.h) - y)
        });
        let node = |node_id: u64, key: &str, role: &str, depth: usize, label: String| {
            let mut node = chrome_status_accessibility_node(node_id, key, label);
            node.role = role.to_string();
            node.depth = depth;
            node.live = ui_contract::liveness_name(ui_contract::Liveness::Off).to_string();
            node
        };
        let mut dialog = node(1, &key, "dialog", 0, request.title.clone());
        dialog.description = (!request.body.is_empty()).then(|| request.body.clone());
        dialog.rect = bounds.map(|rect| [rect.x, rect.y, rect.w, rect.h]);
        let mut nodes = vec![dialog];
        with_chrome_control_names(|names| {
            for (id, hit) in controls.into_iter().take(SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY) {
                let presentation = names.get(id);
                let pressed = presentation.and_then(|name| name.pressed);
                let role = presentation.and_then(|name| name.role).unwrap_or_else(|| if pressed.is_some() { "button" } else { chrome_accessibility_role(&hit.kind) });
                let disabled = presentation.is_some_and(|name| name.disabled);
                let draft = (hit.kind == HitKind::Input).then(|| (0..request.fields.len()).find(|index| request.control_id(ChromeDialogStop::Field(*index)) == id).map(|index| request.fields[index].draft.clone())).flatten();
                let mut control = node(nodes.len() as u64 + 1, id, role, 1, presentation.map_or_else(|| humanize_control_id(id), |name| name.label.clone()));
                control.description = presentation.and_then(|name| name.description.clone());
                control.disabled = disabled;
                control.focusable = true;
                control.tabbable = stops.iter().any(|stop| stop == id);
                control.actionable = !disabled;
                control.focused = focused == id;
                control.pressed = pressed;
                control.editable = matches!(role, "textbox" | "spinbutton");
                control.rect = Some([hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h]);
                control.value_min = presentation.and_then(|name| name.value_range).map(|range| range[0]);
                control.value_max = presentation.and_then(|name| name.value_range).map(|range| range[1]);
                control.value_now = presentation.and_then(|name| name.value_range).map(|range| range[2]);
                control.value_text = presentation.and_then(|name| name.value_text.clone()).or(draft);
                nodes.push(control);
            }
        });
        Some(nodes)
    }

    /// ♿️ This frame's chrome controls as accessibility projection nodes — the SAME wire shape the
    /// retained document publishes, so `🚀️browser-boot/🟦️.ts`'s mirror gives them elements with no
    /// new code: `aria-label` from the painted text, `aria-keyshortcuts` from this session's own
    /// remappable chord table (React's `aria-keyshortcuts` twin, audit W14 §B14), `aria-disabled`
    /// from the registry.
    pub fn chrome_accessibility_nodes(&mut self, hits: &[HitTarget<ActionDescriptor>]) -> Vec<ui_contract::AccessibilityProjectionNode> {
        if let Some(mut nodes) = self.dialog_accessibility_nodes(hits).or_else(|| self.palette_accessibility_nodes(hits)) {
            if let Some(notice) = self.transient_notice_accessibility_node(nodes.iter().map(|node| node.node_id).max().unwrap_or(0) + 1) {
                nodes.push(notice);
            }
            for mut band in self.as_overlays_view().band_accessibility_nodes(nodes.iter().map(|node| node.node_id).max().unwrap_or(0) + 1) {
                band.focusable = false;
                band.focused = false;
                nodes.push(band);
            }
            return nodes;
        }
        let shortcuts = self.chrome.shortcut_table();
        with_chrome_control_names(|names| {
            let mut nodes = hits
                .iter()
                .filter_map(|hit| hit.control_id.as_deref().filter(|id| !id.is_empty()).map(|id| (id, hit)))
                .filter(|(id, _)| !self.input.retained_hit_windows.contains_key(*id))
                .filter(|(id, hit)| match hit.kind {
                    HitKind::PanelResize | HitKind::DockSplit | HitKind::DockJoinCorner => false,
                    HitKind::ScrollRegion => names.get(*id).is_some_and(|name| name.role == Some("tabpanel")),
                    HitKind::Generic => hit.event.is_some() || names.contains_key(*id),
                    _ => true,
                })
                .take(SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY)
                .enumerate()
                .map(|(index, (id, hit))| {
                    let presentation = names.get(id);
                    let pressed = presentation.and_then(|name| name.pressed).or_else(|| self.chrome_accessibility_pressed(id, &hit.kind));
                    let role = presentation.and_then(|name| name.role).unwrap_or_else(|| if pressed.is_some() { "button" } else { chrome_accessibility_role(&hit.kind) });
                    let selected = names.get(id).and_then(|name| name.selected).or_else(|| self.chrome_accessibility_selected(id, &hit.kind));
                    let focusable = !matches!(role, "tabpanel" | "note");
                    ui_contract::AccessibilityProjectionNode {
                        value_step: None,
                        invalid: false,
                        set_size: None,
                        pos_in_set: None,
                        tone: None,
                        node_id: index as u64 + 1,
                        key: id.to_string(),
                        role: role.to_string(),
                        depth: 0,
                        label: Some(names.get(id).map(|name| name.label.clone()).unwrap_or_else(|| humanize_control_id(id))),
                        description: names.get(id).and_then(|name| name.description.clone()).or_else(|| ShellPaletteKind::from_input_id(id).map(|kind| {
                            shell_chrome_string(
                                match kind {
                                    ShellPaletteKind::Search => "search.description",
                                    ShellPaletteKind::Find => "find.description",
                                },
                                self.prefs.locale_id == "de",
                            )
                            .to_string()
                        })),
                        live: ui_contract::liveness_name(ui_contract::Liveness::Off).to_string(),
                        shortcut: shell_control_hotkey_badge(&shortcuts, id),
                        hidden: false,
                        disabled: presentation.is_some_and(|name| name.disabled),
                        focusable,
                        tabbable: focusable && (role != "tab" || selected == Some(true)),
                        actionable: names.get(id).is_none_or(|name| name.role != Some("tabpanel") && !name.disabled),
                        focused: self.input.accessibility_focused_control_id.as_deref() == Some(id),
                        checked: (role == "switch").then(|| self.chrome_accessibility_checked(id, &hit.kind)).flatten(),
                        pressed,
                        selected,
                        expanded: (hit.kind == HitKind::Select).then(|| self.input.open_selects.get(id).copied().unwrap_or(false)),
                        editable: false,
                        multiline: false,
                        controls: names.get(id).and_then(|name| name.controls.clone()),
                        active_descendant: None,
                        level: None,
                        rect: Some([hit.rect.x, hit.rect.y, hit.rect.w, hit.rect.h]),
                        value_min: presentation.and_then(|name| name.value_range).map(|range| range[0]),
                        value_max: presentation.and_then(|name| name.value_range).map(|range| range[1]),
                        value_now: presentation.and_then(|name| name.value_range).map(|range| range[2]),
                        value_text: ShellPaletteKind::from_input_id(id)
                            .map(|kind| self.commands.palette_query(kind).to_string())
                            .or_else(|| presentation.and_then(|name| name.value_text.clone()))
                            .or_else(|| self.input.widget_maps.input_metas.get(id).map(|meta| meta.value.clone())),
                        busy: false,
                    }
                })
                .collect::<Vec<_>>();
            for anchor in PanelAnchor::ALL.into_iter().filter(|anchor| !self.chrome.mobile_panel_active() && self.dock.anchor_open(*anchor)) {
                for row in self.dock.anchor_panel_tab_rows(anchor) {
                    for tab in row {
                        if nodes.len() >= SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY || nodes.iter().any(|node| node.key == tab.id) {
                            continue;
                        }
                        nodes.push(ui_contract::AccessibilityProjectionNode {
                            value_step: None,
                            invalid: false,
                            set_size: None,
                            pos_in_set: None,
                            tone: None,
                            node_id: nodes.len() as u64 + 1,
                            key: tab.id.clone(),
                            role: "button".to_string(),
                            depth: 0,
                            label: Some(tab.label.clone()),
                            description: None,
                            live: ui_contract::liveness_name(ui_contract::Liveness::Off).to_string(),
                            shortcut: shell_control_hotkey_badge(&shortcuts, &tab.id),
                            hidden: false,
                            disabled: false,
                            focusable: true,
                            tabbable: true,
                            actionable: true,
                            focused: self.input.accessibility_focused_control_id.as_deref() == Some(tab.id.as_str()),
                            checked: None,
                            pressed: Some(self.dock.anchor_state(anchor).path.iter().any(|id| id == &tab.id)),
                            selected: None,
                            expanded: None,
                            editable: false,
                            multiline: false,
                            controls: None,
                            active_descendant: None,
                            level: None,
                            rect: None,
                            value_min: None,
                            value_max: None,
                            value_now: None,
                            value_text: None,
                            busy: false,
                        });
                    }
                }
            }
            if let Some(export) = self.opening.icon_export.as_ref() {
                let mut status = chrome_status_accessibility_node(nodes.len() as u64 + 1, "shell.icon-export.progress", export.message(self.prefs.locale_id == "de"));
                let (done, total) = export.counts();
                status.role = "progressbar".into();
                status.value_min = Some(0.0);
                status.value_max = Some(total as f64);
                status.value_now = Some(done as f64);
                status.value_text = status.label.clone();
                status.busy = export.running();
                nodes.push(status);
            }
            if let Some(notice) = self.transient_notice_accessibility_node(nodes.len() as u64 + 1).filter(|_| nodes.len() < SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY) {
                nodes.push(notice);
            }
            if let Some(folder) = self.as_overlays_view().folder_reconnect_accessibility_node(nodes.len() as u64 + 1).filter(|_| nodes.len() < SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY) {
                nodes.push(folder);
            }
            for band in self.as_overlays_view().band_accessibility_nodes(nodes.len() as u64 + 1) {
                if nodes.len() < SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY {
                    nodes.push(band);
                }
            }
            for (key, label) in self.footer_status_chips() {
                if nodes.len() < SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY && !nodes.iter().any(|node| node.key == key) {
                    nodes.push(chrome_status_accessibility_node(nodes.len() as u64 + 1, key, label));
                }
            }
            nodes
        })
    }

    /// 🧯️ The showing transient notice as the polite status React renders it (`role=status`, `aria-live=polite`,
    /// `data-notice-code`): named by its localized message and described by its machine code, so an assistive technology
    /// and the ARIA mirror hear what the banner paints and not only its close button. Projected over a modal too — a
    /// refusal answered while a dialog is open is still told. `None` once dismissed or past its 4 s deadline.
    pub fn transient_notice_accessibility_node(&self, node_id: u64) -> Option<ui_contract::AccessibilityProjectionNode> {
        let notice = self.chrome.chrome_build.transient_notice.as_ref().filter(|notice| !transient_notice_expired(notice, chrome_now_ms()))?;
        let mut node = chrome_status_accessibility_node(node_id, TRANSIENT_NOTICE_STATUS_ID, notice.message.clone());
        node.description = notice.code.clone();
        Some(node)
    }

    /// 🏷️ The footer chips this shell paints without a hit target — the presence roster (React's `#s-presence-peers`
    /// status) and the hub connection — with the text they paint, so a screen reader and the
    /// accessibility mirror read them too. The signed-out action is a separate button and gets its own hit-backed node.
    pub fn footer_status_chips(&self) -> Vec<(&'static str, String)> {
        let mut chips = Vec::new();
        if !self.chrome.mobile_panel_active() {
            chips.push(("s-presence-peers", ui_wgpu::wgpu::presence_bar_chip_accessible_text(&self.footer_presence_rows(), None, self.prefs.active_locale())));
        }
        chips.push(("s-hub-connection", self.sync.hub_footer_label(&self.prefs, &self.directory)));
        chips
    }
}

//#endregion ♿️ChromeAccessibleNames

