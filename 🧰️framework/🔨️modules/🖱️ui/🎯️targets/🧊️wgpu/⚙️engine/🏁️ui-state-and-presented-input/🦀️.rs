//! 🏁️ Ui state and presented input: the `Ui` state, the retained input draft, the constructor, the theme and driver settings, the presented input candidates and the surface close. See [`super`].

use super::*;

// #region engine
//#region 🔖️Ui
/// 🧵️ Assembles the individually-milestoned retained modules into the one façade a host drives per
/// tick: `apply_tree` runs `reconcile`, `frame` runs `flex` (dirty-gated) then `paint` then hands
/// `scene_slots` to an optional `SceneHost`, `dispatch_event` runs `events::EventRouter`, and
/// `needs_frame` reads the same dirty flags `frame` itself gates on. One `UiWindow` per window id
/// (app-content trees); window-chrome (dock/split/tab) is the separate `🐚️Shell` this façade also owns,
/// since `shell`'s own doc comment models it as independent of any single window's content tree.
/// Never submits to the GPU itself — `frame` returns a `&DrawList` for the caller to hand to the
/// existing `gpu::GpuContext::render_frame`, exactly like the immediate-mode `widgets` path's callers
/// already do; wiring that hand-off into a real host event loop is later, renderer-thinning work.
pub struct Ui {
    pub windows: UiSurfaceRegistry,
    pub(super) document_generation: u64,
    pub(super) shell: Shell,
    pub theme: Theme,
    pub(super) driver_drag: UiDriverDrag,
    pub(super) pending_commands: Vec<UiCommand>,
    sealed_visibility_candidate: Option<UiSealedVisibilityCandidate>,
    pub layout_queues: [SurfaceLaneRing; 3],
    pub layout_pressure: Option<SurfaceLaneEntry>,
    pub(super) lane_cursor: usize,
    pub theme_propagation: Option<ThemePropagationCursor>,
    pub(super) pending_theme: Option<Theme>,
    pub(super) theme_fault: bool,
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) const _: fn() = || {
    fn assert_send<T: Send>() {}
    assert_send::<Ui>();
};

/// 🧾️ The exact retained input generation and submitted text held until one dispatch settles.
#[derive(Clone, Debug)]
pub struct RetainedInputDraft {
    pub surface: UiSurfaceToken,
    pub node: NodeId,
    pub key: semio_framework_ui_retained_tree::tree::NodeKey,
    pub text: String,
    pub action: semio_framework_ui_declaration::layout::ActionDescriptor,
}

impl Ui {
    pub fn new(locale: semio_framework_ui_locale::Locale) -> Self {
        Self {
            windows: UiSurfaceRegistry::default(),
            document_generation: 0,
            shell: Shell::new(locale),
            theme: Theme::default(),
            driver_drag: UiDriverDrag::Handle,
            pending_commands: Vec::new(),
            sealed_visibility_candidate: None,
            layout_queues: std::array::from_fn(|_| SurfaceLaneRing::default()),
            layout_pressure: None,
            lane_cursor: 0,
            theme_propagation: None,
            pending_theme: None,
            theme_fault: false,
        }
    }

    /// 🌐️ Re-sets the language the engine's own chrome strings resolve in — the host's resolved locale changed.
    pub fn set_locale(&mut self, locale: semio_framework_ui_locale::Locale) {
        self.shell.set_locale(locale);
    }

    pub fn set_theme(&mut self, theme: Theme) {
        let layout_changed = theme_layout_identity(&self.theme) != theme_layout_identity(&theme);
        if !layout_changed && self.theme_propagation.is_none() {
            self.theme = theme;
            return;
        }
        if self.theme_propagation.is_some() {
            self.pending_theme = Some(theme);
        } else {
            self.theme_propagation = Some(ThemePropagationCursor::new(theme));
        }
    }

