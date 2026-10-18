//! 🌳️ Ui tree reconcile: the `UiTree` document reconcile that mounts, updates and retires nodes from a record table. See [`super`].

use super::*;

// #region reconcile
//#region 🌳️DocumentTreeReconcile
impl UiTree {
    /// 🌳️ Advances this surface's document→arena reconcile by exactly one unit. See
    /// [`UiDocumentReconcileCursor`] for the phase ladder and the identity rule.
    ///
    /// 📐️ The record's own `LayoutSpec` is the React-parity geometry channel
    /// (`layoutSpecStyle`'s input); a change to it alone — a grid gaining a
    /// column, a stack flipping to `justify: SpaceBetween` — moves every
    /// descendant's box without touching the projected `UiNode` at all, so it
    /// is re-stamped unconditionally. `Publish` below already re-dirties the
    /// whole surface's layout, so no per-node bubble is needed here.
    ///
    /// 🎬️ Always re-stamped, never diffed: the address carries the document's
    /// CURRENT revision, which moves on every published patch even when the
    /// node's own spec did not — and a node left at an old revision would
    /// start dropping its own live gestures as stale.
    ///
    /// 🪟️ Only ever RAISED here: `events::EventRouter::open_overlay`/`close_overlay`
    /// own this bit at runtime for popups the document knows nothing about, so a
    /// re-mount must not drop an open Select's hit-test priority.
    pub fn step_document_reconcile(&mut self, cursor: &mut UiDocumentReconcileCursor, surface: &str, controller: &str) -> UiDocumentReconcileStep {
        let mut component_mount_generation = self.document_bindings().iter().filter_map(|(_, node)| self.node(*node).map(Node::component_generation)).max().unwrap_or(0);
        self.step_document_reconcile_preserving(cursor, surface, controller, None, 0, None, true, &mut component_mount_generation, &mut |_| true)
    }

