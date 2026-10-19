//! 📄️ Viewport and document ingress: the viewport and window flow, the presence notes, the tree application and the document begin, page, finish, publish, reconcile and close steps. See [`super`].

use super::*;

// #region engine
//#region 🔖️Ui
impl Ui {
    pub(super) fn window_mut(&mut self, window_id: &str) -> Option<&mut UiWindow> {
        let token = self.try_admit_surface(window_id).ok()?;
        self.windows.get_token_mut(token)
    }

    /// 📐️ Stores the viewport a later `frame` call lays out against for `window_id`, creating that
    /// window's retained state on first use.
    pub fn set_viewport(&mut self, window_id: &str, width: f32, height: f32) {
        let Some(window) = self.window_mut(window_id) else { return };
        if window.viewport == (width, height) {
            return;
        }
        let Some(next_generation) = window.layout_generation.checked_add(1) else { return };
        let Some(next_viewport_revision) = window.viewport_revision.checked_add(1) else { return };
        window.viewport = (width, height);
        window.layout_generation = next_generation;
        window.viewport_revision = next_viewport_revision;
        if let Some(root) = window.tree.root {
            window.tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
        }
        self.enqueue_layout_reason(window_id, SurfaceLayoutReason::Metrics);
    }

    /// 🧭️ Sets `window_id`'s logical flow — React's `FlowProvider` around a `Panel`/`Pane`
    /// (`🖼️Panel/🟦️.tsx:516`), whose value comes from `UiFlow::for_anchor` on that surface's dock
    /// anchor. `Rtl` mirrors every anchored overlay this window places and every inline arrow key it
    /// routes; `Up` is read by paint. Invalidates the layout generation exactly like
    /// [`Self::set_viewport`], because a direction change re-solves the inline axis.
    pub fn set_window_flow(&mut self, window_id: &str, flow: ui_contract::UiFlow) {
        let Some(window) = self.window_mut(window_id) else { return };
        if !window.router.set_flow(flow) {
            return;
        }
        let Some(next_generation) = window.layout_generation.checked_add(1) else { return };
        window.layout_generation = next_generation;
        if let Some(root) = window.tree.root {
            window.tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
        }
        self.enqueue_layout_reason(window_id, SurfaceLayoutReason::Metrics);
    }

    /// 🧭️ `window_id`'s current flow, or React's `DEFAULT_FLOW` for a window that has none yet.
    pub fn window_flow(&self, window_id: &str) -> ui_contract::UiFlow {
        self.windows.get(window_id).map_or(ui_contract::UiFlow::DEFAULT, |window| window.router.flow())
    }

    /// 👥️ Gives every open window's retained trees the peers' notes by record key (sorted by key) — React's host
    /// presence overlay `notes`. Keys are document-unique framework ids, so one table serves every window; a window that
    /// opens later takes it on the next call. `true` when any window changed.
    pub fn set_presence_notes(&mut self, notes: &[(String, String)]) -> bool {
        let mut changed = false;
        for window in self.windows.values_mut().filter(|window| window.closing.is_none()) {
            changed |= window.tree.set_presence_notes(notes);
            changed |= window.presented_tree.set_presence_notes(notes);
        }
        changed
    }

