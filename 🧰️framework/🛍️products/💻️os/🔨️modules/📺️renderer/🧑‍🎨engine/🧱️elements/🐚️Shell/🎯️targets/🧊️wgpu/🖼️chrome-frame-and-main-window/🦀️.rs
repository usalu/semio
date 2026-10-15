//! 🖼️ Chrome frame cursors and the main window render steps.
//!
//! `ShellChromeFrameCursor` and the child cursors, plus the first render half of `impl ShellState`: the chrome
//! step, preferences/introduction steps, maintenance, board presence, navbar layout, dock windows, the main
//! window, anchors, panels and the mobile panel.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

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

/// 🧭️ Fixed 64-deep footer-utility tree cursor path. A newtype rather than a bare `[u16; 64]`
/// field because std implements `Default` for arrays only up to length 32, and
/// `ShellChromeChildCursor` must stay derivable — the bound is on the array, not on the cursor.
struct ShellUtilityPath([u16; 64]);

impl Default for ShellUtilityPath {
    fn default() -> Self {
        Self([0; 64])
    }
}

#[derive(Default)]
pub(crate) struct ShellChromeChildCursor {
    empty_notice_lines: Vec<(std::ops::Range<usize>, Rect)>,
    empty_notice_text: &'static str,
    pub(crate) phase: u16,
    pub(crate) item: usize,
    pub(crate) scalar: usize,
    pub(crate) x: f32,
    pub(crate) right: f32,
    pub(crate) flag: bool,
    pub(crate) fault: bool,
    pub(crate) group_phase: u8,
    pub(crate) document: UiDocumentFrameCursor,
    pub(crate) window: Option<UiText>,
    pub(crate) rect: Option<Rect>,
    pub(crate) path: ShellUtilityPath,
    pub(crate) depth: usize,
    find_rejected: Option<ShellFindItem>,
    pub(crate) glyph: RetainedGlyphCursor,
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

/// 🩺️ Opportunities one chrome phase may spend before the walk reports itself parked. A complete
/// walk of a busy shell spends far fewer than this in ANY single phase — every phase is bounded by
/// its own content (a glyph run, a dock tab, one document's `SHELL_WINDOW_PAINT_OPPORTUNITIES`) —
/// so crossing it means the phase's own cursor is not advancing at all.
const SHELL_CHROME_PHASE_OPPORTUNITIES: u64 = 1 << 18;
/// 🩺️ How many times one parked phase reports itself before it goes quiet: enough to prove the walk
/// is still parked rather than merely slow, bounded so a wedged shell cannot flood the console.
const SHELL_CHROME_PARKED_REPORTS: u8 = 4;

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

impl ShellState {
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
        self.advance_icon_export(cx)?;
        if cx.should_yield(){return Ok(false)}
        let mut failure=None;let completed=(||{
        if let Some(census) = cursor.note_opportunity() {
            Self::debug_log(&format!("[TRACE] wgpu-shell chrome walk parked {census} windows={}", self.dock_window_plan.len()));
            crate::interpreter::note_chrome_walk_parked(&census);
        }
        let w = self.screen_w;
        let h = self.screen_h;
        let body = self.body_rect(theme);
        match cursor.phase {
            ShellChromeFramePhase::CloseDocument => {
                let close_consumed=match crate::interpreter::close_ui_document_one(cx){Ok(advanced)=>advanced||self.close_document_one(),Err(error)=>{failure=Some(error);return false}};
                begin_ui_document_opportunity(close_consumed);
                if crate::interpreter::ui_document_close_pending() {
                    return false;
                }
                cursor.phase = ShellChromeFramePhase::LoadPreferences;
            }
            ShellChromeFramePhase::LoadPreferences => {
                self.request_chrome_preferences_load();
                cursor.phase = ShellChromeFramePhase::IntroductionRead;
            }
            ShellChromeFramePhase::IntroductionRead => {
                self.request_introduction_read();
                cursor.phase = ShellChromeFramePhase::Presence;
            }
            ShellChromeFramePhase::Presence => {
                self.request_presence_preview();
                cursor.phase = ShellChromeFramePhase::PersistLayout;
            }
            ShellChromeFramePhase::PersistLayout => {
                self.request_panel_layout_persist();
                cursor.phase = ShellChromeFramePhase::FrameSetup;
            }
            ShellChromeFramePhase::FrameSetup => {
                match cursor.setup {
                    0 => {
                        self.reconcile_host_introduction_policy();
                        draw.set_screen_height(h);
                        self.retained_body_rects_staging.clear();
                        self.retained_hit_windows_staging.clear();
                        self.retained_scene_hits_staging.clear();
                        with_chrome_control_names(|names| names.clear());
                        crate::interpreter::begin_accessibility_visible_documents();
                    }
                    1 => overlay.set_screen_height(h),
                    2 => {
                        draw.push_solid([0.0, 0.0, w, h], theme.background);
                        self.chrome_floor_scope = Some(ui_wgpu::wgpu::SurfaceScope { level: ui_wgpu::wgpu::Level::Base, fill: ui_wgpu::wgpu::SurfaceFill::Surface });
                    }
                    3 => {
                        if self.chrome_build.find_items.pop_front().is_some() {
                            return false;
                        }
                        if !self.chrome_build.find_items.begin_next_generation() {
                            cursor.phase = ShellChromeFramePhase::Complete;
                            return false;
                        }
                    }
                    4 => {
                        if self.find_items.pop_front().is_some() {
                            return false;
                        }
                        if !self.find_items.begin_next_generation() {
                            cursor.phase = ShellChromeFramePhase::Complete;
                            return false;
                        }
                    }
                    5 => {
                        if self.chrome_build.tooltip_titles.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    6 => {
                        if self.chrome_build.element_rects.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    7 => {
                        if self.widget_maps_staging.input_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    8 => {
                        if self.widget_maps_staging.select_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    9 => {
                        if self.widget_maps_staging.toggle_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    10 => {
                        if self.widget_maps_staging.slider_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    11 => {
                        if self.widget_maps_staging.stepper_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    12 => {
                        if self.widget_maps_staging.ring_metas.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    13 => {
                        if self.widget_maps_staging.slider_live_values.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    14 => {
                        if self.widget_maps_staging.ring_live_values.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    15 => {
                        if self.widget_maps_staging.tree_hover_commands.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    16 => {
                        if self.widget_maps_staging.tree_unhover_commands.extract_if(|_, _| true).next().is_some() {
                            return false;
                        }
                    }
                    17 => {
                        self.widget_maps_staging.clear_select_popup_wheel();
                        self.widget_maps_staging.tree_selection_change = None;
                    }
                    18 => {
                        self.chrome_build.compute_click_edge(input.pointer_down);
                    }
                    19 => {
                        self.chrome_tour_frame_begin();
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
                if let Some(item) = self.chrome_build.find_items.pop_front() {
                    if let Err(item) = self.find_items.try_push(item) {
                        cursor.child.find_rejected = Some(item);
                    }
                    return false;
                }
                let binding = match self.chrome_build.find_items.bind() {
                    Ok(binding) => binding,
                    Err(()) => return false,
                };
                let mut overlay_slot = Some(overlay);
                let complete = self.render_main_window_step(&mut cursor.child, draw, &mut overlay_slot, atlas, icons, input, theme, body, world_resources, cx);
                drop(binding);
                if !complete {
                    return false;
                }
                if !self.chrome_build.find_items.terminal_is_empty() {
                    return false;
                }
                cursor.setup = 0;
                cursor.advance(ShellChromeFramePhase::PaneOverlayHits);
            }
            ShellChromeFramePhase::PaneOverlayHits => {
                if !self.pane_overlay_hits.is_empty() {
                    let hit = self.pane_overlay_hits.remove(0);
                    input.register_hit(hit);
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::Panels);
            }
            ShellChromeFramePhase::Panels if self.mobile_panel_active() => {
                if !self.render_mobile_panel_step(&mut cursor.child, overlay, atlas, icons, input, theme, body, world_resources, cx) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::AgentChatHeader);
            }
            ShellChromeFramePhase::Panels => {
                let Some(anchor) = PanelAnchor::ALL.get(cursor.setup as usize).copied() else {
                    cursor.advance(ShellChromeFramePhase::AgentChatHeader);
                    return false;
                };
                if self.anchor_open(anchor) && !self.render_panel_step(&mut cursor.child, anchor, overlay, None, atlas, icons, input, theme, body, world_resources, cx) {
                    return false;
                }
                cursor.setup += 1;
                cursor.child = ShellChromeChildCursor::default();
            }
            ShellChromeFramePhase::AgentChatHeader => {
                if !self.render_agent_chat_header_step(&mut cursor.child, overlay, atlas, input, theme, body) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::Navbar);
            }
            ShellChromeFramePhase::Navbar => {
                if !self.render_navbar_step(&mut cursor.child, overlay, atlas, icons, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TutorialBar);
            }
            ShellChromeFramePhase::TutorialBar => {
                if !self.render_tutorial_bar_step(&mut cursor.child, overlay, atlas, icons, input, theme, w) {
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
                if !self.render_folder_reconnect_band_step(&mut cursor.child, overlay, atlas, input, theme) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TimeTravelBand);
            }
            ShellChromeFramePhase::TimeTravelBand => {
                if !self.render_time_travel_band_step(&mut cursor.child, overlay, atlas, input, theme) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::HistoryReprojectionBand);
            }
            ShellChromeFramePhase::HistoryReprojectionBand => {
                if !self.render_history_reprojection_band_step(&mut cursor.child, overlay, atlas, input, theme) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::Overlay);
            }
            ShellChromeFramePhase::Overlay => {
                if !self.render_overlay_step(&mut cursor.child, overlay, atlas, icons, input, theme, w, h) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::HubWorkspace);
            }
            ShellChromeFramePhase::HubWorkspace => {
                if !self.render_hub_workspace_step(&mut cursor.child, overlay, atlas, icons, input, theme, w, h, world_resources, cx) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::SpaceAdministration);
            }
            ShellChromeFramePhase::SpaceAdministration => {
                if !self.render_space_administration_step(&mut cursor.child, overlay, atlas, input, theme, w, h) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::AgentApprovals);
            }
            ShellChromeFramePhase::AgentApprovals => {
                if !self.render_agent_approvals_step(&mut cursor.child, overlay, atlas, input, theme, w, h) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::TransientNotice);
            }
            ShellChromeFramePhase::TransientNotice => {
                if !self.render_transient_notice_step(&mut cursor.child, overlay, atlas, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::PluginInstall);
            }
            ShellChromeFramePhase::PluginInstall => {
                if !self.render_plugin_install_step(&mut cursor.child, overlay, atlas, input, theme, w) {
                    return false;
                }
                cursor.advance(ShellChromeFramePhase::DocumentOpen);
            }
            ShellChromeFramePhase::DocumentOpen => {
                if !self.render_document_opening_step(&mut cursor.child, overlay, atlas, input, theme, w) {
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
                render_tutorial_gesture_overlay_node(self, overlay, theme);
                cursor.advance(ShellChromeFramePhase::Error);
            }
            ShellChromeFramePhase::Error => {
                if let Some(error) = &self.error {
                    match chrome_text_complete_step(draw, atlas, error, 12.0, h - theme.footer_height - self.subfooter_height(theme) - 24.0, (w - 24.0).max(1.0), theme.font_size_small, theme.error, &mut cursor.child.glyph) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => cursor.child.glyph.reset(),
                    }
                }
                cursor.advance(ShellChromeFramePhase::IntroductionWrite);
            }
            ShellChromeFramePhase::IntroductionWrite => {
                if self.chrome_present.maintenance.introduction_write.is_some() {
                    cursor.phase = ShellChromeFramePhase::PersistPreferences;
                } else if let Some(app_id) = self.chrome_build.introduction_seen_writes.pop() {
                    if app_id.capacity() > SHELL_CHROME_IO_FIELD_BYTES {
                        cursor.phase = ShellChromeFramePhase::PersistPreferences;
                    } else {
                        self.chrome_present.maintenance.introduction_write = Some(app_id);
                    }
                } else {
                    cursor.phase = ShellChromeFramePhase::PersistPreferences;
                }
            }
            ShellChromeFramePhase::PersistPreferences => {
                self.request_chrome_preferences_persist();
                cursor.phase = ShellChromeFramePhase::Complete;
            }
            ShellChromeFramePhase::Complete => return true,
        }
        cursor.terminal_is_complete()
    })();match failure{Some(error)=>Err(error),None=>Ok(completed)}
    }

    /// 📨️ Coalesces the persisted chrome load without touching platform storage in the frame step.
    fn request_chrome_preferences_load(&mut self) {
        if !self.chrome_present.preferences_loaded {
            self.chrome_present.maintenance.load_requested = true;
        }
    }

    /// 🎓️ Coalesces one bounded introduction lookup for the shared I/O lane, against the BRAND-SCOPED
    /// key (see [`Self::introduction_seen_key`]).
    pub(crate) fn request_introduction_read(&mut self) {
        if self.chrome_present.maintenance.introduction_read.is_some() {
            return;
        }
        let Some(seen_key) = self.introduction_seen_key() else { return };
        if seen_key.len() <= SHELL_CHROME_IO_FIELD_BYTES && !self.chrome_build.introduction_seen.contains_key(&seen_key) {
            self.chrome_present.maintenance.introduction_read = Some(seen_key);
        }
    }

    /// 🎓️🏷️ The key this boot's introduction-seen flag is filed under — React's `introductionSeenKey`
    /// (`🏛️ShellHost/🟦️.tsx:4159`): `"<brandId>:<appId>"` on a branded shell, the bare app id otherwise,
    /// so the BRANDED tour still plays on a device that already saw the unbranded one.
    ///
    /// 🩸️ What this replaces: the bare `session.app.id`, every time. The wgpu shell carried a brand ID
    /// and nothing else (`📓️w1d` gap 1), so on `aggregator` (6023/6123) it wrote
    /// `ui.introduction.seen.<appId>` where React writes `ui.introduction.seen.<brandId>:<appId>` — the
    /// one concretely demonstrated brand divergence (`📓️w5a-browser-prefs-persistence.md` §7.1).
    pub(crate) fn introduction_seen_key(&self) -> Option<String> {
        let app_id = self.session.as_ref().map(|session| session.app.id.clone())?;
        Some(match crate::boot_brand_id() {
            Some(brand_id) => format!("{brand_id}:{app_id}"),
            None => app_id,
        })
    }

    /// 🎓️ Arms the open app's introduction the first time it launches on this device, the wgpu twin of
    /// `🏛️ShellHost/🟦️.tsx:4139-4166`'s auto-start effect. It runs from the ONE place the answer to
    /// "has this app been seen" becomes known — the `ui.introduction.seen.<appId>` read landing — so it
    /// can neither arm before the flag is readable nor re-arm afterwards: the read is requested only
    /// while `introduction_seen` lacks the key, and the key is written here.
    ///
    /// 🩸️ What this replaces: NOTHING called `start_introduction`. It was `#[cfg(test)]`, so the whole
    /// `🔖️ChromeOverlaysAndTour` tour — veil, info box, advance-by-doing, reveal, keyboard — was
    /// reachable only from its own unit tests, and every wgpu boot painted no tour at all where React
    /// painted `Welcome to Puzzle 3D`.
    ///
    /// 🏷️ React's `replayIntroductionOnLoad` brand flag ignores the device-local seen flag entirely,
    /// which is exactly what a kiosk brand needs. Carried by the brand ROW since this packet.
    pub(crate) fn auto_start_introduction(&mut self, seen_key: &str, seen: bool) {
        let has_introduction = self.session.as_ref().is_some_and(|session| session.app.introduction.is_some());
        let replay_on_load = crate::boot_brand().replays_introduction();
        if !should_auto_start_introduction(seen_key, has_introduction, self.tutorial.is_some(), seen, replay_on_load, crate::host_introduction_suppressed()) {
            return;
        }
        self.chrome_build.start_introduction();
    }

    /// 🎓️ Reconciles a live host-policy transition without marking an unseen introduction as answered.
    pub(crate) fn reconcile_host_introduction_policy(&mut self) {
        let suppressed = crate::host_introduction_suppressed();
        if self.chrome_build.introduction_host_suppressed == suppressed {
            return;
        }
        self.chrome_build.introduction_host_suppressed = suppressed;
        if suppressed {
            self.chrome_build.skip_introduction();
        } else if let Some(key) = self.introduction_seen_key() {
            let seen = self.chrome_build.introduction_was_seen(&key);
            self.auto_start_introduction(&key, seen);
        }
    }

    /// 🗄️ Coalesces one bounded panel-layout page for the shared I/O lane, and ONLY when the dock
    /// actually moved — see [`Self::request_chrome_preferences_persist`] for why an unconditional
    /// arm is not "free".
    fn request_panel_layout_persist(&mut self) {
        if self.chrome_present.last_persisted_dock_ui.as_ref() != Some(&self.dock_ui_snapshot()) || self.chrome_present.last_persisted_dock_skeleton.as_ref() != Some(&self.dock_override) {
            self.chrome_present.maintenance.layout_requested = true;
        }
    }

    /// 💓️ Coalesces one bounded presence preview page for the shared I/O lane. A shell with no sync
    /// channel has no peer to heartbeat to, so the step there would only clear its own flag.
    pub(crate) fn request_presence_preview(&mut self) {
        if self.sync_channel.is_some() {
            self.chrome_present.maintenance.presence_requested = true;
        }
    }

    /// 💾️ Coalesces CHANGED preference fields for the shared I/O lane.
    ///
    /// 🩸️ The three arms above used to fire unconditionally at the end of EVERY chrome walk, so a
    /// settled shell asked the deferred I/O lane for five steps per frame that all had nothing to do.
    /// Each of those steps is a full `check_out_interaction("frame-deferred")` plus an async hand-back,
    /// and `FrameTransaction::step` refuses to run at all while the interaction state is out
    /// (`AppFrameTransactionStep::Superseded`) — so every frame build re-armed the lane, the lane took
    /// the state, the build was superseded, and the next build re-armed it again. Measured on the
    /// puzzle3d wgpu playground as ONE presented frame per boot: the canvas was byte-identical from
    /// t≈9 s to t≈45 s, and the concrete-forest GLB that becomes resident at t≈12 s could never reach
    /// a presented packet (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W5c). Maintenance is
    /// pressure-driven: no pressure, no checkout.
    fn request_chrome_preferences_persist(&mut self) {
        if self.chrome_present.last_synced_preferences.as_ref() != Some(&UiPrefsSnapshot::capture(self)) {
            self.chrome_present.maintenance.persist_requested = true;
        }
    }

    /// 🧵️ Reports whether the frame has retained chrome I/O or preview work to resume.
    pub(crate) fn chrome_maintenance_pending(&self) -> bool {
        !self.retired_world3d_states.is_empty() || self.chrome_present.maintenance.pending()
    }

    /// 🎨️ Exposes the already-loaded theme id without synchronously opening preference storage.
    pub(crate) fn active_theme_id_for_frame(&self) -> &str {
        &self.chrome_build.preferences.theme_id
    }

    /// ⏭️ Advances one bounded field or preview page and retains every remaining owner.
    ///
    /// 🏷️ React's `shouldPersistIntroductionSeen(brand)`: a `replayIntroductionOnLoad` or
    /// `ephemeral` brand answers its tour without ever writing a device-local flag, so the next
    /// load replays it. The in-memory answer still stands for THIS session (the tour does not
    /// come back on a republication) — that is React's `dismissedIntroductionAppIds` half.
    pub(crate) fn advance_chrome_maintenance_step(&mut self) -> bool {
        if !self.retired_world3d_states.is_empty() {
            self.advance_world3d_retirement_step();
        } else if self.chrome_present.maintenance.load_requested {
            self.advance_chrome_preferences_load_step();
        } else if let Some(seen_key) = self.chrome_present.maintenance.introduction_read.take() {
            let key = format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}{seen_key}");
            let seen = stored_field_get(&key).as_deref() == Some("true");
            self.chrome_build.introduction_seen.insert(seen_key.clone(), seen);
            self.auto_start_introduction(&seen_key, seen);
        } else if let Some(seen_key) = self.chrome_present.maintenance.introduction_write.take() {
            if crate::boot_brand().persists_introduction_seen() {
                let key = format!("{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}{seen_key}");
                stored_field_set(&key, "true");
            }
        } else if self.chrome_present.maintenance.layout_requested {
            self.advance_panel_layout_persist_step();
        } else if self.chrome_present.maintenance.presence_requested {
            self.advance_presence_preview_step();
        } else if self.chrome_present.maintenance.persist_requested {
            self.advance_chrome_preferences_persist_step();
        }
        !self.retired_world3d_states.is_empty() || self.chrome_present.maintenance.pending()
    }

    /// 🪦 Advances exactly one retired scene owner under the ordinary frame maintenance budget.
    pub(crate) fn world3d_retirement_fault(&self)->Option<infinite_world::world::WorldDynamicFault> {self.world3d_retirement_fault}

    pub(crate) fn advance_world3d_retirement_step(&mut self) -> bool {
        let Some((_, state)) = self.retired_world3d_states.front_mut() else { return false };
        if world3d_dynamic_retirement_terminal_is_empty(state) {
            self.retired_world3d_states.pop_front();
            assert!(self.world3d_states.set_external_reservations(self.retired_world3d_states.len()), "retired World3d release restores admission credits");
            return !self.retired_world3d_states.is_empty();
        }
        let now = semio_framework_job::default_now_us();
        let Some(budget) = now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US)) else {
            self.world3d_retirement_fault=Some(infinite_world::world::WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::WorkLimit));
            return true;
        };
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(self.world3d_retirement_epoch),
            semio_framework_job::Generation(1),
            budget,
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut self.world3d_retirement_sequence,
        );

        let terrain_grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64};
        let terrain=infinite_world::world::step_world3d_terrain_retirement(state,terrain_grant,&mut context);
        if !terrain.ownership.fits(terrain_grant) {self.world3d_retirement_fault=Some(infinite_world::world::WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::InvariantViolated));return true;}
        match terrain.step {
            infinite_world::world::WorldTerrainMeshPublicationStep::Idle=>{},
            infinite_world::world::WorldTerrainMeshPublicationStep::Pending|infinite_world::world::WorldTerrainMeshPublicationStep::Complete=>return true,
            infinite_world::world::WorldTerrainMeshPublicationStep::Fault(fault)=>{self.world3d_retirement_fault=Some(fault);return true;}
        }
        step_world3d_dynamic_retirement(state, &mut context);
        if world3d_dynamic_retirement_terminal_is_empty(state) {
            self.retired_world3d_states.pop_front();
            assert!(self.world3d_states.set_external_reservations(self.retired_world3d_states.len()), "retired World3d release restores admission credits");
        }
        !self.retired_world3d_states.is_empty()
    }

    pub(crate) fn close_component_world_step(&mut self, target: &crate::element_contracts::ScenePointerTarget, grant: semio_framework_value::retained_clone::RetainedCloneGrant, context: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_value::retained_clone::RetainedCloneStep,infinite_world::world::WorldDynamicFault> {
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,ValueRefusalKind};
        use infinite_world::world::{WorldDynamicFault,WorldTerrainMeshPublicationStep};
        if !matches!(target.kind, ui_wgpu::wgpu::SurfaceKind::World3d | ui_wgpu::wgpu::SurfaceKind::IconRender) {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if context.is_cancelled() || context.should_yield() || grant.maximum_items==0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        if grant.maximum_depth==0 {return Err(WorldDynamicFault::Ownership(ValueRefusalKind::DepthLimit));}
        if self.component_world3d_retirement.is_none() {
            let Some(token)=self.world3d_states.token(&target.host_id) else {return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));};
            let Some(owner)=self.world3d_states.take_exact_to_retirement(token) else {return Err(WorldDynamicFault::StaleToken);};
            self.component_world3d_retirement=Some(owner);
            context.stage("componentWorldRetirement");
            context.consume_fuel(1);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0}));
        }
        let owner=self.component_world3d_retirement.as_mut().expect("component World3d retirement owner");
        if owner.id!=target.host_id {return Err(WorldDynamicFault::StaleToken);}
        let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut owner.value,grant,context);
        if !terrain.ownership.fits(grant) {return Err(WorldDynamicFault::Ownership(ValueRefusalKind::InvariantViolated));}
        match terrain.step {
            WorldTerrainMeshPublicationStep::Pending|WorldTerrainMeshPublicationStep::Complete=>Ok(RetainedCloneStep::Progress(terrain.ownership)),
            WorldTerrainMeshPublicationStep::Fault(fault)=>Err(fault),
            WorldTerrainMeshPublicationStep::Idle=>Err(WorldDynamicFault::Ownership(ValueRefusalKind::UnsupportedOwner)),
        }
    }

    fn advance_chrome_preferences_load_step(&mut self) {
        let phase = self.chrome_present.maintenance.load_phase;
        match phase {
            0 => {
                let preferences = read_ui_preferences();
                let custom_themes = custom_themes_from(&preferences);
                self.appearance_id = resolve_appearance_id(&preferences);
                self.locale_id = resolve_locale_id(env_lock("SEMIO_LOCKED_LOCALE"), &preferences, self.active_locale());
                crate::interpreter::install_ui_engine_locale(self.active_locale());
                self.terminology_id = env_lock("SEMIO_LOCKED_TERMINOLOGY").or(preferences.terminology).unwrap_or_else(|| self.active_terminology().as_str().to_string());
                self.driver_id = preferences.driver_id.unwrap_or_else(|| "default".to_string());
                self.chrome_build.preferences.custom_drivers = preferences.custom_drivers;
                self.chrome_build.driver = resolve_ui_driver_chrome(&self.driver_id, &self.chrome_build.preferences.custom_drivers);
                self.chrome_build.preferences.ui_layout = match preferences.layout {
                    Some(OsUiChromeLayout::Tablet) => "tablet",
                    _ => "desktop",
                }
                .to_string();
                self.chrome_build.preferences.theme_id = env_lock("SEMIO_LOCKED_THEME").or(preferences.theme_id).unwrap_or_else(|| "semio".to_string());
                self.chrome_build.preferences.custom_themes = custom_themes;
                self.chrome_build.preferences.keybinding_overrides = preferences.keybinding_overrides;
                self.trace_resolved_locale("preferences");
                let loaded = self.chrome_build.preferences.clone();
                with_chrome_prefs(|current| *current = loaded);
            }
            1 => {
                self.chrome_build.preferences.worker_count = stored_field_get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY).and_then(|raw| raw.parse::<u32>().ok()).filter(|count| *count >= 1).unwrap_or_else(default_compute_worker_count);
            }
            _ => {
                self.chrome_present.preferences_loaded = true;
                self.chrome_present.maintenance.load_requested = false;
                self.chrome_present.maintenance.load_phase = 0;
                self.chrome_present.last_synced_preferences = Some(UiPrefsSnapshot::capture(self));
                return;
            }
        }
        self.chrome_present.maintenance.load_phase = phase.saturating_add(1);
    }

    fn advance_panel_layout_persist_step(&mut self) {
        self.persist_dock_ui_if_changed();
        self.chrome_present.maintenance.layout_requested = false;
    }

    pub(crate) fn advance_presence_preview_step(&mut self) {
        self.chrome_present.maintenance.presence_requested = false;
        let Some(channel) = self.sync_channel.as_ref() else { return };
        if channel.document_id.len() > SHELL_CHROME_IO_FIELD_BYTES || channel.plugin_id.len() > SHELL_CHROME_IO_FIELD_BYTES {
            return;
        }
        let actor = self.current_shell_actor(channel.instance_id);
        if actor.len() > SHELL_CHROME_IO_FIELD_BYTES {
            return;
        }
        let _document_id = channel.document_id.clone();
        let connected_at_ms = channel.connected_at_ms;
        let label = Some(shell_presence_label(self.identity.as_ref(), &actor)).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES);
        let user_id = self.identity.as_ref().map(|identity| identity.user_id.clone()).filter(|value| value.len() <= SHELL_CHROME_IO_FIELD_BYTES);
        let (views, active_tool) = self.board_presence_views();
        let ephemeral = self.plugins.iter().find(|entry| entry.plugin_id == channel.plugin_id).and_then(|plugin| plugin.ephemeral_snapshot(channel.instance_id)).unwrap_or_default();
        let (presence_pack, interaction) = (ephemeral.presence, ephemeral.interaction);
        let peer = PresencePeer { actor, label, presence_pack, connected_at_ms, user_id, role: None, drag_ghost_json: None, interaction, color: None, surface: None, views, ui: None, tool_run: ephemeral.tool_run, principal_kind: None, active_tool, history_edit: ephemeral.history_edit, typing: Vec::new() };
        self.document_host.presence_heartbeat_key(&channel.document_key, chrome_now_ms() as u64, peer);
    }

    /// 👕️ Every board window of the attached session as the presence wire names it — camera, size and
    /// the world point under the pointer (`crate::canvas_presence::board_presence_view`) — plus the
    /// active utility of the board under the pointer, React's `publishLocalActiveToolV1`.
    pub(crate) fn board_presence_views(&self) -> (Vec<PresenceWindowView>, Option<String>) {
        let mut active_tool = None;
        let views = self
            .board2d_states
            .iter()
            .filter_map(|(host_id, surface)| {
                let (camera, utility) = crate::engine_canvas::board2d_presence_state(host_id)?;
                let view = crate::canvas_presence::board_presence_view(&surface.window_id, surface.bounds, camera, self.presence_pointer);
                if view.pointer.is_some() {
                    active_tool = utility;
                }
                Some(view)
            })
            .collect();
        (views, active_tool)
    }

    /// 👥️ Every board window's peer overlays from the verified roster: the other actors' cursors,
    /// viewports and marks this shell paints over its boards (`crate::canvas_presence::board_peer_overlays`,
    /// React's `CanvasPresenceOverlayV1`). Nothing before the hub named this connection's own actor.
    pub(crate) fn board_peer_overlays(&self) -> Vec<(Rect, crate::canvas_presence::BoardPeerOverlays)> {
        let Some((actor, color)) = self.presence_self.as_ref() else { return Vec::new() };
        self.board2d_states
            .iter()
            .filter_map(|(host_id, surface)| {
                let (camera, _) = crate::engine_canvas::board2d_presence_state(host_id)?;
                let overlays = crate::canvas_presence::board_peer_overlays(&self.presence_peers, &surface.window_id, surface.bounds, camera, actor, *color, &format!("board/{}", surface.window_id));
                (!overlays.cursors.is_empty() || !overlays.marks.is_empty()).then_some((surface.bounds, overlays))
            })
            .collect()
    }

    fn advance_chrome_preferences_persist_step(&mut self) {
        let phase = self.chrome_present.maintenance.persist_phase;
        let snapshot = UiPrefsSnapshot::capture(self);
        match phase {
            0 if self.chrome_present.last_synced_preferences.as_ref() != Some(&snapshot) => {
                persist_ui_preferences(self, self.chrome_present.last_synced_preferences.as_ref());
            }
            1 if self.chrome_present.last_synced_preferences.as_ref().map(|synced| synced.worker_count) != Some(snapshot.worker_count) => {
                stored_field_set(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, &snapshot.worker_count.to_string());
            }
            0..=1 => {}
            _ => {
                self.chrome_present.last_synced_preferences = Some(snapshot);
                self.chrome_present.maintenance.persist_requested = false;
                self.chrome_present.maintenance.persist_phase = 0;
                return;
            }
        }
        self.chrome_present.maintenance.persist_phase = phase.saturating_add(1);
    }

    pub(crate) fn body_rect(&self, theme: &Theme) -> Rect {
        let top = theme.navbar_height + self.tutorial_bar_reserve(theme);
        Rect::new(0.0, top, self.screen_w, self.screen_h - top - theme.footer_height - self.subfooter_height(theme))
    }

    /// 🎬️ Extra vertical space the tutorial control bar reserves below the navbar while a tutorial is
    /// active — `0.0` otherwise, so `body_rect`/the canvas layout are byte-identical to before this
    /// region existed whenever no tutorial is running.
    fn tutorial_bar_reserve(&self, theme: &Theme) -> f32 {
        if self.tutorial.is_some() {
            tutorial_bar_height(theme)
        } else {
            0.0
        }
    }

    pub(crate) fn shell_uri(&self) -> String {
        self.uri_history.get(self.uri_index).cloned().unwrap_or_else(|| self.session.as_ref().map(|s| format!("os://{}/{}", s.plugin_id, s.app.id)).unwrap_or_else(|| "os://home".into()))
    }

    /// 📑️ The `index`-th folded root tab of ONE anchor, in the anchor's own declared order.
    fn anchor_tab_row_item(&self, anchor: PanelAnchor, index: usize) -> Option<(PanelAnchor, DockTabNode)> {
        self.dock_tabs.tabs(anchor).get(index).map(|node| (anchor, node.clone()))
    }

    pub(crate) fn navbar_leading_tab_row_item(&self, index: usize) -> Option<(PanelAnchor, DockTabNode)> {
        self.anchor_tab_row_item(PanelAnchor::TopLeft, index)
    }

    pub(crate) fn navbar_center_tab_row_item(&self, index: usize) -> Option<(PanelAnchor, DockTabNode)> {
        self.anchor_tab_row_item(PanelAnchor::TopMiddle, index)
    }

    pub(crate) fn navbar_trailing_tab_row_item(&self, index: usize) -> Option<(PanelAnchor, DockTabNode)> {
        let tabs = self.dock_tabs.tabs(PanelAnchor::TopRight);
        let last = tabs.len().checked_sub(1)?;
        tabs.get(last.checked_sub(index)?).cloned().map(|node| (PanelAnchor::TopRight, node))
    }

    fn footer_tab_row_item(&self, index: usize) -> Option<(PanelAnchor, DockTabNode)> {
        let mut seen = 0;
        for anchor in [PanelAnchor::BottomLeft, PanelAnchor::BottomMiddle, PanelAnchor::BottomRight] {
            let tabs = self.dock_tabs.tabs(anchor);
            if index < seen + tabs.len() {
                return Some((anchor, tabs[index - seen].clone()));
            }
            seen += tabs.len();
        }
        None
    }

    fn chrome_tab_band_width(&self, atlas: &mut FontAtlas, theme: &Theme, anchor: PanelAnchor) -> f32 {
        self.dock_tabs
            .tabs(anchor)
            .iter()
            .map(|node| {
                let item = ChromeGroupItem { control_id: "", icon_id: Some(node.icon_id.as_str()), label: Some(node.label.as_str()), active: false, disabled: false, kind: HitKind::Toggle };
                retained_panel_chrome_item_width(atlas, theme, &item).unwrap_or(0.0)
            })
            .sum()
    }

    pub(crate) fn navbar_control_band_width(&self, atlas: &mut FontAtlas, theme: &Theme, controls: &[ShellNavbarControl]) -> f32 {
        controls.iter().map(|control| self.navbar_control_width(atlas, theme, control).unwrap_or(0.0)).sum()
    }

    /// 📐️ The one width resolver used by both centred-band reservation and retained paint.
    pub(crate) fn navbar_control_width(&self, atlas: &mut FontAtlas, theme: &Theme, control: &ShellNavbarControl) -> Option<f32> {
        let label = self.chrome_control_label(&control.control_id, &control.label);
        let item = ChromeGroupItem { control_id: control.control_id.as_str(), icon_id: control.icon_id, label: Some(label.as_str()), active: control.active, disabled: false, kind: HitKind::NavbarItem };
        retained_chrome_group_item_width(atlas, theme, &item).map(|width| resolve_shell_navbar_control_width(width.max(theme.control_height), control.width_policy, theme.root_rem_pixels))
    }

    fn navbar_center_width(&self, atlas: &mut FontAtlas, theme: &Theme) -> f32 {
        let title = self.session.as_ref().map_or_else(|| SHELL_NAVBAR_SESSIONLESS_TITLE.to_string(), |session| shell_navbar_title(&session.app, &self.terminology_id));
        let logo = theme.gap_standard * SHELL_NAVBAR_LOGO_UI_SPACING;
        let mut logo_title = logo + theme.gap_standard + atlas.measure_text(&title, theme.font_size_body).0 + theme.padding_standard * 2.0;
        if let Some(role) = self.session.as_ref().map(|session| session.app.role) {
            let label = shell_surface_role_chip_text(role, self.locale_id == "de");
            let item = ChromeGroupItem { control_id: "", icon_id: None, label: Some(label), active: false, disabled: false, kind: HitKind::Button };
            logo_title += theme.gap_standard + retained_chrome_group_item_width(atlas, theme, &item).unwrap_or(0.0);
        }
        let rows = if crate::boot_locks().example_id.is_empty() { self.session_example_rows() } else { Vec::new() };
        let examples: Vec<ShellNavbarControl> = shell_example_control(&rows, matches!(&self.overlay_state, OverlayState::Dropdown(id) if id == "example"), self.locale_id == "de").into_iter().collect();
        let modes = self.session.as_ref().map(|session| shell_mode_controls(&session.app, session.view_state.active_mode_id.as_deref(), self.active_terminology(), self.active_locale())).unwrap_or_default();
        let roles = self
            .session
            .as_ref()
            .and_then(|session| self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|entry| shell_role_controls(&entry.manifest.apps, &session.app, self.active_terminology(), self.active_locale(), &self.shortcut_table())))
            .unwrap_or_default();
        let widths = [
            logo_title,
            self.navbar_control_band_width(atlas, theme, &examples),
            self.navbar_control_band_width(atlas, theme, &modes),
            self.navbar_control_band_width(atlas, theme, &roles),
            self.chrome_tab_band_width(atlas, theme, PanelAnchor::TopMiddle),
        ];
        let occupied = widths.into_iter().filter(|width| *width > 0.0).collect::<Vec<_>>();
        occupied.iter().sum::<f32>() + theme.gap_standard * 2.0 * occupied.len().saturating_sub(1) as f32
    }

    pub(crate) fn fullscreen_item_width(&self, atlas: &mut FontAtlas, theme: &Theme) -> f32 {
        let is_de = self.locale_id == "de";
        let item = ChromeGroupItem {
            control_id: "ui.fullscreen.toggle",
            icon_id: Some(if self.fullscreen_active { "minimize-2" } else { "maximize-2" }),
            label: Some(if self.fullscreen_active { shell_chrome_string("fullscreen.exit", is_de) } else { shell_chrome_string("fullscreen.toggle", is_de) }),
            active: self.fullscreen_active,
            disabled: false,
            kind: HitKind::Toggle,
        };
        retained_chrome_group_item_width(atlas, theme, &item).unwrap_or(theme.control_height)
    }

    pub(crate) fn navbar_chrome_layout(&self, atlas: &mut FontAtlas, theme: &Theme, width: f32) -> ShellNavbarChromeLayout {
        let leading_right = theme.padding_standard + self.chrome_tab_band_width(atlas, theme, PanelAnchor::TopLeft);
        let fullscreen_width = self.fullscreen_item_width(atlas, theme);
        let top_right_width = self.chrome_tab_band_width(atlas, theme, PanelAnchor::TopRight);
        let fullscreen_left = (width - theme.padding_standard - fullscreen_width).max(theme.padding_standard);
        let trailing_right = fullscreen_left - if top_right_width > 0.0 { theme.gap_standard } else { 0.0 };
        let trailing_left = (trailing_right - top_right_width).max(0.0);
        let occupied = [ShellChromeBandSpan { left: 0.0, right: leading_right }, ShellChromeBandSpan { left: trailing_left, right: width }];
        ShellNavbarChromeLayout { center: shell_chrome_centered_band(width, &occupied, self.navbar_center_width(atlas, theme)), trailing_right }
    }

    fn footer_band_width(&self, atlas: &mut FontAtlas, theme: &Theme, anchor: PanelAnchor) -> f32 {
        self.chrome_tab_band_width(atlas, theme, anchor)
    }

    pub(crate) fn footer_chrome_layout(&self, atlas: &mut FontAtlas, theme: &Theme, width: f32, btn_y: f32, btn_h: f32) -> ShellFooterChromeLayout {
        let mobile = self.mobile_panel_active();
        let bottom_left = if mobile { 0.0 } else { self.footer_band_width(atlas, theme, PanelAnchor::BottomLeft) };
        let bottom_middle = if mobile { 0.0 } else { self.footer_band_width(atlas, theme, PanelAnchor::BottomMiddle) };
        let bottom_right = if mobile { 0.0 } else { self.footer_band_width(atlas, theme, PanelAnchor::BottomRight) };
        let hub = self.hub_connection_state();
        let hub_label = self.hub_footer_label();
        let hub_item = ChromeGroupItem { control_id: "s-hub-connection", icon_id: Some(hub.icon_id()), label: Some(hub_label.as_str()), active: false, disabled: false, kind: HitKind::Button };
        let hub_width = retained_chrome_group_item_width(atlas, theme, &hub_item).unwrap_or(theme.control_height);
        let sign_in_label = shell_chrome_string("hub.signIn", self.locale_id == "de");
        let sign_in_item = ChromeGroupItem { control_id: "framework.hub.signIn", icon_id: Some("user"), label: Some(sign_in_label), active: false, disabled: false, kind: HitKind::Button };
        let sign_in_width = (hub == HubConnectionState::SignedOut).then(|| retained_chrome_group_item_width(atlas, theme, &sign_in_item).unwrap_or(theme.control_height));
        let presence_rows = self.footer_presence_rows();
        let presence_label = ui_wgpu::wgpu::presence_bar_chip_text(&presence_rows, None, self.active_locale());
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
    pub(crate) fn footer_tab_row_rect(&self, atlas: &mut FontAtlas, theme: &Theme, index: usize, width: f32, btn_y: f32, btn_h: f32) -> Option<(PanelAnchor, DockTabNode, Rect)> {
        if self.mobile_panel_active() {
            return None;
        }
        let (anchor, node) = self.footer_tab_row_item(index)?;
        let band = self.dock_tabs.tabs(anchor);
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

    pub(crate) fn navbar_leading_tab_row_rect(&self, atlas: &mut FontAtlas, theme: &Theme, index: usize, lead_x: f32, btn_y: f32, btn_h: f32) -> Option<(PanelAnchor, DockTabNode, Rect)> {
        let (anchor, node) = self.navbar_leading_tab_row_item(index)?;
        let item = ChromeGroupItem { control_id: "", icon_id: Some(node.icon_id.as_str()), label: Some(node.label.as_str()), active: false, disabled: false, kind: HitKind::Toggle };
        let item_w = retained_panel_chrome_item_width(atlas, theme, &item)?;
        Some((anchor, node, Rect::new(lead_x.max(theme.padding_standard), btn_y, item_w, btn_h)))
    }

    fn has_display_tabs(&self) -> bool {
        self.session.as_ref().is_some_and(|s| !s.app.window_kinds.is_empty())
    }

    /// 🧭️ This frame's box for one OPEN anchor: `anchor_panel_rect` plus the anchor's slot in its own
    /// column, so two open anchors in one column band it between them instead of painting over each
    /// other — and the anchor's MEASURED content height, so the panel hugs its content exactly as
    /// React's `<Panel>` does instead of filling its band.
    pub(crate) fn anchor_rect(&self, anchor: PanelAnchor, body: Rect, theme: &Theme) -> Rect {
        let body = Self::panel_band(anchor, body, theme);
        let column: Vec<PanelAnchor> = ANCHOR_COLUMN_ORDER.into_iter().filter(|candidate| candidate.horizontal() == anchor.horizontal() && self.anchor_open(*candidate)).collect();
        let index = column.iter().position(|candidate| *candidate == anchor).unwrap_or(0);
        anchor_panel_rect(anchor, self.anchor_state(anchor).size, self.anchor_content_height(anchor, theme), column.len().max(1), index, body, theme)
    }

    /// 📐️ Extra vertical space a CHROME-HOSTED open panel overhangs its band by, the Rust twin of
    /// React's `chromeHostedOpenPanelPositionStyle` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6894`):
    /// `calc(-1 * (var(--size-large) + var(--size-medium)) / 2)` — from the body's edge back to the
    /// centred `h-medium` row inside the `h-large` navbar/footer band, where `PanelChromeTabBar`
    /// parked the folded toggle. So an open panel's own cap sits IN the chrome band, not below it.
    fn chrome_hosted_panel_overhang(theme: &Theme) -> f32 {
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
    pub(crate) fn panel_band(anchor: PanelAnchor, body: Rect, theme: &Theme) -> Rect {
        let overhang = Self::chrome_hosted_panel_overhang(theme);
        let inset = theme.panel_inset;
        let root_row = theme.control_height;
        match anchor.vertical() {
            "top" => Rect::new(body.x, body.y - overhang - inset + root_row, body.w, (body.h + overhang + inset * 2.0 - root_row).max(0.0)),
            "bottom" => Rect::new(body.x, body.y - inset, body.w, (body.h + overhang + inset * 2.0 - root_row).max(0.0)),
            _ => body,
        }
    }

    /// 📐️ The height one anchor's panel WANTS: its tab bar plus the active leaf's measured document,
    /// plus the content inset the panel adds around that document. `None` while the leaf has no
    /// accepted layout yet (its very first frame), which keeps the anchor on its full column band
    /// rather than collapsing it to a hairline.
    fn anchor_content_height(&self, anchor: PanelAnchor, theme: &Theme) -> Option<f32> {
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
    pub(crate) fn dock_chrome_maps(&self) -> (HashMap<String, String>, HashMap<String, String>) {
        let mut labels = HashMap::new();
        let mut icon_ids = HashMap::new();
        let Some(session) = &self.session else {
            return (labels, icon_ids);
        };
        let terminology = self.active_terminology();
        let locale = self.active_locale();
        let mut instance_titles = session.app.default_layout.as_ref().map(crate::dock::window_layout_instance_titles).unwrap_or_default();
        instance_titles.extend(self.layout_override.as_ref().map(crate::dock::window_layout_instance_titles).unwrap_or_default());
        for (window_id, window_kind_id) in self.dock.window_instances() {
            let Some(kind) = session.app.window_kinds.iter().find(|kind| kind.id == window_kind_id) else {
                continue;
            };
            labels.insert(window_id.clone(), self.window_title_override(&window_id).map(str::to_string).or_else(|| instance_titles.get(&window_id).cloned()).unwrap_or_else(|| kind.label.resolve(terminology, locale).to_string()));
            let icon_id = dock_tab_icon_id_v1(kind.icon_id.as_str(), self.window_icon_override(&window_id), self.world_projection_template.get(&window_id).map(String::as_str));
            if !icon_id.is_empty() {
                icon_ids.insert(window_id, icon_id);
            }
        }
        (labels, icon_ids)
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
    pub(crate) fn plan_dock_windows(&mut self, rect: Rect, theme: &Theme, atlas: &mut FontAtlas) {
        self.dock_canvas_bounds = rect;
        self.dock_axis_separator = theme.gap_standard;
        self.dock.mobile = self.screen_w <= crate::dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX;
        self.dock_view = self.dock.render_view(self.dock_drag.as_ref().map(|drag| &drag.payload));
        if self.space_mode && self.spawned_ui.is_some() {
            self.dock_window_plan = vec![("spawned".to_string(), rect)];
            self.dock_drop_bodies = Vec::new();
            self.dock_drop_tab_bars = Vec::new();
            self.window_content_rects = HashMap::from([("spawned".to_string(), rect)]);
            self.window_silhouettes = HashMap::new();
            return;
        }
        let (labels, _icon_ids) = self.dock_chrome_maps();
        for (window_id, label) in &labels {
            note_dock_control_name(window_id, crate::dock::DockControlName::Body(label), self.locale_id == "de");
        }
        let (bodies, silhouettes) = self.dock_view.stack_body_rects_with_silhouettes(rect, theme, &labels, atlas);
        self.dock_drop_tab_bars = self.dock_view.stack_corner_tab_bar_rects(rect, theme, atlas, &labels);
        self.dock_window_plan = bodies.iter().filter(|(_, _, window_id)| !window_id.is_empty()).map(|(_, body, window_id)| (window_id.clone(), *body)).collect();
        self.window_content_rects = bodies.iter().filter(|(_, _, window_id)| !window_id.is_empty()).map(|(_, body, window_id)| (window_id.clone(), *body)).collect();
        self.window_silhouettes = silhouettes;
        self.dock_drop_bodies = bodies;
        let plan = self.dock_window_plan.iter().map(|(id, body)| format!("{id}@{}x{}+{},{}", body.w.round(), body.h.round(), body.x.round(), body.y.round())).collect::<Vec<_>>().join(" ");
        if self.dock_plan_trace.as_deref() != Some(plan.as_str()) {
            Self::debug_log(&format!("[TRACE] wgpu-shell dock plan canvas={}x{} windows={} {plan}", rect.w.round(), rect.h.round(), self.dock_window_plan.len()));
            self.dock_plan_trace = Some(plan);
        }
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
                if ui_wgpu::wgpu::shell_floor_paints(self.chrome_floor_scope) {
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
                self.plan_dock_windows(rect, theme, atlas);
                self.pane_overlay_hits.clear();
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
                let Some((window_id, window_rect)) = self.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = if self.dock.collect_window_ids().is_empty() { 14 } else { 5 };
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
                let Some((window_id, window_rect)) = self.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                let controller = self.window_document_session(&window_id).map(|session| session.app.controller_id.clone()).unwrap_or_else(|| if window_id == "spawned" { String::new() } else { self.document_controller_id() });
                self.apply_initial_world_projection_template(&window_id);
                let Some(document) = self.take_window_document(&window_id) else {
                    cursor.document = UiDocumentFrameCursor::default();
                    cursor.scalar = 0;
                    cursor.flag = false;
                    cursor.phase = 8;
                    return false;
                };
                let complete = {
                    let scroll_offsets = &mut self.scroll_offsets;
                    let collapsed_sections = &mut self.collapsed_sections;
                    let open_selects = &mut self.open_selects;
                    let widget_maps = &mut self.widget_maps_staging;
                    let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.locale_id == "de"), world3d_states: &mut self.world3d_states, world_resources, window_id: window_id.as_str() };
                    let viewport_height_for_widgets = draw.screen_height();
                    let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
                    ctx.pick_clip = Some(window_rect);
                    render_ui_document_step(&mut cursor.document, &document, window_rect, &mut ctx, window_id.as_str(), controller.as_str(), self.chrome_build.driver.drag, &mut hosts, cx)
                };
                self.restore_window_document(&window_id, document);
                if !complete {
                    if document_opportunity_remains(cursor, SHELL_WINDOW_PAINT_OPPORTUNITIES) {
                        return false;
                    }
                    if cursor.scalar >= SHELL_WINDOW_PAINT_OPPORTUNITIES {
                        Self::debug_log(&format!("[TRACE] wgpu-shell window paint {window_id} exhausted {SHELL_WINDOW_PAINT_OPPORTUNITIES} opportunities parked-in={}", cursor.document.phase_name()));
                    }
                    self.record_document_paint_fault(&window_id);
                } else {
                    self.note_retained_body_painted(&window_id);
                }
                self.register_retained_body_hits(&window_id, window_rect, input);
                cursor.document = UiDocumentFrameCursor::default();
                cursor.scalar = 0;
                cursor.flag = false;
                cursor.phase = 8;
            }
            8 => {
                let Some((window_id, window_rect)) = self.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                if !self.paint_window_measures_step(cursor, draw, overlay, atlas, icons, input, theme, &window_id, window_rect, world_resources, cx) {
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
                let Some((window_id, window_rect)) = self.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                if !self.paint_window_pane_chips_step(cursor, draw, atlas, icons, input, theme, &window_id, window_rect) {
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
                let Some((window_id, window_rect)) = self.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                if !self.paint_window_utilities_step(cursor, draw, atlas, icons, input, theme, &window_id, window_rect) {
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
                let Some((window_id, window_rect)) = self.dock_window_plan.get(cursor.item).cloned() else {
                    cursor.phase = 5;
                    return false;
                };
                if !self.paint_window_projection_step(cursor, draw, overlay, atlas, icons, input, theme, &window_id, window_rect, world_resources, cx) {
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
                let Some((window_id, window_rect)) = self.dock_window_plan.get(cursor.item).cloned() else {
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
                let Some((window_id, window_rect)) = self.dock_window_plan.get(cursor.item).cloned() else {
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
                let german = self.locale_id == "de";
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
                        self.error = Some("Shell empty dock notice exceeded the retained glyph boundary".into());
                        cursor.phase = 5;
                    }
                }
            }
            5 => {
                if !(self.space_mode && self.spawned_ui.is_some()) {
                    let (labels, icon_ids) = self.dock_chrome_maps();
                    let bounds = self.dock_canvas_bounds;
                    let german = self.locale_id == "de";
                    let mut names = |id: &str, name: crate::dock::DockControlName<'_>| note_dock_control_name(id, name, german);
                    let mut ctx = DockRenderContext { draw, atlas, icons, input, theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: Some(&mut names) };
                    self.dock_view.paint_chrome(&mut ctx, bounds, false);
                    self.dock_view.register_resize_hits(&mut ctx, bounds);
                }
                cursor.phase = 6;
            }
            6 => {
                if let Some((drag, zone)) = self.dock_drag.as_ref().and_then(|drag| drag.drop_zone.as_ref().map(|zone| (drag, zone))) {
                    if let Some(indicator) = drop_zone_indicator_rect(zone, &self.dock_drop_tab_bars, &self.dock_drop_bodies, self.dock_canvas_bounds, theme.gap_standard) {
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

    /// 🎯️ Completes a tab drag: the anchor under the pointer wins, and a drop back on the tab's own
    /// anchor is a no-op — React's `handleTabDockDrop` over `computeTabDockDropZone`'s anchor pick.
    ///
    /// 🫳️ The index the preview pill painted, so what drops is what was shown: the chip half the
    /// pointer released over, or the row's end.
    pub(crate) fn finish_dock_tab_drag(&mut self, tab_id: &str, from_anchor: PanelAnchor, x: f32, y: f32, theme: &Theme) {
        let Some(target_anchor) = self.anchor_at_point(x, y, theme) else { return };
        if target_anchor == from_anchor {
            return;
        }
        let index = self.dock_tab_drop_index(target_anchor, x, y, theme).unwrap_or_else(|| self.dock_tabs.tabs(target_anchor).len());
        if self.move_dock_tab(tab_id, &DockTabMoveTarget { anchor: target_anchor, parent_path: Vec::new(), index }) {
            self.anchor_state_mut(target_anchor).visible = true;
            Self::debug_log(&format!("[TRACE] wgpu-shell dock tab {tab_id} re-anchored {} -> {}", from_anchor.as_str(), target_anchor.as_str()));
        }
    }

    /// 🫳️ The ROOT-ROW index a drop at `(x, y)` lands at inside `anchor`, over the same chip-half rule
    /// the insert preview paints — `None` when the point is not on that anchor's own root tab row (a
    /// drop anywhere else in the anchor appends, React's own `index: buttons.length` fallback).
    fn dock_tab_drop_index(&self, anchor: PanelAnchor, x: f32, y: f32, theme: &Theme) -> Option<usize> {
        if !self.anchor_open(anchor) {
            return None;
        }
        let panel = self.anchor_rect(anchor, self.body_rect(theme), theme);
        let rows = self.anchor_tab_rows(anchor);
        let row_y = flowed_anchor_tab_row_y(anchor.flow().block, panel, 0, rows.len(), theme.control_height);
        let row_rect = Rect::new(panel.x, row_y, panel.w, theme.control_height);
        if !row_rect.contains(x, y) {
            return None;
        }
        let row = self.dock_tabs.tabs(anchor);
        let dragged = self.dock_tab_drag.as_ref().map(|(tab_id, _)| tab_id.as_str());
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
    fn anchor_at_point(&self, x: f32, y: f32, theme: &Theme) -> Option<PanelAnchor> {
        let body = self.body_rect(theme);
        for anchor in self.open_anchors() {
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

    fn panel_tab_scroll_id(anchor: PanelAnchor, row_index: usize) -> String {
        format!("shell.panel.tabs.{}.{}.scroll", anchor.as_str(), row_index)
    }

    pub(crate) fn reveal_panel_tab_row(&mut self, anchor: PanelAnchor, tab_id: &str) {
        if let Some(row_index) = self.anchor_panel_tab_rows(anchor).iter().position(|row| row.iter().any(|node| node.id == tab_id)) {
            self.scroll_offsets.insert(Self::panel_tab_scroll_id(anchor, row_index), f32::MAX);
        }
    }

    /// 📑️ Paints and hit-tests one anchor's tab bar: a branch row toggles open/shut, a leaf row selects,
    /// and every row is a `PanelTab` drag source so a tab can be carried to another anchor.
    ///
    /// 🫳️ While a tab drag is live, the row under the pointer paints the INSERT PREVIEW — React's
    /// `panelTabInsertPreviewClass` (`🧭️PanelTabBar/🟦️.tsx`), a `w-0.5` accent pill the height of the
    /// row, drawn before the chip the drop would land in front of (and after the last chip when the
    /// drop index is past the end). The index is React's own rule: the half of the chip the pointer is
    /// over (`fraction >= 0.5` → after), or the row's length when the pointer is past every chip.
    pub(crate) fn paint_anchor_tab_bar(&mut self, anchor: PanelAnchor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, panel: Rect) {
        let rows = self.anchor_panel_tab_rows(anchor);
        let active: Vec<String> = self.anchor_state(anchor).path.clone();
        let active_offset = usize::from(matches!(anchor.vertical(), "top" | "bottom"));
        let row_h = theme.control_height;
        let dragging = self.dock_tab_drag.is_some();
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
            let scroll_id = Self::panel_tab_scroll_id(anchor, row_index);
            let offset = self.scroll_offsets.get(&scroll_id).copied().unwrap_or_default().clamp(0.0, max_offset);
            self.scroll_offsets.insert(scroll_id.clone(), offset);
            input.register_hit(HitTarget { rect: row_rect, event: None, control_id: Some(scroll_id), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
            let mut x = row_rect.x - offset;
            let preview = dragging.then(|| self.dock_tab_insert_preview(row, atlas, theme, row_rect, offset, input.pointer_x, input.pointer_y)).flatten();
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
                let icon_id = if node.is_branch() && self.dock_branch_collapsed(anchor, &node.id) { "chevron-right" } else { node.icon_id.as_str() };
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
                    self.chrome_build.register_element_rect(panel_tab_introduction_element_id(&node.id), visible);
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

    /// 🫳️ The x the insert preview paints at for one tab row, or `None` when the pointer is outside it.
    /// React's `computeTabDockDropZone` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`): the chip the pointer is over
    /// decides by its own half (`fraction >= 0.5` inserts after), and a pointer inside the row but past
    /// every chip inserts at the row's end. The dragged tab's own chip is excluded, exactly as React's
    /// `excludedIds` excludes the dragged subtree.
    #[allow(clippy::too_many_arguments, reason = "the tab row's own geometry, forwarded unchanged")]
    fn dock_tab_insert_preview(&self, row: &[DockTabNode], atlas: &mut FontAtlas, theme: &Theme, row_rect: Rect, offset: f32, pointer_x: f32, pointer_y: f32) -> Option<f32> {
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

    #[allow(clippy::too_many_arguments, reason = "one retained panel document opportunity forwards the host frame context")]
    fn render_panel_document_opportunity(
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
        let controller = self.document_controller_id();
        let document = self.panel_documents.remove(window)?;
        crate::interpreter::set_ui_document_flow(window, anchor.flow());
        let complete = {
            let scroll_offsets = &mut self.scroll_offsets;
            let collapsed_sections = &mut self.collapsed_sections;
            let open_selects = &mut self.open_selects;
            let widget_maps = &mut self.widget_maps_staging;
            let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.locale_id == "de"), world3d_states: &mut self.world3d_states, world_resources, window_id: window };
            let viewport_height_for_widgets = panel_draw.screen_height();
            let mut ctx = framework_widget_context(panel_draw, overlay, atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
            ctx.pick_clip = Some(content);
            render_ui_document_step(&mut cursor.document, &document, content, &mut ctx, window, controller.as_str(), self.chrome_build.driver.drag, &mut hosts, cx)
        };
        self.panel_documents.insert(window.to_string(), document);
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
    pub(crate) fn render_panel_step(
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
                cursor.window = self.anchor_state(anchor).active_tab().and_then(UiText::try_from_str);
                cursor.flag = cursor.window.as_ref().is_none_or(|window| !self.panel_documents.contains_key(window.as_str()));
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
                    let content = self.anchor_content_rect(anchor, panel, theme);
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
                let content = self.anchor_content_rect(anchor, panel, theme);
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
                if !self.panel_documents.contains_key(window.as_str()) {
                    cursor.phase = 9;
                    return false;
                }
                let content = self.anchor_content_rect(anchor, panel, theme);
                let complete = self.render_panel_document_opportunity(cursor, window.as_str(), content, panel_draw, overlay, atlas, icons, input, theme, world_resources, anchor, cx).unwrap_or(false);
                if !complete {
                    if !cursor.document.terminal_is_fault() {
                        return false;
                    }
                    self.record_document_paint_fault(window.as_str());
                } else {
                    self.note_retained_body_painted(window.as_str());
                }
                self.register_retained_panel_scroll_region(window.as_str(), content, input);
                self.register_retained_body_hits(window.as_str(), content, input);
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
    fn render_mobile_panel_step(
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
        let tabs = self.mobile_panel_tabs();
        if !self.mobile_panel_visible || tabs.is_empty() {
            return true;
        }
        match cursor.phase {
            0 => {
                let path = self.mobile_panel_path.clone();
                let path = mobile_panel_reconcile_path(&tabs, &path);
                self.mobile_panel_path = path.clone();
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
                let controller = self.document_controller_id();
                let Some(document) = self.panel_documents.remove(window.as_str()) else {
                    cursor.phase = 3;
                    return false;
                };
                let complete = {
                    let scroll_offsets = &mut self.scroll_offsets;
                    let collapsed_sections = &mut self.collapsed_sections;
                    let open_selects = &mut self.open_selects;
                    let widget_maps = &mut self.widget_maps_staging;
                    let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.locale_id == "de"), world3d_states: &mut self.world3d_states, world_resources, window_id: window.as_str() };
                    let viewport_height_for_widgets = panel_draw.screen_height();
                    let mut ctx = framework_widget_context(panel_draw, None, atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
                    ctx.pick_clip = Some(content);
                    render_ui_document_step(&mut cursor.document, &document, content, &mut ctx, window.as_str(), controller.as_str(), self.chrome_build.driver.drag, &mut hosts, cx)
                };
                self.panel_documents.insert(window.as_str().to_string(), document);
                if !complete {
                    if !cursor.document.terminal_is_fault() {
                        return false;
                    }
                    self.record_document_paint_fault(window.as_str());
                } else {
                    self.note_retained_body_painted(window.as_str());
                }
                self.register_retained_panel_scroll_region(window.as_str(), content, input);
                self.register_retained_body_hits(window.as_str(), content, input);
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
    pub(crate) fn paint_mobile_tab_bar(&self, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, panel: Rect, tabs: &[DockTabNode]) {
        let mut x = panel.x + theme.gap_standard;
        let active = self.mobile_panel_path.first().cloned().unwrap_or_default();
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
//#endregion ShellChrome
