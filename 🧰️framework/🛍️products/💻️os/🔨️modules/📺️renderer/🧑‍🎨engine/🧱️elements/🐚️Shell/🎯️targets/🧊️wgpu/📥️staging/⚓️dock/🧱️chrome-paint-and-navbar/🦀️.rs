//! ⚓️ `shell-dock` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

pub fn retained_panel_chrome_item_width(atlas: &mut FontAtlas, theme: &Theme, item: &ChromeGroupItem<'_>) -> Option<f32> {
    let label = item.label.unwrap_or("");
    if label.len() > semio_framework_ui_wgpu_paint::RETAINED_NODE_TEXT_MAX_BYTES {
        return None;
    }
    let leading = item.icon_id.map_or(0.0, |_| PANEL_CHROME_ICON_TINY + theme.gap_standard);
    let label_width = if label.is_empty() { 0.0 } else { atlas.measure_text(label, theme.font_size_small).0 };
    Some(theme.padding_standard * 2.0 + leading + label_width + theme.gap_standard + PANEL_CHROME_GRIP_SIZE)
}

/// 📑️ Whether this dock tab subtree carries `tab_id` anywhere — root row or drilled-down child.
pub fn dock_tab_node_carries(node: &DockTabNode, tab_id: &str) -> bool {
    node.id == tab_id || node.children.iter().any(|child| dock_tab_node_carries(child, tab_id))
}

//#region 🚦️SyncPill
/// 🚦️ Localized pill text, term for term with `syncPillText` (`🛠️ShellHelpers/🟦️.tsx`) — including
/// its German column, which this side used to lack: the pill was English-only, so a `de` shell read
/// `Remote: detached` where React reads `Remote: getrennt`. The strings live in
/// [`shell_chrome_string`] with every other shell-owned label rather than inline here.
pub fn shell_sync_pill_text(pill: ShellSyncPill, is_de: bool) -> String {
    match pill {
        ShellSyncPill::Recovering { received_bytes, total_bytes, received_chunks, total_chunks } => {
            format!("{} {received_bytes}/{total_bytes} · {received_chunks}/{total_chunks}", shell_chrome_string("sync.recovering", is_de))
        }
        ShellSyncPill::Persisted => shell_chrome_string("sync.persisted", is_de).to_string(),
        ShellSyncPill::Pending(count) => format!("{} ({count})", shell_chrome_string("sync.pending", is_de)),
        ShellSyncPill::Remote(remote) => {
            let state = match remote {
                ShellSyncRemote::Connected => "sync.remote.connected",
                ShellSyncRemote::Connecting => "sync.remote.connecting",
                ShellSyncRemote::Backoff => "sync.remote.backoff",
                ShellSyncRemote::Detached => "sync.remote.detached",
            };
            format!("{}: {}", shell_chrome_string("sync.remote", is_de), shell_chrome_string(state, is_de))
        }
    }
}

//#endregion 🚦️SyncPill
//#region ActionPanelAndUtilities
/// 🧰️ Resolves the utilities a window kind presents in the utility bar — the utility mirror of
/// {@link semio_framework::resolve_window_actions}: explicit `window_kind.utilities` refs in declared
/// order, plus any app utility referenced by no window kind (an "orphan" appearing on every window — the
/// scoping fallback that prevents blank utility bars mid-migration, Architecture Decision 8).
pub fn resolve_window_utilities<'a>(app: &'a AppDefinition, window_kind: &semio_framework::WindowKindDefinition) -> Vec<&'a semio_framework::UtilityDefinition> {
    use std::collections::HashSet;
    let referenced: HashSet<&str> = app.window_kinds.iter().flat_map(|window| window.utilities.iter().map(|utility_ref| utility_ref.as_str())).collect();
    let mut resolved: Vec<&'a semio_framework::UtilityDefinition> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for utility_ref in &window_kind.utilities {
        if let Some(utility) = app.utilities.iter().find(|utility| utility.id == utility_ref.as_str()) {
            if seen.insert(utility.id.as_str()) {
                resolved.push(utility);
            }
        }
    }
    for utility in &app.utilities {
        if !referenced.contains(utility.id.as_str()) && seen.insert(utility.id.as_str()) {
            resolved.push(utility);
        }
    }
    resolved
}

//#region 🔀️ChromeParity
/// ⌨️ One stop of the native shell's keyboard ring ([`ShellState::keyboard_ring`]): a focusable chrome control by
/// its accessibility key, or a dock window by its stack path and id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellKeyboardStop {
    Chrome(String),
    Window(Vec<usize>, String),
}

//#endregion 🔀️ChromeParity
//#region 🪟️WindowPaneChrome

fn world_projection_icon_id_v1(template_id: &str) -> Option<&'static str> {
    world_projection_template_selection_id(template_id).and_then(|id| WORLD_PROJECTION_TEMPLATES.iter().find(|template| template.id == id)).map(|template| template.icon_id)
}

/// 🖼️ Resolves one accepted dock tab through React's `windowIconsById[id] ?? projection ?? kind`
/// precedence. The projection arm remains a fallback for a newly mounted World pane whose local
/// `SetWindowIconContext` publication has not run yet.
pub fn dock_tab_icon_id_v1(kind_icon_id: &str, override_icon_id: Option<&str>, template_id: Option<&str>) -> String {
    override_icon_id.filter(|icon_id| !icon_id.is_empty()).or_else(|| template_id.and_then(world_projection_icon_id_v1)).unwrap_or(kind_icon_id).to_string()
}