    /// 🎯️ Applies one host-owned single selection to a reconciled Tree candidate without folding
    /// ephemeral presence into the immutable document record. The generation fence prevents a late
    /// host update from decorating a replacement document that reused the same surface id.
    pub fn stamp_tree_selected_item(&mut self, window_id: &str, generation: u64, selected_key: &str) -> bool {
        fn stamp_items(items: &mut [semio_framework_ui_declaration::ui::UiTreeItemNode], selected_key: &str, changed: &mut bool) -> bool {
            let mut matched = false;
            for item in items {
                let selected = item.id == selected_key;
                *changed |= item.presence.selected != selected;
                item.presence.selected = selected;
                matched |= selected;
                if let Some(children) = item.items.as_mut() {
                    matched |= stamp_items(children, selected_key, changed);
                }
            }
            matched
        }

        let Some(window) = self.windows.get_mut(window_id).filter(|window| window.closing.is_none()) else { return false };
        if window.tree.document().is_none_or(|document| document.generation() != generation) {
            return false;
        }
        let Some(root) = window.tree.root else { return false };
        let mut pending = vec![root];
        let mut changed = false;
        let mut matched = false;
        while let Some(id) = pending.pop() {
            pending.extend(window.tree.children(id));
            let is_item = window.tree.authored_tree_item(id).is_some();
            let row_selected = is_item && window.tree.node(id).is_some_and(|node| matches!(&node.key, semio_framework_ui_retained_tree::tree::NodeKey::Explicit(key) if key == selected_key));
            let Some(node) = window.tree.node_mut(id) else { continue };
            if is_item {
                let presence = node.spec.0.presence_mut();
                changed |= presence.selected != row_selected;
                presence.selected = row_selected;
                matched |= row_selected;
            }
            if let UiNode::Tree(tree) = &mut node.spec.0 {
                for section in &mut tree.sections {
                    matched |= stamp_items(&mut section.items, selected_key, &mut changed);
                }
            }
        }
        if changed {
            window.tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
        }
        matched
    }

    /// 🔁️ Runs `UiTree::apply_tree` (`reconcile`) to diff `ui_node` into `window_id`'s retained tree,
    /// creating that window's tree/layout-engine/event-router on first use.
    #[cfg(any(test, feature = "testkit"))]
    pub fn apply_tree(&mut self, window_id: &str, ui_node: &UiNode) {
        let Some(window) = self.window_mut(window_id) else { return };
        let unchanged = window.tree.root.and_then(|root| window.tree.node(root)).is_some_and(|node| node.spec.0 == *ui_node);
        if !unchanged && (window.revision == u64::MAX || window.layout_generation == u64::MAX) {
            return;
        }
        window.tree.apply_tree(ui_node);
        if !unchanged {
            window.revision += 1;
            window.layout_generation += 1;
        }
        let needs_layout = window.tree.root.and_then(|root| window.tree.node(root)).is_some_and(|node| node.flags.contains(NodeFlags::DIRTY_LAYOUT) || node.flags.contains(NodeFlags::SUBTREE_DIRTY));
        if needs_layout {
            self.enqueue_layout(window_id);
        }
    }

    //#region 📄️DocumentIngress
    pub fn document_status(&self, window_id: &str, generation: u64) -> UiDocumentIngressStatus {
        let Some(window) = self.windows.get(window_id) else { return UiDocumentIngressStatus::Vacant };
        if window.tree.document().is_some_and(|document| document.generation() == generation) {
            return UiDocumentIngressStatus::Published;
        }
        if window.candidate_baseline.as_deref().and_then(UiCandidateBaseline::generation) == Some(generation) {
            return UiDocumentIngressStatus::Published;
        }
        window
            .document_ingress
            .as_ref()
            .filter(|ingress| ingress.document.generation() == generation)
            .map_or(UiDocumentIngressStatus::Vacant, |ingress| UiDocumentIngressStatus::Pending { next_page: ingress.next_page, node_count: ingress.node_count })
    }

