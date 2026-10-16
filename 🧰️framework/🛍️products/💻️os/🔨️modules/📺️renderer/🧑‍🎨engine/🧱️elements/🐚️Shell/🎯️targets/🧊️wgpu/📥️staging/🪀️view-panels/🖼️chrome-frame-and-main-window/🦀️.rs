//! 🪀️ `shell-view-panels` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
pub(crate) struct ShellChromeFrameCursor {
    pub(crate) phase: ShellChromeFramePhase,
    pub(crate) setup: u8,
    pub(crate) child: ShellChromeChildCursor,
    /// 🩺️ Opportunities spent inside [`Self::phase`] without leaving it. `FrameBuildPhase::Chrome`
    /// holds the whole frame until this walk answers `true`, so a phase that can never finish
    /// presents NOTHING — no navbar, no window, no footer — and says nothing either: every
    /// per-document lane has its own `SHELL_WINDOW_PAINT_OPPORTUNITIES` escape hatch, but the
    /// immediate-mode chrome phases between them had none. gis2d on wgpu booted `ready`, laid its
    /// document out at the full viewport and painted a uniformly empty canvas for seven minutes
    /// because of exactly that (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w14a-*`).
    parked_opportunities: u64,
    parked_reports: u8,
    parked_phase: ShellChromeFramePhase,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ShellChromeFramePhase {
    #[default]
    CloseDocument,
    LoadPreferences,
    IntroductionRead,
    Presence,
    PersistLayout,
    FrameSetup,
    MainWindow,
    /// 🪟️ The pane chips' hit rows, registered after every window BODY and before every docked
    /// panel — React's own three-level stack, `--z-window: 10` under `--z-pane: 20` under
    /// `--z-panel: 30` (`🖱️ui/🎨️styling/🖌️ui/🎨️.css:834`) — see [`ShellState::pane_overlay_hits`].
    PaneOverlayHits,
    Panels,
    /// 💬️ The agent chat panel's own header (packet W1j) — title + `🚦️AgentPresence` indicator,
    /// painted over whichever anchor is currently showing the `framework.chat` tab.
    AgentChatHeader,
    Navbar,
    TutorialBar,
    Footer,
    /// 📎️ The offer to reconnect the folder this device remembers for the session's document (`📎️local-folders`),
    /// stacked above the time-travel band like React's bottom bands.
    FolderReconnectBand,
    /// ⏪️ The persistent time-travel band (`HistoryPatch.timeTravel`) above the footer — under every overlay, so an
    /// open dialog's veil covers it exactly as it covers the rest of the workbench.
    TimeTravelBand,
    /// 📡️ The band of a history change replaying before adoption (`HistoryPatch.reprojection`), stacked above the session band.
    HistoryReprojectionBand,
    Overlay,
    HubWorkspace,
    /// 🛂️ The Space Administration sheet (packet W15e) — React mounts it in the same absolutely
    /// positioned overlay band as the inference port and the directory bootstrap notice, below the
    /// approvals dialog and the notice banner.
    SpaceAdministration,
    /// ✅️ The agent approvals modal (packet W1j) — above every other overlay, like React's
    /// unconditionally mounted `<AgentApprovals>` dialog.
    AgentApprovals,
    /// 🧯️ The transient notice banner (packet W1j) — painted last of the overlays so it sits on top.
    TransientNotice,
    /// 🎬️ The lazy plugin-install band — the one long-running phase the shell itself owns (every
    /// other one belongs to a guest job), painted under the notice with its own cancel control.
    PluginInstall,
    /// 🚪️ The frame-pumped document open's band — phase, step, elapsed time and its cancel control —
    /// painted under the plugin-install band while the guest instantiates and loads the document.
    DocumentOpen,
    IconExport,
    TreeDrag,
    TutorialGesture,
    Error,
    IntroductionWrite,
    PersistPreferences,
    Complete,
}

impl Default for ShellChromeFrameCursor {
    fn default() -> Self {
        Self { phase: ShellChromeFramePhase::default(), setup: 0, child: ShellChromeChildCursor::default(), parked_opportunities: 0, parked_reports: 0, parked_phase: ShellChromeFramePhase::default() }
    }
}

impl ShellChromeFrameCursor {
    pub(crate) fn terminal_is_complete(&self) -> bool {
        matches!(self.phase, ShellChromeFramePhase::Complete)
    }

    pub(crate) fn advance(&mut self, phase: ShellChromeFramePhase) {
        self.phase = phase;
        self.child = ShellChromeChildCursor::default();
    }