impl ShellDockState {
    /// 🎯️ The live window instance and the chip a pane-chip control id addresses — the read side of
    /// [`WindowPaneChip::control_id`].
    ///
    /// ⚖️ The window is RESOLVED, never parsed back out: `element_id_segment` is lossy
    /// (`puzzle3d-main-top` → `puzzle3dMainTop`), so the segment is matched against the instances this
    /// frame actually carries — which is also what refuses a chip addressed to a window that has since
    /// closed.
    pub fn window_pane_chip_target(&self, control_id: &str, docs: &DocumentState) -> Option<(String, WindowPaneChip)> {
        let (parent, rest) = [WINDOW_PANE_CHIP_PARENT, WORLD_PROJECTION_PANE_PARENT].into_iter().find_map(|parent| control_id.strip_prefix(parent).and_then(|rest| rest.strip_prefix('.')).map(|rest| (parent, rest)))?;
        let (segment, tail) = rest.split_once('.')?;
        let (pane, verb) = tail.split_once('.')?;
        let chip = WindowPaneChip::from_pane_and_verb(pane, verb)?;
        if matches!(chip, WindowPaneChip::Projection) != (parent == WORLD_PROJECTION_PANE_PARENT) {
            return None;
        }
        self.dock.window_instances().into_iter().map(|(window, _)| window).chain(self.live_window_ids(docs)).find(|window| semio_framework::element_id_segment(window) == segment).map(|window| (window, chip))
    }

    /// 🎛️ Flips one pane chip's own fold state, and NOTHING else — React's chips are fold toggles.
    ///
    /// ⚖️ Every chip first activates its own window, because React activates a window on any pointer
    /// that lands in it (`GhostRegionShell`'s `onPointerDownCapture` → `onActivate`,
    /// `🪟️Window/🟦️.tsx:302`), and a chip is painted inside the window body.
    ///
    /// ⚖️ The five folds are independent. No chip closes another pane's fold, and none of them
    /// opens or closes a SURFACE — every `Pane` holds its own `useState(true)`
    /// (`🪟️Window/🟦️.tsx:169`/`:176`/`:183`, `WorldOrbitProjectionSwitchPane`'s at
    /// `🌐️World3dHost/🟦️.tsx:5225`).
    ///
    /// 🩸️ `Search` used to also flip `search_open`/`overlay_state`, i.e. open the SHELL's own
    /// centred command palette (the `ui.search.toggle` / `⌘K` overlay). React's Search chip opens a
    /// window-scoped `Pane` and focuses its input; it never opens the shell overlay, so the wgpu chip
    /// raised a surface React does not raise (`📓️w12d-pane-chip-ids-and-projection-toggle.md` §2).
    pub fn toggle_window_pane_chip(&mut self, window_id: &str, chip: WindowPaneChip, surfaces: &SurfaceState, docs: &mut DocumentState, windows: &mut WindowState) -> Result<(), String> {
        if self.dock.activate_window(window_id) {
            windows.active_window_id = Some(window_id.to_string());
        }
        match chip {
            WindowPaneChip::WindowOptions => {
                let folded = windows.measures_rail_folded(window_id);
                windows.measures_folded.insert(window_id.to_string(), !folded);
            }
            WindowPaneChip::Utilities => {
                let folded = windows.utility_bar_folded(window_id);
                windows.utility_bar_folded.insert(window_id.to_string(), !folded);
            }
            WindowPaneChip::Projection => {
                let folded = windows.projection_pane_folded(window_id);
                windows.projection_pane_folded.insert(window_id.to_string(), !folded);
                if folded {
                    windows.refresh_window_projection_document(window_id, surfaces, docs)?;
                } else {
                    let previous = docs.window_projection_documents.remove(window_id);
                    docs.retire_one_surface_document(previous)?;
                }
            }
            WindowPaneChip::Actions => {
                let folded = windows.window_actions_folded(window_id);
                windows.action_panel_folded.insert(window_id.to_string(), !folded);
            }
            WindowPaneChip::Search => {
                let folded = windows.window_search_folded(window_id);
                windows.search_panel_folded.insert(window_id.to_string(), !folded);
            }
        }
        Ok(())
    }

    /// 🔀️ Applies one live Projection Tree row through the same view-state and camera path as the
    /// former bespoke row, then republishes the retained Tree so selection and AX stay atomic.
    pub fn select_world_projection_template(&mut self, window_id: &str, template_id: &str, surfaces: &mut SurfaceState, docs: &mut DocumentState, windows: &mut WindowState) -> Result<(), String> {
        if surfaces.world3d_host_id_for_window(window_id).is_none() {
            return Ok(());
        }
        let effective_id = match template_id {
            "parallel" => "orthographic",
            "axonometric" => "axonometric-isometric",
            "oblique" => "oblique-cavalier",
            "perspective" => "three-point",
            other => other,
        };
        let template = world_projection_template(effective_id);
        if !self.apply_window_title_host_command(window_id, template.label, docs, windows) {
            return Ok(());
        }
        windows.world_projection_template.insert(window_id.to_string(), template.id.to_string());
        let _ = self.apply_window_icon_host_command(window_id, template.icon_id, docs, windows);
        if let Some(world) = surfaces.world3d_state_for_window_mut(window_id) {
            infinite_world::world::apply_world3d_projection_mode(&mut world.session, &mut world.camera, &mut world.scene, template.mode);
        }
        windows.refresh_window_projection_document(window_id, surfaces, docs)
    }
}

//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities

