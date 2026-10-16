//! 🪀️ `shell-view-panels` staging of `pointer-input`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl<'ctx> PanelsViewCtx<'ctx> {
    //#region 🎯️RetainedPointerRouting
    /// 🏁️ Promotes one GPU-accepted frame's complete input authority in one runtime lock. The chrome's
    /// accessibility publication is replaced (and its generation advanced) only when its projection
    /// changed, so the mirror's addresses stay valid across frames that present the same chrome.
    ///
    /// ♿️ The chrome's accessible names ride the SAME promotion as its hit registry (packet W15d)
    /// — a production path, not a diagnostics one, so an assistive technology reads the chrome
    /// whether or not `SEMIO_RUNTIME_DIAGNOSTICS` is armed.
    pub fn acknowledge_presented_input(&mut self, input: &mut InputState<ActionDescriptor>, witness: PresentedInputCandidateWitness) -> bool {
        if !self.input.presented_input_candidate_matches(witness) {
            return false;
        }
        if !semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::acknowledge_presented_input(witness.0) {
            return false;
        }
        semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_host_temporal_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_table_stepper_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_table_editable_text_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_table_button_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_vfs_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_block_list_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_event_feed_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_world3d_accessibility_candidates(witness.0);
        semio_framework_os_renderer_wgpu_scene_feeds::acknowledge_graph_timeline_accessibility_candidates(witness.0);
        std::mem::swap(&mut self.input.retained_hit_windows, &mut self.input.retained_hit_windows_staging);
        std::mem::swap(&mut self.input.retained_scene_hits, &mut self.input.retained_scene_hits_staging);
        std::mem::swap(&mut self.input.widget_maps, &mut self.input.widget_maps_staging);
        std::mem::swap(&mut self.input.presented_input_geometry, &mut self.input.presented_input_geometry_staging);
        input.publish_hits();
        self.input.presented_input_epoch = witness.0;
        self.input.presented_input_candidate = None;
        semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::publish_accessibility_visible_documents();
        self.input.resolve_time_travel_focus(input, &mut self.chrome, &mut self.history);
        let chrome_accessibility = self.chrome_accessibility_nodes(input.hits());
        let focus_changed = self.chrome.presented_chrome_accessibility.iter_mut().zip(&chrome_accessibility).fold(false, |changed, (previous, current)| {
            let focus_changed = previous.focused != current.focused;
            previous.focused = current.focused;
            changed || focus_changed
        });
        let authority_changed = self.chrome.presented_chrome_accessibility_generation == 0 || chrome_accessibility != self.chrome.presented_chrome_accessibility;
        if authority_changed {
            self.chrome.presented_chrome_accessibility = chrome_accessibility;
            self.chrome.presented_chrome_accessibility_generation = witness.0;
        }
        if authority_changed || focus_changed {
            semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::note_chrome_accessibility(self.chrome.presented_chrome_accessibility_generation, self.chrome.presented_chrome_accessibility.clone());
        }
        semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::note_chrome_hit_registry(input.hits(), &self.input.retained_hit_windows);
        semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::note_chrome_surfaces(&self.input.chrome_surface_census(&self.chrome, &self.dock));
        #[cfg(not(target_arch = "wasm32"))]
        semio_framework_os_renderer_wgpu_native_platform::native_accessibility::publish_native_accessibility(semio_framework_os_renderer_wgpu_native_platform::native_accessibility::NativeAccessibilityPublication { title: semio_framework_os_renderer_wgpu_host_base::boot_window_title(), windows: semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::native_accessibility_windows() });
        true
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn publish_retained_hit_registry(&mut self, input: &mut InputState<ActionDescriptor>) {
        self.publish_retained_input_for_test(input, &Theme::default());
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn publish_retained_input_for_test(&mut self, input: &mut InputState<ActionDescriptor>, theme: &Theme) {
        let witness = self.as_chrome_view().seal_presented_input_candidate(theme).expect("test input candidate witness");
        assert!(self.acknowledge_presented_input(input, witness));
    }
    //#endregion 🎯️RetainedPointerRouting
}

//#endregion ShellInput