    /// 🩺️ Counts this opportunity against the current phase and answers the census line to publish
    /// when the phase has overstayed [`SHELL_CHROME_PHASE_OPPORTUNITIES`]. Keyed on the phase itself
    /// rather than on [`Self::advance`], because several phases move the cursor by assignment.
    /// Pure, so a law can own it.
    fn note_opportunity(&mut self) -> Option<String> {
        if self.parked_phase != self.phase {
            self.parked_phase = self.phase;
            self.parked_opportunities = 0;
            self.parked_reports = 0;
        }
        self.parked_opportunities = self.parked_opportunities.saturating_add(1);
        if self.parked_opportunities % SHELL_CHROME_PHASE_OPPORTUNITIES != 0 || self.parked_reports >= SHELL_CHROME_PARKED_REPORTS {
            return None;
        }
        self.parked_reports = self.parked_reports.saturating_add(1);
        Some(format!(
            "phase={} opportunities={} setup={} child-phase={} child-item={} child-scalar={} child-group={} child-depth={} child-flag={} child-fault={} document={}",
            self.phase.name(),
            self.parked_opportunities,
            self.setup,
            self.child.phase,
            self.child.item,
            self.child.scalar,
            self.child.group_phase,
            self.child.depth,
            self.child.flag,
            self.child.fault,
            self.child.document.phase_name(),
        ))
    }
}

impl ShellChromeFramePhase {
    /// 🩺️ The phase's own name, for the parked-walk census. A `Debug` derive would do, but this
    /// enum is `Copy` in a hot cursor and the census is the only reader.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::CloseDocument => "CloseDocument",
            Self::LoadPreferences => "LoadPreferences",
            Self::IntroductionRead => "IntroductionRead",
            Self::Presence => "Presence",
            Self::PersistLayout => "PersistLayout",
            Self::FrameSetup => "FrameSetup",
            Self::MainWindow => "MainWindow",
            Self::PaneOverlayHits => "PaneOverlayHits",
            Self::Panels => "Panels",
            Self::AgentChatHeader => "AgentChatHeader",
            Self::Navbar => "Navbar",
            Self::TutorialBar => "TutorialBar",
            Self::Footer => "Footer",
            Self::FolderReconnectBand => "FolderReconnectBand",
            Self::TimeTravelBand => "TimeTravelBand",
            Self::HistoryReprojectionBand => "HistoryReprojectionBand",
            Self::Overlay => "Overlay",
            Self::HubWorkspace => "HubWorkspace",
            Self::SpaceAdministration => "SpaceAdministration",
            Self::AgentApprovals => "AgentApprovals",
            Self::TransientNotice => "TransientNotice",
            Self::PluginInstall => "PluginInstall",
            Self::DocumentOpen => "DocumentOpen",
            Self::IconExport => "IconExport",
            Self::TreeDrag => "TreeDrag",
            Self::TutorialGesture => "TutorialGesture",
            Self::Error => "Error",
            Self::IntroductionWrite => "IntroductionWrite",
            Self::PersistPreferences => "PersistPreferences",
            Self::Complete => "Complete",
        }
    }
}