    /// 🔒️ Reconciles while preserving the synthesized rows owned by the Select whose option has
    /// pointer capture. The owner record itself keeps its arena identity, and its rows are relinked
    /// after the document links so the matching release still reaches the exact pressed row.
    pub fn step_document_reconcile_preserving(
        &mut self,
        cursor: &mut UiDocumentReconcileCursor,
        surface: &str,
        controller: &str,
        preserved_composite_owner: Option<NodeId>,
        window_generation: u64,
        presented: Option<&UiTree>,
        preserve_candidate_scene_identity: bool,
        component_mount_generation: &mut u64,
        retire_scene: &mut impl FnMut(UiRetiredComponentScene) -> bool,
    ) -> UiDocumentReconcileStep {
        match cursor.phase {
            UiDocumentReconcilePhase::Complete => return UiDocumentReconcileStep::Complete,
            UiDocumentReconcilePhase::Fault => return UiDocumentReconcileStep::Fault(cursor.fault.unwrap_or(UiDocumentReconcileFault::MissingRecord)),
            _ => {}
        }
        let Some(root_id) = self.document().map(UiDocumentTree::root_id) else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
        match cursor.phase {
            UiDocumentReconcilePhase::Compose => {
                if !self.retire_composite_row_except_step(preserved_composite_owner) {
                    return UiDocumentReconcileStep::Pending;
                }
                cursor.phase = UiDocumentReconcilePhase::Adopt;
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Adopt => {
                // 🧹️ A root this ledger never minted can only come from the testkit `apply_tree`
                // path; it is removed whole so the document owns the arena outright.
                let adopted = self.root.is_some_and(|root| self.document_bindings().iter().any(|(_, node)| *node == root));
                if let Some(root) = self.root.filter(|_| !adopted) {
                    self.remove(root);
                }
                self.root = None;
                cursor.stack.push((root_id, None));
                cursor.phase = UiDocumentReconcilePhase::Plan;
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Plan => {
                let Some((id, parent)) = cursor.stack.pop() else {
                    cursor.sweep = 0;
                    cursor.phase = UiDocumentReconcilePhase::Unlink;
                    return UiDocumentReconcileStep::Pending;
                };
                if cursor.plan.len() >= UI_DOCUMENT_NODES || cursor.stack.len() > UI_DOCUMENT_NODES {
                    return cursor.refuse(UiDocumentReconcileFault::Depth);
                }
                let children: Vec<UiNodeId> = {
                    let Some(document) = self.document() else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
                    let Some(record) = document.record(id) else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
                    if record_consumes_subtree(record) {
                        Vec::new()
                    } else {
                        record.children.iter().rev().copied().collect()
                    }
                };
                let index = cursor.plan.len();
                cursor.plan.push(PlannedNode { id, parent, node: None });
                cursor.remember_planned(id);
                for child in children {
                    cursor.stack.push((child, Some(index)));
                }
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Unlink => {
                let Some((_, node)) = self.document_bindings().get(cursor.sweep).copied() else {
                    cursor.mount = 0;
                    cursor.phase = UiDocumentReconcilePhase::Mount;
                    return UiDocumentReconcileStep::Pending;
                };
                self.clear_links(node);
                cursor.sweep += 1;
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Mount => {
                let Some(planned) = cursor.plan.get(cursor.mount).copied() else {
                    cursor.sweep = 0;
                    cursor.phase = UiDocumentReconcilePhase::Retire;
                    return UiDocumentReconcileStep::Pending;
                };
                let (key, mut spec, layout_spec, intent) = {
                    let Some(document) = self.document() else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
                    let Some(record) = document.record(planned.id) else { return cursor.refuse(UiDocumentReconcileFault::MissingRecord) };
                    (NodeKey::Explicit(record.key.as_str().to_string()), WidgetSpec(ui_node_from_record(document, record, surface, controller)), record.layout.clone(), record_intent_bindings(record, surface, document.revision().0))
                };
                let routing = layout_routing_flags(&layout_spec);
                let presented_component = presented.and_then(|tree| {
                    let node = tree.document_node(planned.id)?;
                    let retained = tree.node(node)?;
                    let UiNode::ComponentScene(scene) = &retained.spec.0 else { return None };
                    Some((node, scene.host_id.clone(), retained.component_generation(), retained.key.clone(), scene.component_kind, scene.surface_id.clone()))
                });
                let presented_scene = match (&spec.0, presented_component.as_ref()) {
                    (UiNode::ComponentScene(next), Some((_, host_id, generation, presented_key, kind, surface_id))) if presented_key == &key && *kind == next.component_kind && surface_id == &next.surface_id => Some((host_id.clone(), *generation)),
                    _ => None,
                };
                let node = match self.document_node(planned.id).filter(|node| self.contains(*node)) {
                    Some(node) => {
                        let Some((old_scene, candidate_scene)) = self.node(node).map(|existing| {
                            let old_scene = match &existing.spec.0 {
                                UiNode::ComponentScene(scene) => Some(scene),
                                _ => None,
                            };
                            let same_logical_scene = matches!(
                                (old_scene, &spec.0),
                                (Some(old_scene), UiNode::ComponentScene(next_scene))
                                    if existing.key == key && old_scene.component_kind == next_scene.component_kind && old_scene.surface_id == next_scene.surface_id
                            );
                            let old_scene = old_scene.map(|old_scene| (old_scene.host_id.clone(), existing.component_generation(), existing.key.clone(), old_scene.component_kind, old_scene.surface_id.clone()));
                            let candidate_scene = if preserve_candidate_scene_identity && same_logical_scene { old_scene.as_ref().map(|(host_id, generation, ..)| (host_id.clone(), *generation)) } else { None };
                            (old_scene, candidate_scene)
                        }) else {
                            return cursor.refuse(UiDocumentReconcileFault::Detached);
                        };
                        let preserved_scene_identity = presented_scene.or(candidate_scene);
                        let retirement = old_scene.filter(|(host_id, _, _, _, _)| {
                            let owned = preserve_candidate_scene_identity || presented.is_some_and(|tree| tree.component_scene_host_is_mounted(host_id));
                            let survives = preserved_scene_identity.as_ref().is_some_and(|(preserved_host, _)| preserved_host == host_id);
                            owned && !survives
                        });
                        if let Some((host_id, generation, key, kind, surface_id)) = retirement {
                            if !retire_scene(UiRetiredComponentScene { document_id: planned.id, host_id, window_id: surface.to_owned(), window_generation, component_generation: generation, node, key, kind, surface_id }) {
                                return UiDocumentReconcileStep::Pending;
                            }
                        }
                        let scene_identity = if matches!(&spec.0, UiNode::ComponentScene(_)) {
                            match preserved_scene_identity {
                                Some(identity) => Some(identity),
                                None => {
                                    let Some(generation) = component_mount_generation.checked_add(1) else { return cursor.refuse(UiDocumentReconcileFault::ComponentGeneration) };
                                    *component_mount_generation = generation;
                                    Some((component_scene_host_id(window_generation, generation), generation))
                                }
                            }
                        } else {
                            None
                        };
                        if let UiNode::ComponentScene(scene) = &mut spec.0 {
                            scene.host_id = scene_identity.as_ref().expect("scene mount identity").0.clone();
                        }
                        if let Some(existing) = self.node_mut(node) {
                            if let Some((_, generation)) = scene_identity {
                                existing.component_generation = generation;
                            }
                            if existing.key != key {
                                existing.key = key;
                            }
                            if existing.spec != spec {
                                existing.spec = spec;
                            }
                            existing.layout_spec = Some(layout_spec);
                            existing.intent = Some(intent);
                        }
                        node
                    }
                    None => {
                        if presented_scene.is_none() {
                            if let Some((node, host_id, generation, key, kind, surface_id)) = presented_component {
                                if !retire_scene(UiRetiredComponentScene { document_id: planned.id, host_id, window_id: surface.to_owned(), window_generation, component_generation: generation, node, key, kind, surface_id }) {
                                    return UiDocumentReconcileStep::Pending;
                                }
                            }
                        }
                        let scene_identity = if matches!(&spec.0, UiNode::ComponentScene(_)) {
                            match presented_scene {
                                Some(identity) => Some(identity),
                                None => {
                                    let Some(generation) = component_mount_generation.checked_add(1) else { return cursor.refuse(UiDocumentReconcileFault::ComponentGeneration) };
                                    *component_mount_generation = generation;
                                    Some((component_scene_host_id(window_generation, generation), generation))
                                }
                            }
                        } else {
                            None
                        };
                        if let (UiNode::ComponentScene(scene), Some((host_id, _))) = (&mut spec.0, scene_identity.as_ref()) {
                            scene.host_id = host_id.clone();
                        }
                        let mut mounted = Node::new(key, spec);
                        if let Some((_, generation)) = scene_identity {
                            mounted.component_generation = generation;
                        }
                        mounted.layout_spec = Some(layout_spec);
                        mounted.intent = Some(intent);
                        let node = self.insert_detached(mounted);
                        self.bind_document_node(planned.id, node);
                        node
                    }
                };
                if let Some(mounted) = self.node_mut(node) {
                    mounted.flags.set(NodeFlags::SCROLLABLE, routing.scrollable);
                    mounted.flags.set(NodeFlags::CLIPS_CHILDREN, routing.clips);
                    if routing.overlay {
                        mounted.flags.set(NodeFlags::OVERLAY, true);
                    }
                }
                if let Some(entry) = cursor.plan.get_mut(cursor.mount) {
                    entry.node = Some(node);
                }
                match planned.parent {
                    None => self.root = Some(node),
                    Some(parent) => {
                        let Some(parent_node) = cursor.plan.get(parent).and_then(|entry| entry.node) else { return cursor.refuse(UiDocumentReconcileFault::Detached) };
                        self.attach_child(parent_node, node);
                    }
                }
                if Some(node) == preserved_composite_owner {
                    self.reattach_composite_rows(node);
                }
                cursor.mount += 1;
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Retire => {
                let Some((id, node)) = self.document_bindings().get(cursor.sweep).copied() else {
                    cursor.phase = UiDocumentReconcilePhase::Publish;
                    return UiDocumentReconcileStep::Pending;
                };
                if cursor.planned_ids.binary_search(&id).is_ok() {
                    cursor.sweep += 1;
                    return UiDocumentReconcileStep::Pending;
                }
                let retirement = self.node(node).and_then(|existing| {
                    let UiNode::ComponentScene(scene) = &existing.spec.0 else { return None };
                    Some(UiRetiredComponentScene {
                        document_id: id,
                        host_id: scene.host_id.clone(),
                        window_id: surface.to_owned(),
                        window_generation,
                        component_generation: existing.component_generation(),
                        node,
                        key: existing.key.clone(),
                        kind: scene.component_kind,
                        surface_id: scene.surface_id.clone(),
                    })
                });
                if retirement.is_some_and(|retirement| !retire_scene(retirement)) {
                    return UiDocumentReconcileStep::Pending;
                }
                self.unbind_document_node_at(cursor.sweep);
                self.remove_detached(node);
                UiDocumentReconcileStep::Pending
            }
            UiDocumentReconcilePhase::Publish => {
                if let Some(root) = self.root {
                    self.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
                }
                cursor.phase = UiDocumentReconcilePhase::Complete;
                UiDocumentReconcileStep::Complete
            }
            UiDocumentReconcilePhase::Complete => UiDocumentReconcileStep::Complete,
            UiDocumentReconcilePhase::Fault => UiDocumentReconcileStep::Fault(cursor.fault.unwrap_or(UiDocumentReconcileFault::MissingRecord)),
        }
    }
}
//#endregion 🌳️DocumentTreeReconcile
// #endregion reconcile
