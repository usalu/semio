//! 🚶️ Cursor step: the single resumable `SurfaceReconcileCursor::step` that advances the reconcile one admitted unit at a time. See [`super`].

use super::*;

//#region ⏭️ResumableReconcile
impl SurfaceReconcileCursor {
    pub fn step(&mut self, current: &SurfaceShadow) -> SurfaceReconcileStep {
        if let Some(fault) = self.fault.clone() {
            return SurfaceReconcileStep::Fault(fault);
        }
        if current.revision != self.base_revision {
            let fault = SurfaceReconcileFault::StaleGeneration { expected: self.base_revision.0, actual: current.revision.0 };
            self.fault = Some(fault.clone());
            return SurfaceReconcileStep::Fault(fault);
        }
        if self.pending_op.get().is_some() {
            let returning_component = self.record_diff.as_ref().is_some_and(|diff| diff.fresh.is_none() && diff.field == 0 && diff.owned_copy.as_ref().and_then(RecordOwnedCopy::component).is_some_and(ui_contract::UiComponentCopy::terminal_is_empty));
            let step = if returning_component { self.advance_existing_component(current) } else { self.advance_pending_patch() };
            #[cfg(any(test, feature = "testkit"))]
            if self.fail_component_step { panic!("injected retained component output callback unwind"); }
            return step;
        }
        let step = match self.stage {
            SurfaceReconcileStage::TraversePresentation => {
                if self.held_node.is_none() {
                    if let Some(root) = self.pending_root.take() {
                        self.held_node = Some((None, root));
                        self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
                        self.semantic_usage = SurfaceSemanticUsage::default();
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 };
                    }
                    if let Some(frame) = self.traversal.last_mut() {
                        if let Some(child) = frame.children.next() {
                            self.held_node = Some((Some(frame.index), child));
                            self.semantic_census = Some(SurfaceSemanticCensusCursor::default());
                            self.semantic_usage = SurfaceSemanticUsage::default();
                            return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<usize>() };
                        }
                        let Some(complete) = self.traversal.pop() else {
                            self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                            return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                        };
                        if self.postorder.try_push(complete.index).is_err() {
                            self.overflow_frame = Some(complete);
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<usize>() };
                    }
                    self.stage = SurfaceReconcileStage::AllocateIdentities;
                    return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 };
                }
                if let Some((_, node)) = self.held_node.as_ref() {
                    let key_bytes = node.key.len();
                    if key_bytes > self.limits.max_identifier_bytes {
                        let fault = SurfaceReconcileFault::IdentifierBytes { actual: key_bytes, max: self.limits.max_identifier_bytes };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if self.flat.len() >= self.limits.max_nodes {
                        let usage = SurfaceReconcileUsage { nodes: self.flat.len().saturating_add(1), items: self.usage.items, bytes: self.usage.bytes };
                        let fault = SurfaceReconcileFault::Credits { usage, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    let Some(semantic_census) = self.semantic_census.as_mut() else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let semantic = semantic_census.step(node);
                    match semantic {
                        SurfaceSemanticCensusStep::Progress(delta) => {
                            let Some(semantic_items) = self.semantic_usage.items.checked_add(delta.items) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            let Some(semantic_bytes) = self.semantic_usage.bytes.checked_add(delta.bytes) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            self.semantic_usage.items = semantic_items;
                            self.semantic_usage.bytes = semantic_bytes;
                            let Some(node_page_bytes) = size_of::<FlatPresentedNode>().checked_add(self.semantic_usage.bytes) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            let Some(projected_nodes) = self.usage.nodes.checked_add(1) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            let Some(projected_items) = self.usage.items.checked_add(self.semantic_usage.items) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            let Some(projected_bytes) = self.usage.bytes.checked_add(node_page_bytes) else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            let projected = SurfaceReconcileUsage { nodes: projected_nodes, items: projected_items, bytes: projected_bytes };
                            if !projected.fits(self.limits) {
                                let fault = SurfaceReconcileFault::Credits { usage: projected, limits: self.limits };
                                self.fault = Some(fault.clone());
                                return SurfaceReconcileStep::Fault(fault);
                            }
                            return SurfaceReconcileStep::Yield { nodes: 0, bytes: delta.bytes };
                        }
                        SurfaceSemanticCensusStep::Fault(fault) => {
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                        SurfaceSemanticCensusStep::Complete => {}
                    }
                    let Some(node_page_bytes) = size_of::<FlatPresentedNode>().checked_add(self.semantic_usage.bytes) else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let Some(projected_nodes) = self.usage.nodes.checked_add(1) else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let Some(projected_items) = self.usage.items.checked_add(self.semantic_usage.items) else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let Some(projected_bytes) = self.usage.bytes.checked_add(node_page_bytes) else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let projected = SurfaceReconcileUsage { nodes: projected_nodes, items: projected_items, bytes: projected_bytes };
                    if let Err(fault) = self.admit_credit(projected) {
                        return self.fail(fault);
                    }
                    let Some((parent, node)) = self.held_node.take() else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    let mut node = node;
                    self.semantic_census = None;
                    match self.seen.try_insert((parent, node.key.clone())) {
                        Ok(true) => {}
                        Ok(false) => {
                            self.held_node = Some((parent, node));
                            self.fault = Some(SurfaceReconcileFault::DuplicateSiblingKey);
                            return SurfaceReconcileStep::Fault(SurfaceReconcileFault::DuplicateSiblingKey);
                        }
                        Err(_) => {
                            self.held_node = Some((parent, node));
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                    }
                    let children = take(&mut node.children).into_iter();
                    let index = self.flat.len();
                    let flat = FlatPresentedNode { parent, node, child_ids: ui_contract::UiNodeChildren::default() };
                    if let Err(flat) = self.flat.try_push(flat) {
                        self.held_node = Some((parent, flat.node));
                        self.overflow_frame = Some(PresentationFrame { index, children });
                        let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if let Err(frame) = self.traversal.try_push(PresentationFrame { index, children }) {
                        self.overflow_frame = Some(frame);
                        let fault = SurfaceReconcileFault::ValueDepth { actual: SURFACE_RECONCILE_VALUE_DEPTH.saturating_add(1), max: SURFACE_RECONCILE_VALUE_DEPTH };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    self.usage = projected;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                } else {
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::AllocateIdentities => {
                if self.allocate_index < self.flat.len() {
                    let parent_index = self.flat[self.allocate_index].parent;
                    let parent = parent_index.map(|index| self.ids[index]);
                    let key = self.flat[self.allocate_index].node.key.clone();
                    let key_bytes = key.len();
                    let identity = (parent, key);
                    let id = match current.key_index.get(&identity).copied() {
                        Some(id) => id,
                        None => {
                            let Some(id) = self.allocator.try_allocate() else {
                                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                            };
                            id
                        }
                    };
                    if self.new_key_index.try_insert(identity, id).is_err() {
                        let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if self.ids.try_push(id).is_err() {
                        let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
                        self.fault = Some(fault.clone());
                        return SurfaceReconcileStep::Fault(fault);
                    }
                    if let Some(parent) = parent_index {
                        if self.flat[parent].child_ids.try_push(id).is_err() {
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                    }
                    self.allocate_index += 1;
                    let Some(bytes) = size_of::<NodeIdentity>().checked_add(key_bytes) else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    SurfaceReconcileStep::Yield { nodes: 0, bytes }
                } else {
                    self.stage = SurfaceReconcileStage::DiffRecords;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::DiffRecords => {
                if self.record_diff.as_ref().is_some_and(|diff| diff.field >= 8 && diff.fresh.is_none()) { return self.advance_record_assembly(); }
                if self.record_diff.as_ref().is_some_and(|diff| diff.fresh.is_none() && diff.field == 0) {
                    let step = self.advance_existing_component(current);
                    #[cfg(any(test, feature = "testkit"))]
                    if self.fail_component_step { panic!("injected existing component callback unwind"); }
                    return step;
                }
                if self.record_diff.as_ref().is_some_and(|diff| diff.fresh.is_some() && diff.field == 1) {
                    let step = Self::advance_fresh_component(self.record_diff.as_mut().expect("checked retained record"), &mut self.usage, self.limits, &mut self.fault);
                    #[cfg(any(test, feature = "testkit"))]
                    if self.fail_component_step { panic!("injected component callback unwind"); }
                    return step;
                }
                if self.record_diff.as_ref().is_some_and(|diff| diff.fresh.is_some() && diff.field == 5) {
                    let step = Self::advance_fresh_bindings(self.record_diff.as_mut().expect("checked retained record"), &mut self.usage, self.limits, &mut self.fault);
                    #[cfg(any(test, feature = "testkit"))]
                    if self.fail_binding_step { panic!("injected binding callback unwind"); }
                    return step;
                }
                if let Some(mut diff) = self.record_diff.take() {
                    if let Some(fresh) = diff.fresh.as_mut() {
                        let copied = match diff.field {
                            0 => {
                                fresh.key = Some(diff.record.key.clone());
                                true
                            }
                            2 => {
                                fresh.layout = Some(diff.record.layout.clone());
                                true
                            }
                            3 => {
                                fresh.children = Some(diff.record.children.clone());
                                true
                            }
                            4 => {
                                fresh.accessibility = Some(diff.record.accessibility.clone());
                                true
                            }
                            6 => match diff.record.menu.as_ref() {
                                Some(menu) => match menu.credited_clone() {
                                    Some(menu) => {
                                        fresh.menu = Some(Some(menu));
                                        true
                                    }
                                    None => false,
                                },
                                None => {
                                    fresh.menu = Some(None);
                                    true
                                }
                            },
                            _ => {
                                let Some(mut fresh) = diff.fresh.take() else {
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(key) = fresh.key.take() else {
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(component) = fresh.component.take() else {
                                    fresh.key = Some(key);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(layout) = fresh.layout.take() else {
                                    fresh.key = Some(key);
                                    fresh.component = Some(component);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(accessibility) = fresh.accessibility.take() else {
                                    fresh.key = Some(key);
                                    fresh.component = Some(component);
                                    fresh.layout = Some(layout);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(bindings) = fresh.bindings.take() else {
                                    fresh.key = Some(key);
                                    fresh.component = Some(component);
                                    fresh.layout = Some(layout);
                                    fresh.accessibility = Some(accessibility);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(menu) = fresh.menu.take() else {
                                    fresh.key = Some(key);
                                    fresh.component = Some(component);
                                    fresh.layout = Some(layout);
                                    fresh.accessibility = Some(accessibility);
                                    fresh.bindings = Some(bindings);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let Some(children) = fresh.children.take() else {
                                    fresh.key = Some(key);
                                    fresh.component = Some(component);
                                    fresh.layout = Some(layout);
                                    fresh.accessibility = Some(accessibility);
                                    fresh.bindings = Some(bindings);
                                    fresh.menu = Some(menu);
                                    diff.fresh = Some(fresh);
                                    self.record_diff = Some(diff);
                                    self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                                    return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                                };
                                let op = ui_contract::UiPatchOp::Upsert(ui_contract::UiNodeRecord {
                                    id: diff.record.id,
                                    key,
                                    component,
                                    layout,
                                    style: diff.record.style,
                                    activity: diff.record.activity,
                                    disabled: diff.record.disabled,
                                    transition: diff.record.transition,
                                    accessibility,
                                    bindings,
                                    menu,
                                    children,
                                });
                                *self.pending_op.source_mut().expect("active cursor owns writable pending patch") = Some(op);
                                true
                            }
                        };
                        if !copied {
                            self.record_diff = Some(diff);
                            self.fault = Some(SurfaceReconcileFault::AliasCapacity);
                            return SurfaceReconcileStep::Fault(SurfaceReconcileFault::AliasCapacity);
                        }
                        diff.field += 1;
                        self.record_diff = Some(diff);
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: if self.pending_op.get().is_some() { size_of::<ui_contract::UiPatchOp>() } else { 0 } };
                    }
                    if diff.field < 8 {
                        let old = match current.read_record(diff.id) {
                            Ok(Some(old)) => old,
                            Err(ui_contract::UiDocumentLeaseError::Contended) => { self.record_diff = Some(diff); return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
                            _ => {
                            self.record_diff = Some(diff);
                            self.fault = Some(SurfaceReconcileFault::AliasCapacity);
                            return SurfaceReconcileStep::Fault(SurfaceReconcileFault::AliasCapacity);
                            }
                        };
                        let op = match diff_record_field(&old, &diff.record, diff.field) {
                            Ok(op) => op,
                            Err(fault) => {
                                self.record_diff = Some(diff);
                                self.fault = Some(fault.clone());
                                return SurfaceReconcileStep::Fault(fault);
                            }
                        };
                        diff.field += 1;
                        *self.pending_op.source_mut().expect("active cursor owns writable pending patch") = op;
                        self.record_diff = Some(diff);
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: if self.pending_op.get().is_some() { size_of::<ui_contract::UiPatchOp>() } else { 0 } };
                    }
                    self.record_diff = Some(diff);
                    self.advance_record_assembly()
                } else if self.diff_index < self.flat.len() {
                    let index = self.postorder[self.diff_index];
                    let id = self.ids[index];
                    let children = take(&mut self.flat[index].child_ids);
                    let transition = match current.read_record(id) {
                        Ok(record) => record.and_then(|record| record.transition),
                        Err(ui_contract::UiDocumentLeaseError::Contended) => { self.flat[index].child_ids = children; return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }; }
                        Err(_) => { self.flat[index].child_ids = children; return self.fail(SurfaceReconcileFault::AliasCapacity); }
                    };
                    let node = std::mem::replace(&mut self.flat[index].node, TreeNode::empty_separator());
                    let record = build_record_owned(id, node, children, transition);
                    let fresh = (!current.ordinals.contains_key(&id)).then(FreshRecordClone::default);
                    self.record_diff = Some(RecordDiffCursor { id, record: record.into(), field: 0, fresh, owned_copy: None });
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                } else {
                    self.remove_next = self.old_root;
                    self.stage = SurfaceReconcileStage::RemoveStale;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::RemoveStale => {
                if let Some(id) = self.remove_next.take() {
                    if self.new_ordinals.contains_key(&id) {
                        if self.removal.try_push(RemovalFrame { id, next_child: 0 }).is_err() {
                            self.remove_next = Some(id);
                            let fault = SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: self.limits.max_nodes.saturating_add(1), ..self.usage }, limits: self.limits };
                            self.fault = Some(fault.clone());
                            return SurfaceReconcileStep::Fault(fault);
                        }
                    } else {
                        let op = ui_contract::UiPatchOp::Remove { id };
                        *self.pending_op.source_mut().expect("active cursor owns writable pending patch") = Some(op);
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiPatchOp>() };
                    }
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiNodeId>() }
                } else if let Some(frame) = self.removal.last_mut() {
                    let child = match current.read_record(frame.id) {
                        Ok(record) => record.and_then(|record| record.children.get(frame.next_child).copied()),
                        Err(ui_contract::UiDocumentLeaseError::Contended) => return SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 },
                        Err(_) => return self.fail(SurfaceReconcileFault::AliasCapacity),
                    };
                    if let Some(child) = child {
                        frame.next_child += 1;
                        self.remove_next = Some(child);
                    } else {
                        self.removal.pop();
                    }
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiNodeId>() }
                } else {
                    self.stage = SurfaceReconcileStage::Finalize;
                    SurfaceReconcileStep::Yield { nodes: 0, bytes: 0 }
                }
            }
            SurfaceReconcileStage::Finalize => {
                let new_root = self.ids.first().copied();
                if !self.root_patch_prepared && self.old_root != new_root {
                    self.root_patch_prepared = true;
                    if let Some(id) = new_root {
                        *self.pending_op.source_mut().expect("active cursor owns writable pending patch") = Some(ui_contract::UiPatchOp::SetRoot { id });
                        return SurfaceReconcileStep::Yield { nodes: 0, bytes: size_of::<ui_contract::UiPatchOp>() };
                    }
                }
                let revision = if self.ops.is_empty() {
                    self.base_revision
                } else {
                    let Some(revision) = self.base_revision.try_next() else {
                        self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                        return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
                    };
                    revision
                };
                let patch = if self.ops.is_empty() { None } else { Some(ui_contract::UiPatch { surface: self.surface.clone(), base_revision: self.base_revision, revision, ops: take(&mut self.ops) }) };
                let shadow = SurfaceShadow {
                    surface: self.surface.clone(),
                    revision,
                    allocator: self.allocator.clone(),
                    document: None,
                    native_resources: self.native_resources.take(),
                    native_resource_loans: Default::default(),
                    assembly: take(&mut self.assembly),
                    ordinals: self.new_ordinals.take_all(),
                    key_index: self.new_key_index.take_all(),
                    root: new_root,
                    retire_scalar: 0,
                    seal_phase: 0,
                };
                SurfaceReconcileStep::Complete { shadow, patch }
            }
        };
        if let SurfaceReconcileStep::Yield { nodes, bytes } = step {
            if bytes > SURFACE_RECONCILE_PAGE_BYTES {
                let fault = SurfaceReconcileFault::PageBytes { actual: bytes, max: SURFACE_RECONCILE_PAGE_BYTES };
                self.fault = Some(fault.clone());
                return SurfaceReconcileStep::Fault(fault);
            }
            let Some(items) = nodes.checked_mul(8) else {
                self.fault = Some(SurfaceReconcileFault::CounterOverflow);
                return SurfaceReconcileStep::Fault(SurfaceReconcileFault::CounterOverflow);
            };
            if !self.usage.include(nodes, items, bytes) || !self.usage.fits(self.limits) {
                let fault = SurfaceReconcileFault::Credits { usage: self.usage, limits: self.limits };
                self.fault = Some(fault.clone());
                return SurfaceReconcileStep::Fault(fault);
            }
        }
        if let Err(fault) = self.admit_credit(self.usage) {
            return self.fail(fault);
        }
        step
    }

}
//#endregion ⏭️ResumableReconcile