    pub fn set_driver_drag(&mut self, driver_drag: UiDriverDrag) {
        if self.driver_drag == driver_drag {
            return;
        }
        self.driver_drag = driver_drag;
        let metrics = TreeRowMetrics::from_theme(&self.theme);
        let mut commands = Vec::new();
        for window in self.windows.values_mut().filter(|window| window.closing.is_none()) {
            let inline = window.router.flow().inline;
            window.router.set_control_border(self.theme.stroke_hairline);
            window.router.set_control_gap(self.theme.gap_standard);
            commands.extend(window.router.set_tree_drag_policy(&mut window.tree, driver_drag, metrics.with_inline(inline)));
            let Some(next_revision) = window.theme_revision.checked_add(1) else { continue };
            window.theme_revision = next_revision;
            if let Some(root) = window.tree.root {
                window.tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
            }
        }
        self.pending_commands.extend(commands);
    }

    pub fn driver_drag(&self) -> UiDriverDrag {
        self.driver_drag
    }

    #[expect(clippy::result_large_err, reason = "Surface and document admission return the exact refused identity or page without allocating a rejection wrapper.")]
    pub fn try_admit_surface(&mut self, window_id: &str) -> Result<UiSurfaceToken, UiSurfaceAdmissionRejected> {
        let id = SurfaceId::try_from(window_id).map_err(|_| UiSurfaceAdmissionRejected { id: SurfaceId::default() })?;
        self.windows.try_admit(id)
    }

    /// 🪪️ Exact registry lifetime; document epochs are exposed separately by surface_generation.
    pub fn surface_token(&self, window_id: &str) -> Option<UiSurfaceToken> {
        self.windows.token(window_id)
    }

    /// 🎞️ Seals every visible candidate interaction revision under the Shell's exact presenter
    /// witness. A live presented capture holds publication until its matching terminal event.
    pub fn seal_presented_input_candidate(&mut self, witness: u64, visible_windows: &[String]) -> bool {
        if self.sealed_visibility_candidate.is_some() || self.windows.values().any(|window| window.sealed_input_candidate.is_some()) {
            eprintln!("[TRACE] seal refuse: existing seals visible={visible_windows:?}");
            return false;
        }
        if visible_windows.iter().any(|window_id| self.windows.get(window_id).is_some_and(|window| window.closing.is_none() && window.presented_ready && !window.candidate_ready)) {
            eprintln!("[TRACE] seal refuse: presented without candidate visible={visible_windows:?}");
            return false;
        }
        self.sealed_visibility_candidate = Some(UiSealedVisibilityCandidate {
            witness,
            surfaces: std::array::from_fn(|slot| {
                let token = self.windows.token_at(slot)?;
                let id = self.windows.id(token)?;
                Some((token, visible_windows.iter().any(|window_id| window_id == id.as_ref())))
            }),
        });
        for window_id in visible_windows {
            let Some(window) = self.windows.get_mut(window_id).filter(|window| window.closing.is_none()) else { continue };
            if !window.candidate_ready {
                continue;
            }
            if window.presented_ready {
                window.tree.transfer_composite_interaction_from(&window.presented_tree);
                window.router.transfer_interaction_from(&window.presented_router, &window.presented_tree, &window.tree);
            }
            window.sealed_input_candidate = Some((witness, window.candidate_base_interaction_epoch));
        }
        true
    }

    pub fn presented_input_candidate_matches(&self, witness: u64) -> bool {
        self.windows.values().all(|window| {
            let input_matches = match window.sealed_input_candidate {
                Some((candidate, base)) => candidate == witness && base == window.presented_interaction_epoch,
                None => true,
            };
            input_matches
        }) && self.sealed_visibility_candidate.as_ref().is_none_or(|candidate| candidate.witness == witness)
    }

    pub fn candidate_is_sealed_for(&self, window_id: &str, witness: u64) -> bool {
        self.windows.get(window_id).filter(|window| window.closing.is_none()).is_some_and(|window| window.sealed_input_candidate.is_some_and(|(candidate, base)| candidate == witness && base == window.presented_interaction_epoch))
    }

    pub fn candidate_scene_node_for_presented_node(&self, window_id: &str, witness: u64, presented_node: NodeId) -> Option<NodeId> {
        let window = self.windows.get(window_id).filter(|window| window.closing.is_none())?;
        let sealed = window.sealed_input_candidate.is_some_and(|(candidate, base)| candidate == witness && base == window.presented_interaction_epoch);
        if !sealed {
            return None;
        }
        let document_id = window.presented_tree.document_id(presented_node)?;
        let candidate_node = window.tree.document_node(document_id)?;
        matches!(&window.presented_tree.node(presented_node)?.spec.0, UiNode::ComponentScene(_)).then_some(())?;
        matches!(&window.tree.node(candidate_node)?.spec.0, UiNode::ComponentScene(_)).then_some(candidate_node)
    }

    pub fn presented_component_scene_matches(&self, witness: &UiComponentSceneWitness<'_>) -> bool {
        let Some(window) = self.windows.get(witness.window_id).filter(|window| window.closing.is_none() && window.presented_ready && window.presented_accessibility_generation == witness.window_generation) else {
            return false;
        };
        let Some(node) = window.presented_tree.document_node(witness.document_id).and_then(|node| window.presented_tree.node(node)) else {
            return false;
        };
        let UiNode::ComponentScene(scene) = &node.spec.0 else { return false };
        node.key.borrowed() == witness.key && node.component_generation() == witness.component_generation && scene.host_id == witness.host_id && scene.component_kind == witness.kind && scene.surface_id == witness.surface_id
    }

    /// 📐️ Resolves the exact candidate scene node and painted rectangle for one presented node.
    pub fn candidate_scene_geometry_for_presented_node(&self, window_id: &str, witness: u64, presented_node: NodeId) -> Option<(NodeId, semio_framework_ui_theme::geometry::Rect)> {
        let node = self.candidate_scene_node_for_presented_node(window_id, witness, presented_node)?;
        Some((node, self.windows.get(window_id)?.tree.absolute_rect(node)?))
    }

    /// 🏁 Swaps bounded move-owned tree/router revisions only after the matching pixels are accepted.
    pub fn acknowledge_presented_input(&mut self, witness: u64) -> bool {
        if self.windows.values().any(|window|window.presented_source.is_some()){return false}
        if !self.presented_input_candidate_matches(witness) {
            return false;
        }
        for window in self.windows.values_mut().filter(|window| window.sealed_input_candidate.is_some_and(|(candidate, _)| candidate == witness)) {
            let presented_revision = window.presented_revision;
            std::mem::swap(&mut window.tree, &mut window.presented_tree);
            std::mem::swap(&mut window.intrinsic_content_height, &mut window.presented_intrinsic_content_height);
            std::mem::swap(&mut window.router, &mut window.presented_router);
            window.presented_revision = window.revision;
            if presented_revision != window.presented_revision {
                window.presented_tooltip = None;
            }
            window.presented_accessibility_generation = window.accessibility_generation;
            window.presented_ready = true;
            if !window.accepted_scene_retirements.is_empty() {
                debug_assert!(window.scene_retirements.is_empty());
                std::mem::swap(&mut window.scene_retirements, &mut window.accepted_scene_retirements);
            }
            window.candidate_baseline = window.presented_tree.document().map(|source| Box::new(UiCandidateBaseline::new(source)));
            window.candidate_ready = false;
            window.candidate_reconciled = false;
            window.candidate_retirements_resolved = false;
            window.candidate_preserves_prior_mounts = false;
            window.candidate_interaction_cursor = None;
            window.sealed_input_candidate = None;
            window.candidate_base_interaction_epoch = window.presented_interaction_epoch;
        }
        let visibility = self.sealed_visibility_candidate.take().filter(|candidate| candidate.witness == witness);
        if let Some(visibility) = visibility {
            for (token, visible) in visibility.surfaces.into_iter().flatten() {
                if visible {
                    continue;
                }
                let Some(window) = self.windows.get_token_mut(token) else { continue };
                let presented_changed = window.presented_router.suspend_clock(&mut window.presented_tree);
                window.router.suspend_clock(&mut window.tree);
                let tooltip_changed = window.presented_tooltip.take().is_some();
                if window.presented_ready && (presented_changed || tooltip_changed) {
                    window.advance_presented_interaction();
                }
            }
        }
        true
    }

    pub fn discard_presented_input_candidate(&mut self, witness: u64) -> bool {
        let mut matched = false;
        for window in self.windows.values_mut() {
            if window.sealed_input_candidate.is_some_and(|(candidate, _)| candidate == witness) {
                window.sealed_input_candidate = None;
                matched = true;
            }
        }
        if self.sealed_visibility_candidate.as_ref().is_some_and(|candidate| candidate.witness == witness) {
            self.sealed_visibility_candidate = None;
            matched = true;
        }
        matched || (self.sealed_visibility_candidate.is_none() && !self.windows.values().any(|window| window.sealed_input_candidate.is_some()))
    }

    /// 🔭️ Observes a retained worker refusal without advancing or releasing its owner.
    pub fn layout_retirement_refusal(&self, window_id: &str) -> Option<UiWorkerRetirementRefusal> { self.windows.get(window_id).and_then(|window| window.layout_retirement_refusal) }

    /// 🧳️ Lends the complete original worker refusal independently of its compact UI notice.
    pub fn layout_retirement_failure(&self,window_id:&str)->Option<&semio_framework_job::OriginalTurnFailure>{let window=self.windows.get(window_id)?;window.layout_close_failure.as_ref().and_then(|owner|owner.original()).or_else(||window.layout_session.as_ref()?.checked_out_failure())}
    /// ⏳️ The same worker failure remains owned while its funded frontier is closing.
    pub fn layout_retirement_failure_pending(&self,window_id:&str)->bool{self.windows.get(window_id).is_some_and(|window|window.layout_close_failure.is_some()||window.layout_session.as_ref().is_some_and(|session|session.checked_out_failure().is_some()))}

    /// 🛑️ Latches retirement before topology removal and refuses further surface admission.
    pub fn begin_surface_close(&mut self, token: UiSurfaceToken) -> bool {
        let Some(window) = self.windows.get_token_mut(token) else { return false };
        if window.closing.is_none() {
            window.presented_tooltip = None;
            window.closing = Some(UiSurfaceClosePhase::Router);
        }
        true
    }

    /// 🧵️ Drives the sole close owner through input, jobs, paint, documents, and registry release.
    pub fn close_surface_one(&mut self,token:UiSurfaceToken,cx:&mut StepContext<'_>)->Result<UiSurfaceCloseStep,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(UiSurfaceCloseStep::Pending)}
        if let Some(window)=self.windows.get_token_mut(token){if let Some(failure)=window.layout_close_failure.as_mut(){let demand=failure.selected_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|failure.step(grant))?;if failure.terminal_is_empty(){window.layout_close_failure=None;window.layout_retirement_refusal=None;}return Ok(UiSurfaceCloseStep::Pending)}}
        Ok((||{
        let Some(window) = self.windows.get_token_mut(token) else { return UiSurfaceCloseStep::Complete };
        let Some(phase) = window.closing else { return UiSurfaceCloseStep::Pending };
        if window.layout_retirement_refusal.is_some_and(|refusal|refusal.stage=="Layout.WorkerRefusal")&&window.layout_session.as_ref().is_some_and(|session|session.checked_out_failure().is_some()){window.layout_retirement_refusal=None;}
        if let Some(refusal) = window.layout_retirement_refusal {
            if !matches!(refusal.stage, "Layout.Admission" | "Layout.CloseAdmission") { return UiSurfaceCloseStep::Refused(refusal); }
            if !window.layout_admission.terminal_is_empty() {
                if let Err(error) = window.layout_admission.close_step(ui_contract::UI_WORKER_RETIREMENT_POLICY, None) {
                    let refusal = UiWorkerRetirementRefusal { stage: "Layout.CloseAdmission", kind: error.kind, progress: error.retained_progress() };
                    assert!(window.layout_admission.failure.is_none(), "original admission failure slot remains available after its prior error transfer");
                    *window.layout_admission.failure = Some(error);
                    window.layout_retirement_refusal = Some(refusal);
                    return UiSurfaceCloseStep::Refused(refusal);
                }
                return UiSurfaceCloseStep::Pending;
            }
            window.layout_retirement_refusal = None;
        }
        if let Some(retirement) = window.scene_retirements.pop_front() {
            return UiSurfaceCloseStep::RetiredScene(retirement);
        }
        if let Some(retirement) = window.accepted_scene_retirements.pop_front() {
            return UiSurfaceCloseStep::RetiredScene(retirement);
        }
        if let Some(retirement) = window.candidate_scene_retirements.pop_front() {
            return UiSurfaceCloseStep::RetiredScene(retirement);
        }
        let next = match phase {
            UiSurfaceClosePhase::Router => {
                if !window.router.close_step() {
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::PresentedRouter
            }
            UiSurfaceClosePhase::PresentedRouter => {
                if !window.presented_router.close_step() {
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::LayoutAdmission
            }
            UiSurfaceClosePhase::LayoutAdmission => {
                if !window.layout_admission.terminal_is_empty() {
                    if let Err(error) = window.layout_admission.close_step(ui_contract::UI_WORKER_RETIREMENT_POLICY, None) {
                        let refusal = UiWorkerRetirementRefusal { stage: "Layout.CloseAdmission", kind: error.kind, progress: error.retained_progress() };
                        window.layout_retirement_refusal = Some(refusal);
                        return UiSurfaceCloseStep::Refused(refusal);
                    }
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::LayoutSession
            }
            UiSurfaceClosePhase::LayoutSession => {
                if let Some(session) = window.layout_session.as_mut() {
                    let admitted = match close_layout_worker_session(session,cx) {
                        Ok(progress) => progress.is_some(),
                        Err(error)=>{let refusal=window.retain_layout_close_failure("Layout.CloseSession",error);return UiSurfaceCloseStep::Refused(refusal)}
                    };
                    if admitted && session.terminal_is_empty() {
                        window.layout_session = None;
                        window.layout_session_generation = None;
                    }
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::LayoutJob
            }
            UiSurfaceClosePhase::LayoutJob => {
                if let Some(job) = window.layout_job.as_mut() {
                    job.begin_close();
                    let grant=cx.retained_grant();
                    let step=match semio_framework_job::InteractiveJob::close_step(job,cx).and_then(|step|step.admit(grant,job.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)){Ok(step)=>step,Err(failure)=>{let refusal=window.retain_layout_close_failure("Layout.CloseUnsubmitted",failure);return UiSurfaceCloseStep::Refused(refusal)}};
                    if matches!(step, semio_framework_job::InteractiveJobCloseStep::Complete { .. }) {
                        window.layout_job = None;
                    }
                    return UiSurfaceCloseStep::Pending;
                }
                window.layout_closing = false;
                UiSurfaceClosePhase::Previews
            }
            UiSurfaceClosePhase::Previews => {
                if window.layout_preview.take().is_some() || window.glyph_preview.take().is_some() {
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::Paint
            }
            UiSurfaceClosePhase::Paint => {
                if let Some(draw) = window.retiring_draw.as_mut() {
                    if draw.retire_step() && draw.retirement_is_empty() {
                        window.retiring_draw = None;
                    }
                    return UiSurfaceCloseStep::Pending;
                }
                if let Some(frame) = window.paint_frame.as_mut() {
                    frame.node_paint.cancel_draw_route(&mut frame.candidate);
                    if !frame.node_sync.close_step() || !frame.node_paint.close_step() || !frame.scene_paint.close_step() {
                        return UiSurfaceCloseStep::Pending;
                    }
                    if frame.hit_candidates.pop().is_some() {
                        return UiSurfaceCloseStep::Pending;
                    }
                    if frame.hit_candidates.capacity() > 0 {
                        frame.hit_candidates = Vec::new();
                        return UiSurfaceCloseStep::Pending;
                    }
                    if !frame.candidate.retire_step() || !frame.candidate.retirement_is_empty() {
                        return UiSurfaceCloseStep::Pending;
                    }
                    window.paint_frame = None;
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::Draw
            }
            UiSurfaceClosePhase::Draw => {
                if !window.draw.retire_step() || !window.draw.retirement_is_empty() {
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::Hits
            }
            UiSurfaceClosePhase::Hits => {
                if window.hit_registry.pop().is_some() {
                    return UiSurfaceCloseStep::Pending;
                }
                if window.hit_registry.capacity() > 0 {
                    window.hit_registry = Vec::new();
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::Document
            }
            UiSurfaceClosePhase::Document => {
                let Some(window_id) = self.windows.id(token).cloned() else { return UiSurfaceCloseStep::Complete };
                if !self.close_document_step(window_id.as_ref()) {
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::PresentedDocument
            }
            UiSurfaceClosePhase::PresentedDocument => {
                if self.windows.get_token(token).is_some_and(|window|window.presented_source.is_some()){return UiSurfaceCloseStep::Pending;}
                let Some(window_id) = self.windows.id(token).cloned() else { return UiSurfaceCloseStep::Complete };
                let Some(window) = self.windows.get_token_mut(token) else { return UiSurfaceCloseStep::Complete };
                let generation = window.presented_accessibility_generation;
                if !window.presented_tree.close_document_binding_step(&mut |document_id, node_id, node| retire_surface_scene(document_id, window_id.as_ref(), generation, &mut window.scene_retirements, node_id, node)) {
                    return UiSurfaceCloseStep::Pending;
                }
                if let Some(document) = window.retiring_presented_document.as_mut() {
                    if !document.close_step() {
                        return UiSurfaceCloseStep::Pending;
                    }
                    window.retiring_presented_document = None;
                    return UiSurfaceCloseStep::Pending;
                }
                if let Some(document) = window.presented_tree.take_document() {
                    window.retiring_presented_document = Some(document);
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::Storage
            }
            UiSurfaceClosePhase::Storage => {
                let Some(window_id) = self.windows.id(token).cloned() else { return UiSurfaceCloseStep::Complete };
                let Some(window) = self.windows.get_token_mut(token) else { return UiSurfaceCloseStep::Complete };
                if !window.close_tree_storage_step(window_id.as_ref()) {
                    return UiSurfaceCloseStep::Pending;
                }
                if window.scene_retirements.capacity() > 0 {
                    window.scene_retirements = VecDeque::new();
                    return UiSurfaceCloseStep::Pending;
                }
                if window.candidate_scene_retirements.capacity() > 0 || window.accepted_scene_retirements.capacity() > 0 {
                    window.candidate_scene_retirements = VecDeque::new();
                    window.accepted_scene_retirements = VecDeque::new();
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::PresentedStorage
            }
            UiSurfaceClosePhase::PresentedStorage => {
                let Some(window_id) = self.windows.id(token).cloned() else { return UiSurfaceCloseStep::Complete };
                let Some(window) = self.windows.get_token_mut(token) else { return UiSurfaceCloseStep::Complete };
                if !window.close_presented_tree_storage_step(window_id.as_ref()) {
                    return UiSurfaceCloseStep::Pending;
                }
                UiSurfaceClosePhase::Scheduler
            }
            UiSurfaceClosePhase::Scheduler => {
                if self.layout_pressure.is_some_and(|entry| entry.token == token) {
                    self.layout_pressure = None;
                    return UiSurfaceCloseStep::Pending;
                }
                if self.layout_queues.iter_mut().any(|lane| lane.remove_token_one(token)) {
                    return UiSurfaceCloseStep::Pending;
                }
                window.queued = false;
                UiSurfaceClosePhase::Complete
            }
            UiSurfaceClosePhase::Complete => {
                return if self.windows.remove_terminal(token) { UiSurfaceCloseStep::Complete } else { UiSurfaceCloseStep::Pending };
            }
        };
        if let Some(window) = self.windows.get_token_mut(token) {
            window.closing = Some(next);
        }
        UiSurfaceCloseStep::Pending
        })())
    }

}
//#endregion 🔖️Ui
// #endregion engine
