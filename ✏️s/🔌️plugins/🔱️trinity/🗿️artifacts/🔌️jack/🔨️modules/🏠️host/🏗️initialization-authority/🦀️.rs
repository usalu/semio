//! 🏗️ Jack store initialization authority: the phase machine and the authority that rebuilds the document store.

use super::*;

//#region 🔖️RetainedStoreInitialization
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JackStoreInitializationPhase {
    BindGenesis,
    ValidateEnvelope,
    ValidateEdit { index: usize },
    CloneInitial,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    BuildOwners,
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

pub struct JackStoreInitializationAuthority {
    pub actor: std::mem::ManuallyDrop<Option<protocol::ActorId>>,
    actor_close: std::mem::ManuallyDrop<Option<ControlledRetirement<protocol::ActorId>>>,
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub envelope: std::mem::ManuallyDrop<Option<store::os_store::store_schema::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>>>,
    pub owners: std::mem::ManuallyDrop<Option<store::os_store::document_store_owners::DocumentStoreOwners<JackSnapshot, TrinityGraphMutation>>>,
    pub runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<JackSnapshot>>>,
    pub candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<JackSnapshot, TrinityGraphMutation>>>,
    pub displaced: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub clone: std::mem::ManuallyDrop<Option<JackSnapshotCloneAuthority>>,
    pub edit_index: store::ArtifactStoreInitializationEditIndex,
    pub phase: JackStoreInitializationPhase,
    pub resume_phase: Option<JackStoreInitializationPhase>,
    pub cancel_requested: bool,
    pub fault: std::mem::ManuallyDrop<Option<semio_framework_diagnostic::Fault>>,
    fault_close: std::mem::ManuallyDrop<Option<ControlledRetirement<semio_framework_diagnostic::Fault>>>,
    pub fault_publication: semio_framework_job_fault_publication::RetainedFaultPublication,
    pub terminal_handoff: bool,
}

impl JackStoreInitializationAuthority {
    pub fn new(envelope: store::os_store::store_schema::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        Self {
            actor: std::mem::ManuallyDrop::new(Some(actor)),
            actor_close: std::mem::ManuallyDrop::new(None),
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            owners: std::mem::ManuallyDrop::new(None),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            displaced: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(None),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: JackStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: std::mem::ManuallyDrop::new(None),
            fault_close: std::mem::ManuallyDrop::new(None),
            fault_publication: semio_framework_job_fault_publication::RetainedFaultPublication::new(),
            terminal_handoff: false,
        }
    }

    pub fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    pub fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    pub fn fail(&mut self, code: &[u8]) {
        if self.fault.is_none() {
            *self.fault = Some(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::App, "trinity.jack.initializer", String::from_utf8_lossy(code).into_owned()));
        }
        self.phase = JackStoreInitializationPhase::RetireFault;
    }

    pub fn fail_with(&mut self, error: ValueError) {
        if self.fault.is_none() {
            let progress = error.retained_progress();
            *self.fault = Some(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, error.kind.as_str(), error.into_message()).with_retained_progress(progress));
        }
        self.phase = JackStoreInitializationPhase::RetireFault;
    }

    pub fn snapshot_factory() -> std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<JackSnapshot>> {
        std::sync::Arc::new(OwnedValueRetirementFactory::<JackSnapshot>::default())
    }

    /// 🧹️ Drains one displaced projection or settles one replaced workspace owner under the step's own retained wallet; answers whether another turn is owed first.
    pub fn settle_displaced(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        let grant = cx.retained_grant();
        if self.active.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant)?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        if self.displaced.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.displaced, &mut self.active, grant)?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let step = runtime.settle_current_retirement_step(grant)?;
            cx.consume_retained(step.progress())?;
            return Ok(!matches!(step, RetainedCloneStep::Complete(_)));
        }
        Ok(false)
    }

    fn fault_is_empty(&self) -> bool {
        self.fault.is_none() && self.fault_close.is_none() && self.fault_publication.terminal_is_empty()
    }

    fn nothing_retained(&self) -> bool {
        self.envelope.is_none()
            && self.owners.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.displaced.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.clone.is_none()
            && self.actor.is_none()
            && self.actor_close.is_none()
    }

    /// 📏️ Quotes the next original-ownership close turn in the fixed order: displaced workspace, runtime, clone, envelope cursor, envelope with its catalog, bare catalog, actor.
    pub fn close_demands(&self, body: usize, include_fault: bool) -> Result<RetirementDemand, ValueError> {
        if include_fault {
            if !self.fault_publication.terminal_is_empty() {
                return self.fault_publication.retirement_demands();
            }
            if self.fault.is_some() {
                return Ok(RetirementDemand { copy_bytes: size_of::<Option<semio_framework_diagnostic::Fault>>() + size_of::<Option<ControlledRetirement<semio_framework_diagnostic::Fault>>>(), depth: 1, ..Default::default() });
            }
            if let Some(owner) = self.fault_close.as_ref() {
                return if owner.terminal_is_empty() {
                    Ok(RetirementDemand { copy_bytes: size_of::<Option<ControlledRetirement<semio_framework_diagnostic::Fault>>>(), depth: 1, ..Default::default() })
                } else {
                    nested(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? })
                };
            }
        }
        if let Some(owner) = self.active.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if self.displaced.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.displaced);
        }
        if let Some(runtime) = self.runtime.as_ref() {
            if runtime.terminal_is_empty() {
                return Ok(RetirementDemand { copy_bytes: size_of::<Option<store::ArtifactStoreInitializationRuntime<JackSnapshot>>>(), depth: 1, ..Default::default() });
            }
            return runtime.initialization_retirement_demands(body);
        }
        if let Some(clone) = self.clone.as_ref() {
            if clone.terminal_is_empty() {
                return Ok(RetirementDemand { copy_bytes: size_of::<Option<JackSnapshotCloneAuthority>>(), depth: 1, ..Default::default() });
            }
            return clone.close_demands(body);
        }
        if let Some(owner) = self.envelope_retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if self.envelope.is_some() {
            let Some(owners) = self.owners.as_ref() else {
                return Ok(RetirementDemand { copy_bytes: size_of::<Option<store::os_store::document_store_owners::DocumentStoreOwners<JackSnapshot, TrinityGraphMutation>>>(), capacity_bytes: store::os_store::document_store_owners::bounded_artifact_store_owners_birth_bytes::<JackSnapshot, TrinityGraphMutation>(), depth: 1, ..Default::default() });
            };
            if !owners.constructor_is_complete() {
                return nested(owners.constructor_demands(body)?);
            }
            let mut demand = nested(owners.uninstalled_envelope_retirement_demands(self.envelope.as_ref().expect("retained envelope")))?;
            demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "jack initializer envelope frame copy overflow"))?;
            return Ok(demand);
        }
        if let Some(owners) = self.owners.as_ref() {
            return if owners.uninstalled_owners_terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: size_of::<Option<store::os_store::document_store_owners::DocumentStoreOwners<JackSnapshot, TrinityGraphMutation>>>(), depth: 1, ..Default::default() })
            } else {
                nested(owners.uninstalled_owners_demands(body)?)
            };
        }
        if let Some(actor) = self.actor_close.as_ref() {
            return if actor.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: size_of::<Option<ControlledRetirement<protocol::ActorId>>>(), depth: 1, ..Default::default() })
            } else {
                nested(RetirementDemand { copy_bytes: actor.next_copy_byte_demand()?, capacity_bytes: actor.next_capacity_byte_demand(body)?, release_bytes: actor.next_release_byte_demand()?, depth: actor.next_depth_demand()? })
            };
        }
        if self.actor.is_some() {
            return Ok(RetirementDemand { copy_bytes: size_of::<Option<protocol::ActorId>>() + size_of::<Option<ControlledRetirement<protocol::ActorId>>>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    /// ♻️ Closes the initializer's original ownership one funded turn at a time. The fault (with its publication) stays retained until the job's own close asks for it, so a fault outcome can still be lent; terminal emptiness is published only after the last owner is gone.
    pub fn close_original(&mut self, grant: RetainedCloneGrant, include_fault: bool) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.nothing_retained() && (!include_fault || self.fault_is_empty()) {
            self.terminal_handoff |= include_fault;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes, include_fault)?;
        if !admit_demand(grant, demand, "jack initializer close exceeds admitted depth")? {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if include_fault {
            if !self.fault_publication.terminal_is_empty() {
                return self.fault_publication.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            if let Some(fault) = self.fault.take() {
                return match ControlledRetirement::new(fault) {
                    Ok(owner) => {
                        *self.fault_close = Some(owner);
                        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
                    }
                    Err((error, fault)) => {
                        *self.fault = Some(fault);
                        Err(error)
                    }
                };
            }
            if let Some(owner) = self.fault_close.as_mut() {
                if owner.terminal_is_empty() {
                    drop(self.fault_close.take());
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
                }
                let step = owner.step(child)?;
                let step = admit_retained_clone_close(child, step, owner.terminal_is_empty(), "jack initializer fault")?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
        }
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.displaced.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.displaced, &mut self.active, grant);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            if runtime.terminal_is_empty() {
                drop(self.runtime.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = runtime.close_step(&Self::snapshot_factory(), grant)?;
            let step = admit_retained_clone_close(grant, step, runtime.terminal_is_empty(), "jack initialization runtime")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(clone) = self.clone.as_mut() {
            if clone.terminal_is_empty() {
                drop(self.clone.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = clone.close_step(grant)?;
            let step = admit_retained_clone_close(grant, step, clone.terminal_is_empty(), "jack initialization clone")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope_retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.envelope_retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            if self.owners.is_none() {
                let placement = demand.copy_bytes;
                let owners_grant = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - placement, ..grant };
                return match store::os_store::document_store_owners::bounded_artifact_store_owners::<JackSnapshot, TrinityGraphMutation>(owners_grant) {
                    Ok((owners, mut receipt)) => {
                        *self.owners = Some(owners);
                        receipt.copied_bytes += placement;
                        Ok(RetainedCloneStep::Progress(receipt))
                    }
                    Err(refused) => {
                        *self.owners = refused.owners;
                        Err(refused.error.with_retained_progress(refused.progress))
                    }
                };
            }
            let owners = self.owners.as_mut().expect("retained catalog");
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map(RetainedCloneStep::Progress).map_err(|(error, receipt)| error.with_retained_progress(receipt));
            }
            let placement = size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>();
            let envelope_grant = RetainedCloneGrant { maximum_copy_bytes: child.maximum_copy_bytes - placement, ..child };
            let owners = self.owners.take().expect("retained catalog");
            let envelope = self.envelope.take().expect("retained envelope");
            return match owners.retire_envelope_uninstalled(envelope, envelope_grant) {
                Ok((owner, mut receipt)) => {
                    *self.envelope_retirement = Some(owner);
                    receipt.copied_bytes += placement;
                    Ok(RetainedCloneStep::Progress(receipt))
                }
                Err((error, owners, envelope)) => {
                    *self.owners = Some(owners);
                    *self.envelope = Some(envelope);
                    Err(error)
                }
            };
        }
        if let Some(owners) = self.owners.as_mut() {
            if owners.uninstalled_owners_terminal_is_empty() {
                drop(self.owners.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = owners.close_uninstalled_owners_step(child)?;
            let step = admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "jack initializer catalog")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(actor) = self.actor_close.as_mut() {
            if actor.terminal_is_empty() {
                drop(self.actor_close.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = actor.step(child)?;
            let step = admit_retained_clone_close(child, step, actor.terminal_is_empty(), "jack initializer actor")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(actor) = self.actor.take() {
            return match ControlledRetirement::new(actor) {
                Ok(owner) => {
                    *self.actor_close = Some(owner);
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
                }
                Err((error, actor)) => {
                    *self.actor = Some(actor);
                    Err(error)
                }
            };
        }
        Ok(RetainedCloneStep::Progress(empty))
    }

    /// 🧹️ One terminal close turn under the step's retained wallet; answers whether the initializer is empty.
    pub fn close_for_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        let step = self.close_original(cx.retained_grant(), false)?;
        cx.consume_retained(step.progress())?;
        Ok(matches!(step, RetainedCloneStep::Complete(_)))
    }

    pub fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff && self.nothing_retained() && self.fault_is_empty()
    }
}
//#endregion 🔖️RetainedStoreInitialization