impl<'ctx> PanelsViewCtx<'ctx> {
    /// 🎭️ Advances one retained chrome child per worker opportunity.
    ///
    /// 🩺️ `FrameBuildPhase::Chrome` holds the whole frame until this walk completes, so a phase
    /// that cannot advance presents nothing at all. Say so — on the console and on the chrome
    /// ledger `dumpChrome` reads — instead of going quiet behind an empty canvas.
    ///
    /// 🎯️ One chrome walk mints one pointer registry, into the STAGING buffer
    /// `FrameBuildPhase::InputFrame` has just retired — so the owner map this walk
    /// fills is cleared with it, while the last complete registry and ITS owner map
    /// stay resolvable until `publish_retained_hit_registry` swaps both in.
    ///
    /// 🎯️ ONE row per opportunity, like every other registration in this walk, and in the
    /// order the panes painted them: a reverse drain would put the LAST pane's chips first,
    /// and a caller addressing a chip by its suffix would resolve the wrong pane's.
    ///
    /// 📱️ Below React's mobile breakpoint the eight anchors do not paint at all: ONE mobile
    /// panel carries every anchor's tabs flattened (`mobile_panel_tabs`), filling the body
    /// instead of floating — React's `LayoutMobilePanel`, an in-flow sibling of the canvas.
    pub(crate) fn render_chrome_step(
        &mut self,
        cursor: &mut ShellChromeFrameCursor,
        draw: &mut DrawList,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool,semio_framework_job::OriginalTurnFailure>{
        self.opening.advance_icon_export(cx)?;
        if cx.should_yield(){return Ok(false)}
        let mut failure=None;let completed=(||{
        if let Some(census) = cursor.note_opportunity() {
            debug_log(&format!("[TRACE] wgpu-shell chrome walk parked {census} windows={}", self.dock.dock_window_plan.len()));
            crate::interpreter::note_chrome_walk_parked(&census);
        }
        let w = self.chrome.screen_w;
        let h = self.chrome.screen_h;
        let body = self.as_session().body_rect(theme);
        match cursor.phase {
            ShellChromeFramePhase::CloseDocument => {
                let close_consumed=match crate::interpreter::close_ui_document_one(cx){Ok(advanced)=>advanced||self.docs.close_document_one(),Err(error)=>{failure=Some(error);return false}};
                begin_ui_document_opportunity(close_consumed);
                if crate::interpreter::ui_document_close_pending() {
                    return false;
                }
                cursor.phase = ShellChromeFramePhase::LoadPreferences;
            }
            ShellChromeFramePhase::LoadPreferences => {
                self.chrome.request_chrome_preferences_load();
                cursor.phase = ShellChromeFramePhase::IntroductionRead;
            }
            ShellChromeFramePhase::IntroductionRead => {
                self.docs.request_introduction_read(&mut self.chrome);
                cursor.phase = ShellChromeFramePhase::Presence;
            }
            ShellChromeFramePhase::Presence => {
                self.sync.request_presence_preview(&mut self.chrome);
                cursor.phase = ShellChromeFramePhase::PersistLayout;
            }
            ShellChromeFramePhase::PersistLayout => {
                self.dock.request_panel_layout_persist(&mut self.chrome);
                cursor.phase = ShellChromeFramePhase::FrameSetup;
            }
            ShellChromeFramePhase::FrameSetup => {
                match cursor.setup {
                    0 => {
                        self.docs.reconcile_host_introduction_policy(&mut self.chrome, &self.tutorial);
                        draw.set_screen_height(h);
                        self.input.retained_body_rects_staging.clear();
                        self.input.retained_hit_windows_staging.clear();
                        self.input.retained_scene_hits_staging.clear();
                        with_chrome_control_names(|names| names.clear());
                        crate::interpreter::begin_accessibility_visible_documents();
                    }
                    1 => overlay.set_screen_height(h),
                    2 => {
                        draw.push_solid([0.0, 0.0, w, h], theme.background);
                        self.chrome.chrome_floor_scope = Some(ui_wgpu::wgpu::SurfaceScope { level: ui_wgpu::wgpu::Level::Base, fill: ui_wgpu::wgpu::SurfaceFill::Surface });
                    }
                    3 => {
                        if self.chrome.chrome_build.find_items.pop_front().is_some() {
                            return false;
                        }
                        if !self.chrome.chrome_build.find_items.begin_next_generation() {
                            cursor.phase = ShellChromeFramePhase::Complete;
                            return false;
                        }
                    }
                    4 => {
                        if self.commands.find_items.pop_front().is_some() {
                            return false;
                        }
                        if !self.commands.find_items.begin_next_generation() {
                            cursor.phase = ShellChromeFramePhase::Complete;
                            return false;
                        }
                    }
                    5 => {
                        if self.chrome.chrome_build.tooltip_titles.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    6 => {
                        if self.chrome.chrome_build.element_rects.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    7 => {
                        if self.input.widget_maps_staging.input_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    8 => {
                        if self.input.widget_maps_staging.select_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    9 => {
                        if self.input.widget_maps_staging.toggle_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    10 => {
                        if self.input.widget_maps_staging.slider_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    11 => {
                        if self.input.widget_maps_staging.stepper_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    12 => {
                        if self.input.widget_maps_staging.ring_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    13 => {
                        if self.input.widget_maps_staging.slider_live_values.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    14 => {
                        if self.input.widget_maps_staging.ring_live_values.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    15 => {
                        if self.input.widget_maps_staging.tree_hover_commands.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    16 => {
                        if self.input.widget_maps_staging.tree_unhover_commands.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    17 => {
                        self.input.widget_maps_staging.clear_select_popup_wheel();
                        self.input.widget_maps_staging.tree_selection_change = None;
                    }
                    18 => {
                        self.chrome.chrome_build.compute_click_edge(input.pointer_down);
                    }
                    19 => {
                        self.as_session().chrome_tour_frame_begin();
                    }
                    20 => {
                        cursor.phase = ShellChromeFramePhase::MainWindow;
                        return false;
                    }
                    _ => return false,
                }
                cursor.setup += 1;
            }
            ShellChromeFramePhase::MainWindow => {
                if cursor.child.find_rejected.take().is_some() {
                    return false;
                }
                if let Some(item) = self.chrome.chrome_build.find_items.pop_front() {
                    if let Err(item) = self.commands.find_items.try_push(item) {
                        cursor.child.find_rejected = Some(item);
                    }
                    return false;
                }
                let binding = match self.chrome.chrome_build.find_items.bind() {
                    Ok(binding) => binding,
                    Err(()) => return false,
                };
                let mut overlay_slot = Some(overlay);
                let complete = self.render_main_window_step(&mut cursor.child, draw, &mut overlay_slot, atlas, icons, input, theme, body, world_resources, cx);
                drop(binding);
                if !complete {
                    return false;
                }
                if !self.chrome.chrome_build.find_items.terminal_is_empty() {
                    return false;
                }
                cursor.setup = 0;
                cursor.advance(ShellChromeFramePhase::PaneOverlayHits);
            }
            ShellChromeFramePhase::PaneOverlayHits => {
                if !self.input.pane_overlay_hits.is_empty() {
                    let hit = self.input.pane_overlay_hits.remove(0);
                    input.register_hit(hit);
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::Panels);
            }
            ShellChromeFramePhase::Panels if self.chrome.mobile_panel_active() => {
                if !self.as_chrome_view().render_mobile_panel_step(&mut cursor.child, overlay, atlas, icons, input, theme, body, world_resources, cx) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::AgentChatHeader);
            }
            ShellChromeFramePhase::Panels => {
                let Some(anchor) = PanelAnchor::ALL.get(cursor.setup as usize).copied() else {
                    cursor.advance(ShellChromeFramePhase::AgentChatHeader);
                    return false;
                };
                if self.dock.anchor_open(anchor) && !self.as_chrome_view().render_panel_step(&mut cursor.child, anchor, overlay, None, atlas, icons, input, theme, body, world_resources, cx) {
                    return false;
                }
                cursor.setup += 1;
                cursor.child = ShellChromeChildCursor::default();
            }
            ShellChromeFramePhase::AgentChatHeader => {
                if !self.as_chrome_view().render_agent_chat_header_step(&mut cursor.child, overlay, atlas, input, theme, body) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::Navbar);
            }
            ShellChromeFramePhase::Navbar => {
                if !self.as_chrome_view().render_navbar_step(&mut cursor.child, overlay, atlas, icons, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TutorialBar);
            }
            ShellChromeFramePhase::TutorialBar => {
                if !self.as_chrome_view().render_tutorial_bar_step(&mut cursor.child, overlay, atlas, icons, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::Footer);
            }
            ShellChromeFramePhase::Footer => {
                if !self.render_footer_step(&mut cursor.child, overlay, atlas, icons, input, theme, w, h) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::FolderReconnectBand);
            }
            ShellChromeFramePhase::FolderReconnectBand => {
                if !self.as_overlays_view().render_folder_reconnect_band_step(&mut cursor.child, overlay, atlas, input, theme) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TimeTravelBand);
            }
            ShellChromeFramePhase::TimeTravelBand => {
                if !self.as_overlays_view().render_time_travel_band_step(&mut cursor.child, overlay, atlas, input, theme) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::HistoryReprojectionBand);
            }
            ShellChromeFramePhase::HistoryReprojectionBand => {
                if !self.as_overlays_view().render_history_reprojection_band_step(&mut cursor.child, overlay, atlas, input, theme) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::Overlay);
            }
            ShellChromeFramePhase::Overlay => {
                if !self.as_overlays_view().render_overlay_step(&mut cursor.child, overlay, atlas, icons, input, theme, w, h) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::HubWorkspace);
            }
            ShellChromeFramePhase::HubWorkspace => {
                if !self.as_overlays_view().render_hub_workspace_step(&mut cursor.child, overlay, atlas, icons, input, theme, w, h, world_resources, cx) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::SpaceAdministration);
            }
            ShellChromeFramePhase::SpaceAdministration => {
                if !self.as_overlays_view().render_space_administration_step(&mut cursor.child, overlay, atlas, input, theme, w, h) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::AgentApprovals);
            }
            ShellChromeFramePhase::AgentApprovals => {
                if !self.as_overlays_view().render_agent_approvals_step(&mut cursor.child, overlay, atlas, input, theme, w, h) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TransientNotice);
            }
            ShellChromeFramePhase::TransientNotice => {
                if !self.as_chrome_view().render_transient_notice_step(&mut cursor.child, overlay, atlas, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::PluginInstall);
            }
            ShellChromeFramePhase::PluginInstall => {
                if !self.as_chrome_view().render_plugin_install_step(&mut cursor.child, overlay, atlas, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::DocumentOpen);
            }
            ShellChromeFramePhase::DocumentOpen => {
                if !self.as_chrome_view().render_document_opening_step(&mut cursor.child, overlay, atlas, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::IconExport);
            }
            ShellChromeFramePhase::IconExport => {
                if !self.render_icon_export_step(&mut cursor.child, overlay, atlas, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TreeDrag);
            }
            ShellChromeFramePhase::TreeDrag => {
                self.render_tree_drag_overlay_node(overlay, input, theme);
                cursor.advance(ShellChromeFramePhase::TutorialGesture);
            }
            ShellChromeFramePhase::TutorialGesture => {
                render_tutorial_gesture_overlay_node(&*self.chrome, &*self.surfaces, &*self.dock, &*self.tutorial, overlay, theme);
                cursor.advance(ShellChromeFramePhase::Error);
            }
            ShellChromeFramePhase::Error => {
                if let Some(error) = &self.chrome.error {
                    match chrome_text_complete_step(draw, atlas, error, 12.0, h - theme.footer_height - self.as_session().subfooter_height(theme) - 24.0, (w - 24.0).max(1.0), theme.font_size_small, theme.error, &mut cursor.child.glyph) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => cursor.child.glyph.reset(),
                    }
                }
                cursor.advance(ShellChromeFramePhase::IntroductionWrite);
            }
            ShellChromeFramePhase::IntroductionWrite => {
                if self.chrome.chrome_present.maintenance.introduction_write.is_some() {
                    cursor.phase = ShellChromeFramePhase::PersistPreferences;
                } else if let Some(app_id) = self.chrome.chrome_build.introduction_seen_writes.pop() {
                    if app_id.capacity() > SHELL_CHROME_IO_FIELD_BYTES {
                        cursor.phase = ShellChromeFramePhase::PersistPreferences;
                    } else {
                        self.chrome.chrome_present.maintenance.introduction_write = Some(app_id);
                    }
                } else {
                    cursor.phase = ShellChromeFramePhase::PersistPreferences;
                }
            }
            ShellChromeFramePhase::PersistPreferences => {
                self.prefs.request_chrome_preferences_persist(&mut self.chrome);
                cursor.phase = ShellChromeFramePhase::Complete;
            }
            ShellChromeFramePhase::Complete => return true,
        }
        cursor.terminal_is_complete()
    })();match failure{Some(error)=>Err(error),None=>Ok(completed)}
    }

    pub(crate) fn footer_chrome_layout(&mut self, atlas: &mut FontAtlas, theme: &Theme, width: f32, btn_y: f32, btn_h: f32) -> ShellFooterChromeLayout {
        let mobile = self.chrome.mobile_panel_active();
        let bottom_left = if mobile { 0.0 } else { self.as_chrome_view().footer_band_width(atlas, theme, PanelAnchor::BottomLeft) };
        let bottom_middle = if mobile { 0.0 } else { self.as_chrome_view().footer_band_width(atlas, theme, PanelAnchor::BottomMiddle) };
        let bottom_right = if mobile { 0.0 } else { self.as_chrome_view().footer_band_width(atlas, theme, PanelAnchor::BottomRight) };
        let hub = self.sync.hub_connection_state(&self.directory);
        let hub_label = self.sync.hub_footer_label(&self.prefs, &self.directory);
        let hub_item = ChromeGroupItem { control_id: "s-hub-connection", icon_id: Some(hub.icon_id()), label: Some(hub_label.as_str()), active: false, disabled: false, kind: HitKind::Button };
        let hub_width = retained_chrome_group_item_width(atlas, theme, &hub_item).unwrap_or(theme.control_height);
        let sign_in_label = shell_chrome_string("hub.signIn", self.prefs.locale_id == "de");
        let sign_in_item = ChromeGroupItem { control_id: "framework.hub.signIn", icon_id: Some("user"), label: Some(sign_in_label), active: false, disabled: false, kind: HitKind::Button };
        let sign_in_width = (hub == HubConnectionState::SignedOut).then(|| retained_chrome_group_item_width(atlas, theme, &sign_in_item).unwrap_or(theme.control_height));
        let presence_rows = self.footer_presence_rows();
        let presence_label = ui_wgpu::wgpu::presence_bar_chip_text(&presence_rows, None, self.prefs.active_locale());
        let presence_item = ChromeGroupItem { control_id: "s-presence-peers", icon_id: None, label: Some(presence_label.as_str()), active: false, disabled: false, kind: HitKind::Button };
        let presence_width = (!mobile).then(|| retained_chrome_group_item_width(atlas, theme, &presence_item).unwrap_or(0.0));
        let bottom_right_left = width - theme.padding_standard - bottom_right;
        let trailing_right = bottom_right_left - if bottom_right > 0.0 { theme.gap_standard } else { 0.0 };
        let sign_in = sign_in_width.map(|sign_in_width| Rect::new((trailing_right - sign_in_width).max(theme.padding_standard), btn_y, sign_in_width, btn_h));
        let hub_right = sign_in.map_or(trailing_right, |rect| rect.x - theme.gap_standard);
        let hub = Rect::new((hub_right - hub_width).max(theme.padding_standard), btn_y, hub_width.min(width.max(0.0)), btn_h);
        let presence = presence_width.map(|presence_width| Rect::new((hub.x - theme.gap_standard - presence_width).max(theme.padding_standard), btn_y, presence_width, btn_h));
        let trailing_left = presence.map_or(hub.x, |rect| rect.x);
        let occupied = [ShellChromeBandSpan { left: 0.0, right: theme.padding_standard + bottom_left }, ShellChromeBandSpan { left: trailing_left, right: width }];
        ShellFooterChromeLayout { center: shell_chrome_centered_band(width, &occupied, bottom_middle), presence, hub, sign_in, bottom_right_left }
    }

    #[allow(clippy::too_many_arguments, reason = "one arg per footer band input")]
    pub(crate) fn footer_tab_row_rect(&mut self, atlas: &mut FontAtlas, theme: &Theme, index: usize, width: f32, btn_y: f32, btn_h: f32) -> Option<(PanelAnchor, DockTabNode, Rect)> {
        if self.chrome.mobile_panel_active() {
            return None;
        }
        let (anchor, node) = self.as_chrome_view().footer_tab_row_item(index)?;
        let band = self.dock.dock_tabs.tabs(anchor);
        let position = band.iter().position(|candidate| candidate.id == node.id)?;
        let lead = band[..position]
            .iter()
            .map(|ahead| {
                let item = ChromeGroupItem { control_id: "", icon_id: Some(ahead.icon_id.as_str()), label: Some(ahead.label.as_str()), active: false, disabled: false, kind: HitKind::Toggle };
                retained_panel_chrome_item_width(atlas, theme, &item).unwrap_or(0.0)
            })
            .sum::<f32>();
        let layout = self.footer_chrome_layout(atlas, theme, width, btn_y, btn_h);
        let origin = match anchor {
            PanelAnchor::BottomMiddle => layout.center.centered.left,
            PanelAnchor::BottomRight => layout.bottom_right_left,
            _ => theme.padding_standard,
        };
        let item = ChromeGroupItem { control_id: "", icon_id: Some(node.icon_id.as_str()), label: Some(node.label.as_str()), active: false, disabled: false, kind: HitKind::Toggle };
        let item_width = retained_panel_chrome_item_width(atlas, theme, &item)?;
        let rect = Rect::new(origin + lead, btn_y, item_width, btn_h);
        let span = match anchor {
            PanelAnchor::BottomMiddle => layout.center.centered,
            PanelAnchor::BottomRight => ShellChromeBandSpan { left: layout.bottom_right_left, right: width - theme.padding_standard },
            _ => ShellChromeBandSpan { left: theme.padding_standard, right: layout.center.free.right },
        };
        shell_chrome_clip_horizontal(rect, span).map(|rect| (anchor, node, rect))
    }

    /// 🎬️ Who answers this document's action bindings: the owning app's controller, the
    /// same identity `queue_host_effects` stamps on every effect-borne descriptor. The
    /// semantic contract moved it off the node onto the session, so the reconcile has to
    /// be handed it here rather than reading it off a record.
    ///
    /// 🧯 One window's paint is that window's failure, never the frame's. The chrome
    /// walk now places EVERY window the dock names, so a single body whose retained
    /// paint never terminates would hold the whole walk — and with it the navbar, the
    /// panels and the present — hostage forever. That is what a blank canvas with a
    /// correct dock plan and a populated arena looks like
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎯️ ONCE per body, when its paint is off this walk's critical path — a registration
    /// on every paint OPPORTUNITY would push the same entries again and again until the
    /// fixed registry filled. Pushed after the window's own `ScrollRegion`, so every child
    /// of the body outranks it in `InputState::hit_at`'s reverse scan.
    ///
    /// 🪟️ The pane's own overlay rows — `Actions · Search · Window Options` on top,
    /// `Utilities · Projection` at the bottom, at React's `Pane` anchors. Painted AFTER the
    /// body and the measures rail so the chips outrank both in `InputState::hit_at`'s
    /// reverse scan, exactly as React's `z-pane` overlay sits over `window-body`.
    ///
    /// 🔀️ The pane's own UNFOLDED Projection switch, last so its rows outrank every chip they
    /// grow over — React's `WorldOrbitProjectionSwitchPane` body, portalled into the same host.
    ///
    /// 🎬️ The pane's own UNFOLDED Actions body (top-left) and Search body (top-middle) — the
    /// two retained documents `refresh_window_action_panes` publishes, painted last so their
    /// rows outrank the chips they grow under, exactly as React's open `Pane` body covers the
    /// canvas beneath its own cap.
    pub(crate) fn render_main_window_step(
        &mut self,
        cursor: &mut ShellChromeChildCursor,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        bounds: Rect,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> bool {
        match cursor.phase {
            0 => {
                if ui_wgpu::wgpu::shell_floor_paints(self.chrome.chrome_floor_scope) {
                    draw.push_solid([bounds.x, bounds.y, bounds.w, bounds.h], theme.background);
                }
                cursor.rect = Some(bounds.inset(theme.panel_inset));
                cursor.phase = 1;
            }
            1 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                self.as_chrome_view().plan_dock_windows(rect, theme, atlas);
                self.input.pane_overlay_hits.clear();
                cursor.item = 0;
                cursor.phase = 2;
            }
            2 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                draw.push_scissor(rect);
                cursor.phase = 3;
            }
            3 => {
                let Some((window_id, window_rect)) = self.dock.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = if self.dock.dock.collect_window_ids().is_empty() { 14 } else { 5 };
                    return false;
                };
                let Some(window) = UiText::try_from_str(&window_id) else {
                    cursor.item += 1;
                    return false;
                };
                cursor.window = Some(window);
                cursor.scalar = 0;
                input.register_hit(HitTarget { rect: window_rect, event: None, control_id: Some(window_id.as_str().into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
                draw.push_solid([window_rect.x, window_rect.y, window_rect.w, window_rect.h], theme.canvas_clear);
                cursor.phase = 4;
            }
            4 => {
                let Some((window_id, window_rect)) = self.dock.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                let controller = self.docs.window_document_session(&window_id, &self.plugins).map(|session| session.app.controller_id.clone()).unwrap_or_else(|| if window_id == "spawned" { String::new() } else { self.docs.document_controller_id(&self.plugins) });
                self.windows.apply_initial_world_projection_template(&window_id, &mut self.surfaces);
                let Some(document) = self.docs.take_window_document(&window_id) else {
                    cursor.document = UiDocumentFrameCursor::default();
                    cursor.scalar = 0;
                    cursor.flag = false;
                    cursor.phase = 8;
                    return false;
                };
                let complete = {
                    let scroll_offsets = &mut self.input.scroll_offsets;
                    let collapsed_sections = &mut self.input.collapsed_sections;
                    let open_selects = &mut self.input.open_selects;
                    let widget_maps = &mut self.input.widget_maps_staging;
                    let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.prefs.locale_id == "de"), world3d_states: &mut self.surfaces.world3d_states, world_resources, window_id: window_id.as_str() };
                    let viewport_height_for_widgets = draw.screen_height();
                    let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
                    ctx.pick_clip = Some(window_rect);
                    render_ui_document_step(&mut cursor.document, &document, window_rect, &mut ctx, window_id.as_str(), controller.as_str(), self.chrome.chrome_build.driver.drag, &mut hosts, cx)
                };
                self.docs.restore_window_document(&window_id, document);
                if !complete {
                    if document_opportunity_remains(cursor, SHELL_WINDOW_PAINT_OPPORTUNITIES) {
                        return false;
                    }
                    if cursor.scalar >= SHELL_WINDOW_PAINT_OPPORTUNITIES {
                        debug_log(&format!("[TRACE] wgpu-shell window paint {window_id} exhausted {SHELL_WINDOW_PAINT_OPPORTUNITIES} opportunities parked-in={}", cursor.document.phase_name()));
                    }
                    self.as_session().record_document_paint_fault(&window_id);
                } else {
                    self.as_session().note_retained_body_painted(&window_id);
                }
                self.input.register_retained_body_hits(&window_id, window_rect, input);
                cursor.document = UiDocumentFrameCursor::default();
                cursor.scalar = 0;
                cursor.flag = false;
                cursor.phase = 8;
            }
            8 => {
                let Some((window_id, window_rect)) = self.dock.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                if !self.as_chrome_view().paint_window_measures_step(cursor, draw, overlay, atlas, icons, input, theme, &window_id, window_rect, world_resources, cx) {
                    return false;
                }
                cursor.document = UiDocumentFrameCursor::default();
                cursor.scalar = 0;
                cursor.flag = false;
                cursor.group_phase = 0;
                cursor.rect = None;
                cursor.depth = 0;
                cursor.phase = 9;
            }
            9 => {
                let Some((window_id, window_rect)) = self.dock.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                if !self.as_overlays_view().paint_window_pane_chips_step(cursor, draw, atlas, icons, input, theme, &window_id, window_rect) {
                    return false;
                }
                cursor.scalar = 0;
                cursor.group_phase = 0;
                cursor.rect = None;
                cursor.x = 0.0;
                cursor.flag = false;
                cursor.depth = 0;
                cursor.path = ShellUtilityPath::default();
                cursor.phase = 10;
            }
            10 => {
                let Some((window_id, window_rect)) = self.dock.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                if !self.as_chrome_view().paint_window_utilities_step(cursor, draw, atlas, icons, input, theme, &window_id, window_rect) {
                    return false;
                }
                cursor.x = 0.0;
                cursor.flag = false;
                cursor.depth = 0;
                cursor.path = ShellUtilityPath::default();
                cursor.group_phase = 0;
                cursor.rect = None;
                cursor.scalar = 0;
                cursor.phase = 11;
            }
            11 => {
                let Some((window_id, window_rect)) = self.dock.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                if !self.as_chrome_view().paint_window_projection_step(cursor, draw, overlay, atlas, icons, input, theme, &window_id, window_rect, world_resources, cx) {
                    return false;
                }
                cursor.scalar = 0;
                cursor.depth = 0;
                cursor.group_phase = 0;
                cursor.rect = None;
                cursor.x = 0.0;
                cursor.document = UiDocumentFrameCursor::default();
                cursor.flag = false;
                cursor.phase = 12;
            }
            12 => {
                let Some((window_id, window_rect)) = self.dock.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                if !self.paint_window_pane_body_step(false, cursor, draw, overlay, atlas, icons, input, theme, &window_id, window_rect, world_resources, cx) {
                    return false;
                }
                cursor.document = UiDocumentFrameCursor::default();
                cursor.scalar = 0;
                cursor.flag = false;
                cursor.phase = 13;
            }
            13 => {
                let Some((window_id, window_rect)) = self.dock.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                if !self.paint_window_pane_body_step(true, cursor, draw, overlay, atlas, icons, input, theme, &window_id, window_rect, world_resources, cx) {
                    return false;
                }
                cursor.document = UiDocumentFrameCursor::default();
                cursor.scalar = 0;
                cursor.flag = false;
                cursor.item += 1;
                cursor.phase = 3;
            }
            14 => {
                let german = self.prefs.locale_id == "de";
                if cursor.group_phase == 0 {
                    cursor.empty_notice_text = shell_chrome_string("display.emptyShell", german);
                    cursor.empty_notice_lines = empty_dock_notice_lines(atlas, bounds.inset(theme.panel_inset), theme, german);
                    cursor.item = 0;
                    cursor.glyph.reset();
                    cursor.group_phase = 1;
                    return false;
                }
                let Some((range, rect)) = cursor.empty_notice_lines.get(cursor.item) else {
                    cursor.empty_notice_lines.clear();
                    cursor.group_phase = 0;
                    cursor.phase = 5;
                    return false;
                };
                let text = &cursor.empty_notice_text[range.clone()];
                match paint_retained_glyph_step_flowed(text, *rect, theme.font_size_body, theme.text_muted, RetainedTextFlow::Clip, atlas, draw, &mut cursor.glyph) {
                    RetainedGlyphStep::Pending => {}
                    RetainedGlyphStep::Complete => {
                        cursor.glyph.reset();
                        cursor.item += 1;
                    }
                    RetainedGlyphStep::Fault => {
                        self.chrome.error = Some("Shell empty dock notice exceeded the retained glyph boundary".into());
                        cursor.phase = 5;
                    }
                }
            }
            5 => {
                if !(self.plugins.space_mode && self.docs.spawned_ui.is_some()) {
                    let (labels, icon_ids) = self.dock.dock_chrome_maps(&self.prefs, &self.docs, &self.windows);
                    let bounds = self.dock.dock_canvas_bounds;
                    let german = self.prefs.locale_id == "de";
                    let mut names = |id: &str, name: crate::dock::DockControlName<'_>| note_dock_control_name(id, name, german);
                    let mut ctx = DockRenderContext { draw, atlas, icons, input, theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: Some(&mut names) };
                    self.dock.dock_view.paint_chrome(&mut ctx, bounds, false);
                    self.dock.dock_view.register_resize_hits(&mut ctx, bounds);
                }
                cursor.phase = 6;
            }
            6 => {
                if let Some((drag, zone)) = self.dock.dock_drag.as_ref().and_then(|drag| drag.drop_zone.as_ref().map(|zone| (drag, zone))) {
                    if let Some(indicator) = drop_zone_indicator_rect(zone, &self.dock.dock_drop_tab_bars, &self.dock.dock_drop_bodies, self.dock.dock_canvas_bounds, theme.gap_standard) {
                        draw.push_rounded([indicator.x, indicator.y, indicator.w, indicator.h], theme.accent.with_alpha(0.24), theme.border_radius);
                    }
                    let ghost_w = (drag.payload.ghost_label.chars().count() as f32 * theme.font_size_small * 0.55 + theme.padding_standard * 2.0).clamp(64.0, 180.0);
                    draw.push_rounded([drag.x - ghost_w * 0.5, drag.y - theme.control_height * 0.5, ghost_w, theme.control_height], theme.selected.with_alpha(0.9), theme.border_radius);
                }
                draw.pop_scissor();
                cursor.phase = 7;
            }
            7 => return true,
            _ => return false,
        }
        false
    }
}

//#endregion ShellChrome