    #[expect(clippy::result_large_err, reason = "Surface and document admission return the exact refused identity or page without allocating a rejection wrapper.")]
    pub fn begin_document(&mut self, window_id: &str, header: UiDocumentLeaseHeader, cx: &mut StepContext<'_>) -> Result<(), (UiDocumentIngressFault, UiDocumentLeaseHeader)> {
        if cx.is_cancelled() {
            return Err((UiDocumentIngressFault::Cancelled, header));
        }
        if cx.should_yield() {
            return Err((UiDocumentIngressFault::Deadline, header));
        }
        if cx.generation().0 != header.generation {
            return Err((UiDocumentIngressFault::StaleGeneration, header));
        }
        let Some(window) = self.window_mut(window_id) else { return Err((UiDocumentIngressFault::StaleGeneration, header)) };
        if let Some(baseline) = window.candidate_baseline.as_mut() {
            if !baseline.close_step() {
                return Err((UiDocumentIngressFault::InterruptedClose, header));
            }
            window.candidate_baseline = None;
            return Err((UiDocumentIngressFault::InterruptedClose, header));
        }
        if let Some(retiring) = window.retiring_document.as_mut() {
            if !retiring.close_step() {
                return Err((UiDocumentIngressFault::InterruptedClose, header));
            }
            window.retiring_document = None;
            return Err((UiDocumentIngressFault::InterruptedClose, header));
        }
        if let Some(ingress) = window.document_ingress.as_mut() {
            if ingress.document.generation() == header.generation {
                return Ok(());
            }
            if !ingress.document.close_step() {
                return Err((UiDocumentIngressFault::InterruptedClose, header));
            }
            window.document_ingress = None;
            return Err((UiDocumentIngressFault::InterruptedClose, header));
        }
        if window.tree.document().is_some_and(|document| document.generation() >= header.generation) {
            return Err((UiDocumentIngressFault::StaleGeneration, header));
        }
        let node_count = header.node_count;
        let document = UiDocumentTree::new(header.clone()).map_err(|fault| (UiDocumentIngressFault::Invalid(fault), header))?;
        window.document_ingress = Some(UiDocumentIngress { document, next_page: 0, node_count, validation_cursor: 0, validation_started: false, validation_stack: UiFixedList::default(), validation_seen: UiFixedList::default() });
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Surface and document admission return the exact refused identity or page without allocating a rejection wrapper.")]
    pub fn apply_document_page(&mut self, window_id: &str, page: UiDocumentNodePage, cx: &mut StepContext<'_>) -> Result<usize, UiDocumentPageRejection> {
        let Some(window) = self.window_mut(window_id) else {
            return Err(UiDocumentPageRejection { fault: UiDocumentTreeFault::Generation, generation: page.generation(), revision: page.revision(), index: page.index(), record: page.into_record() });
        };
        let Some(ingress) = window.document_ingress.as_mut() else {
            return Err(UiDocumentPageRejection { fault: UiDocumentTreeFault::Generation, generation: page.generation(), revision: page.revision(), index: page.index(), record: page.into_record() });
        };
        if cx.is_cancelled() || cx.should_yield() || cx.generation().0 != ingress.document.generation() {
            return Err(UiDocumentPageRejection { fault: UiDocumentTreeFault::Generation, generation: page.generation(), revision: page.revision(), index: page.index(), record: page.into_record() });
        }
        ingress.document.try_push_page(page, ingress.next_page)?;
        ingress.next_page += 1;
        cx.consume_fuel(1);
        Ok(ingress.next_page)
    }

    pub fn finish_document(&mut self, window_id: &str, generation: u64, cx: &mut StepContext<'_>) -> Result<(), UiDocumentIngressFault> {
        let next_document_generation = self.document_generation.checked_add(1);
        if cx.is_cancelled() {
            return Err(UiDocumentIngressFault::Cancelled);
        }
        if cx.should_yield() {
            return Err(UiDocumentIngressFault::Deadline);
        }
        if cx.generation().0 != generation {
            return Err(UiDocumentIngressFault::StaleGeneration);
        }
        let Some(window) = self.window_mut(window_id) else { return Err(UiDocumentIngressFault::StaleGeneration) };
        let Some(ingress) = window.document_ingress.as_mut() else { return Err(UiDocumentIngressFault::StaleGeneration) };
        if ingress.document.generation() != generation || ingress.next_page != ingress.node_count {
            return Err(UiDocumentIngressFault::StaleGeneration);
        }
        ingress.document.validate_header().map_err(UiDocumentIngressFault::Invalid)?;
        if ingress.validation_cursor < ingress.node_count {
            ingress.document.validate_record(ingress.validation_cursor).map_err(UiDocumentIngressFault::Invalid)?;
            ingress.validation_cursor += 1;
            return Err(UiDocumentIngressFault::ValidationPending);
        }
        if !ingress.validation_started {
            ingress.validation_stack.try_push(ingress.document.root_id()).map_err(|_| UiDocumentIngressFault::Invalid(UiDocumentTreeFault::NodeCapacity))?;
            ingress.validation_started = true;
            return Err(UiDocumentIngressFault::ValidationPending);
        }
        if let Some(id) = ingress.validation_stack.pop() {
            if ingress.validation_seen.iter().any(|visited| *visited == id) {
                return Err(UiDocumentIngressFault::Invalid(UiDocumentTreeFault::Cycle));
            }
            ingress.validation_seen.try_push(id).map_err(|_| UiDocumentIngressFault::Invalid(UiDocumentTreeFault::NodeCapacity))?;
            let record = ingress.document.record(id).ok_or(UiDocumentIngressFault::Invalid(UiDocumentTreeFault::MissingChild))?;
            for child in record.children.iter().rev() {
                ingress.validation_stack.try_push(*child).map_err(|_| UiDocumentIngressFault::Invalid(UiDocumentTreeFault::NodeCapacity))?;
            }
            return Err(UiDocumentIngressFault::ValidationPending);
        }
        if ingress.validation_seen.len() != ingress.node_count {
            return Err(UiDocumentIngressFault::Invalid(UiDocumentTreeFault::Cycle));
        }
        let Some(next_revision) = window.revision.checked_add(1) else { return Err(UiDocumentIngressFault::StaleGeneration) };
        let Some(next_layout_generation) = window.layout_generation.checked_add(1) else { return Err(UiDocumentIngressFault::StaleGeneration) };
        let next_accessibility_generation = if window.accessibility_generation == 0 { Some(next_document_generation.ok_or(UiDocumentIngressFault::StaleGeneration)?) } else { None };
        let Some(ingress) = window.document_ingress.take() else { return Err(UiDocumentIngressFault::StaleGeneration) };
        if let Some(generation) = next_accessibility_generation {
            window.accessibility_generation = generation;
        }
        window.candidate_preserves_prior_mounts = window.presented_ready && window.revision > window.presented_revision;
        window.retiring_document = window.tree.publish_document(ingress.document);
        window.intrinsic_content_height = f32::NAN;
        window.candidate_ready = false;
        window.candidate_reconciled = false;
        window.rearm_candidate_scene_retirements();
        window.candidate_interaction_cursor = None;
        window.revision = next_revision;
        window.layout_generation = next_layout_generation;
        if let Some(root) = window.tree.root {
            window.tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
        }
        if let Some(generation) = next_accessibility_generation {
            self.document_generation = generation;
        }
        self.enqueue_layout(window_id);
        Ok(())
    }

    /// 📄️ Publishes an already-assembled document into `window_id` WITHOUT replaying the paged
    /// ingress — the page ladder is pinned by its own laws (`📃️document-lease-owner-move`), and a
    /// `UiDocumentNodePage` has no public constructor, so a reconcile law would otherwise have to
    /// drive the process-wide `UI_DOCUMENT_ARENA` to say anything about the arena. Bookkeeping is
    /// byte-identical to `finish_document`'s own tail.
    #[cfg(any(test, feature = "testkit"))]
    pub fn publish_document(&mut self, window_id: &str, document: UiDocumentTree) -> bool {
        let next_document_generation = self.document_generation.checked_add(1);
        let Some(window) = self.window_mut(window_id) else { return false };
        if window.candidate_baseline.is_some() {
            return false;
        }
        let Some(next_revision) = window.revision.checked_add(1) else { return false };
        let Some(next_layout_generation) = window.layout_generation.checked_add(1) else { return false };
        let next_accessibility_generation = if window.accessibility_generation == 0 {
            let Some(generation) = next_document_generation else { return false };
            Some(generation)
        } else {
            None
        };
        if let Some(generation) = next_accessibility_generation {
            window.accessibility_generation = generation;
        }
        window.candidate_preserves_prior_mounts = window.presented_ready && window.revision > window.presented_revision;
        window.retiring_document = window.tree.publish_document(document);
        window.intrinsic_content_height = f32::NAN;
        window.candidate_ready = false;
        window.candidate_reconciled = false;
        window.rearm_candidate_scene_retirements();
        window.candidate_interaction_cursor = None;
        window.revision = next_revision;
        window.layout_generation = next_layout_generation;
        if let Some(root) = window.tree.root {
            window.tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
        }
        if let Some(generation) = next_accessibility_generation {
            self.document_generation = generation;
        }
        self.enqueue_layout(window_id);
        true
    }

    /// 🌳️ Advances `window_id`'s published document into its paintable arena, spending this
    /// opportunity's whole budget rather than one unit, and enqueues the layout the freshly mounted
    /// root now needs.
    ///
    /// `UiTree::publish_document` stores the document and nothing read it back — `tree.root` stayed
    /// `None` forever and `frame_into_step` answered `Missing` on its first line
    /// (`📓️wgpu-blank-paint-2026-09-12.md` §5). This is that missing edge: the ONLY production writer
    /// of the arena, `Ui::apply_tree` being `cfg(test/testkit)`.
    pub fn step_document_reconcile(&mut self, window_id: &str, controller: &str, cx: &mut StepContext<'_>) -> UiDocumentReconcileStep {
        let Some(window) = self.window_mut(window_id) else { return UiDocumentReconcileStep::Pending };
        if let Some(baseline) = window.candidate_baseline.as_mut() {
            let step = baseline.step(window.presented_tree.document());
            cx.consume_fuel(1);
            return match step {
                UiCandidateBaselineStep::Pending => UiDocumentReconcileStep::Pending,
                UiCandidateBaselineStep::Complete(document) => {
                    window.candidate_baseline = None;
                    window.candidate_preserves_prior_mounts = false;
                    window.retiring_document = window.tree.publish_document(document);
                    window.intrinsic_content_height = f32::NAN;
                    window.candidate_ready = false;
                    window.candidate_reconciled = false;
                    window.candidate_retirements_resolved = false;
                    window.candidate_interaction_cursor = None;
                    UiDocumentReconcileStep::Pending
                }
                UiCandidateBaselineStep::Fault(fault) => {
                    window.candidate_baseline = None;
                    UiDocumentReconcileStep::Fault(fault)
                }
            };
        }
        if window.candidate_reconciled && !window.candidate_retirements_resolved {
            if let Some(retirement) = window.candidate_scene_retirements.pop_front() {
                if !window.tree.component_scene_host_is_mounted(&retirement.host_id) {
                    window.accepted_scene_retirements.push_back(retirement);
                }
                cx.consume_fuel(1);
                return UiDocumentReconcileStep::Pending;
            }
            window.candidate_retirements_resolved = true;
            window.begin_presented_interaction_rebase();
            cx.consume_fuel(1);
            if window.candidate_ready {
                self.enqueue_layout(window_id);
                return UiDocumentReconcileStep::Complete;
            }
            return UiDocumentReconcileStep::Pending;
        }
        if window.candidate_reconciled && window.candidate_interaction_cursor.is_some() {
            let complete = window.step_presented_interaction_rebase();
            cx.consume_fuel(1);
            if complete {
                self.enqueue_layout(window_id);
                return UiDocumentReconcileStep::Complete;
            }
            return UiDocumentReconcileStep::Pending;
        }
        if window.candidate_ready {
            return UiDocumentReconcileStep::Complete;
        }
        let Some(generation) = window.tree.document().map(UiDocumentTree::generation) else { return UiDocumentReconcileStep::Pending };
        window.document_reconcile.rearm(generation);
        if window.document_reconcile.terminal_is_complete() {
            return UiDocumentReconcileStep::Complete;
        }
        let preserved_composite_owner = window.router.capture().and_then(|(target, _)| window.tree.surviving_composite_owner(target));
        let window_generation = window.accessibility_generation;
        let UiWindow { tree, presented_tree, document_reconcile, scene_retirements, candidate_scene_retirements, accepted_scene_retirements, component_mount_generation, candidate_preserves_prior_mounts, .. } = window;
        let step = loop {
            let step = tree.step_document_reconcile_preserving(
                document_reconcile,
                window_id,
                controller,
                preserved_composite_owner,
                window_generation,
                Some(presented_tree),
                *candidate_preserves_prior_mounts,
                component_mount_generation,
                &mut |retirement| {
                    let deferred = presented_tree.component_scene_host_is_mounted(&retirement.host_id);
                    if scene_retirements.len() + candidate_scene_retirements.len() + accepted_scene_retirements.len() == UI_RETIRED_COMPONENT_SCENE_CAPACITY {
                        return false;
                    }
                    let target = if deferred { &mut *candidate_scene_retirements } else { &mut *scene_retirements };
                    target.push_back(retirement);
                    true
                },
            );
            cx.consume_fuel(1);
            if !scene_retirements.is_empty() || !matches!(step, UiDocumentReconcileStep::Pending) || cx.is_cancelled() || cx.should_yield() {
                break step;
            }
        };
        if matches!(step, UiDocumentReconcileStep::Complete) {
            window.candidate_reconciled = true;
            window.candidate_retirements_resolved = false;
            return UiDocumentReconcileStep::Pending;
        }
        step
    }

    pub fn take_retired_component_scene(&mut self, window_id: &str) -> Option<UiRetiredComponentScene> {
        self.windows.get_mut(window_id)?.scene_retirements.pop_front()
    }

    pub fn retired_component_scene(&self, window_id: &str) -> Option<UiRetiredComponentScene> {
        self.windows.get(window_id)?.scene_retirements.front().cloned()
    }

    pub fn acknowledge_retired_component_scene(&mut self, window_id: &str, expected: &UiRetiredComponentScene) -> bool {
        let Some(window) = self.windows.get_mut(window_id) else { return false };
        if window.scene_retirements.front() != Some(expected) {
            return false;
        }
        window.scene_retirements.pop_front();
        true
    }

    pub fn close_document_step(&mut self, window_id: &str) -> bool {
        let Some(window) = self.windows.get_mut(window_id) else { return true };
        if let Some(baseline) = window.candidate_baseline.as_mut() {
            if !baseline.close_step() {
                return false;
            }
            window.candidate_baseline = None;
            return false;
        }
        let window_generation = window.accessibility_generation;
        let UiWindow { tree, scene_retirements, .. } = window;
        if !tree.close_document_binding_step(&mut |document_id, node_id, node| retire_surface_scene(document_id, window_id, window_generation, scene_retirements, node_id, node)) {
            return false;
        }
        if !window.document_reconcile.close_step() {
            return false;
        }
        if let Some(ingress) = window.document_ingress.as_mut() {
            if !ingress.document.close_step() {
                return false;
            }
            window.document_ingress = None;
            return false;
        }
        if let Some(retiring) = window.retiring_document.as_mut() {
            if !retiring.close_step() {
                return false;
            }
            window.retiring_document = None;
            return false;
        }
        if let Some(document) = window.tree.take_document() {
            window.retiring_document = Some(document);
            return false;
        }
        true
    }
    //#endregion 📄️DocumentIngress

}
//#endregion 🔖️Ui
// #endregion engine
