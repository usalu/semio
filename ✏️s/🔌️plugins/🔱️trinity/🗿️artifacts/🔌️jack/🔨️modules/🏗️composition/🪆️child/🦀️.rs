//! 🪆️ Original Jack commands authorize only the exact published child they read.

use super::{GraphEffect, JackComposition, JackSnapshot};
use semio_framework_job::{InteractiveJobCloseStep, OriginalSourceBinding, OriginalTurnFailure, StepContext};
use semio_framework_value::{
    ErasedSnapshotRetirement, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant,
    RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll, RetireOwned,
    RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::{RetainedCloneRef, retained_clone_grant_admits},
    retirement::controlled::ControlledRetirement,
};
use semio_plugin::{AppOperationContext, app::{
    ChildContentView, ChildEmitPreparation, MemberKeyRef,
    child_read::{
        ChildAdmissionSource, ChildReadCursor, ChildReadReference, ChildReadReferenceAdvance,
        ChildReadReferenceLenderV1, ChildReadSourceCustody, ChildReadStamp, OriginalChildMemberLookup,
        command::{OriginalChildEditorCommandAuthority, OriginalChildEditorCommandCursor},
        erase_child_admission_source,
    },
}};
use std::mem::size_of;

const SLOT: &str = "content";
const PARENT_SOURCE: usize = 0x4a41_434b;

struct JackContentReferenceLender;
static JACK_CONTENT_REFERENCE: JackContentReferenceLender = JackContentReferenceLender;

impl ChildReadReferenceLenderV1 for JackContentReferenceLender {
    fn demand(&self, _: usize) -> RetirementDemand {
        RetirementDemand { copy_bytes: 2 * size_of::<RetainedWorkOutcome<ChildReadReferenceAdvance<'static>>>() + size_of::<usize>(), depth: 1, ..Default::default() }
    }

    fn read<'a>(&self, parent: ChildAdmissionSource<'a>, ordinal: usize, grant: RetainedCloneGrant) -> RetainedWorkOutcome<ChildReadReferenceAdvance<'a>> {
        let demand = self.demand(ordinal);
        if !retained_clone_grant_admits(grant, demand) {
            return RetainedWorkOutcome { result: Ok(ChildReadReferenceAdvance { progress: Default::default(), done: false, reference: None }), work: 0 };
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let result = parent.get().downcast_ref::<JackSnapshot>().ok_or_else(|| ValueError::invariant("Jack child lender requires its original parent snapshot type").with_retained_progress(progress)).map(|snapshot| {
            let handle = &snapshot.content;
            let reference = (ordinal == 0).then_some(ChildReadReference {
                parent_identity: snapshot as *const _ as usize,
                identity: handle as *const _ as usize,
                owner: "",
                slot: SLOT,
                child_id: &handle.child_id,
                artifact_id: &handle.target.artifact_id,
                artifact_kind: &handle.target.dialect.artifact_kind,
                standard: &handle.target.dialect.standard,
                subset: &handle.target.dialect.subset,
            });
            ChildReadReferenceAdvance { progress, done: true, reference }
        });
        RetainedWorkOutcome { result, work: 1 }
    }
}

#[derive(RetireOwned)]
#[value(send_only)]
struct ChildOwners<S: RetireOwned + Sync, M: RetireOwned> {
    authority: Option<OriginalChildEditorCommandAuthority>,
    qualifier: Option<OriginalChildEditorCommandCursor>,
    child: ChildReadSourceCustody<S>,
    read: Option<ChildReadCursor>,
    lookup: Option<OriginalChildMemberLookup>,
    mutations: Option<Vec<M>>,
}

/// 🪪️ Retains the captured member and its registered mutation command through the original grant.
pub struct JackChildQualification<C: JackComposition> {
    owners: ControlledRetirement<ChildOwners<C::ContentSnapshot, C::ContentMutation>>,
    stamp: Option<ChildReadStamp>,
    qualified: bool,
    complete: bool,
    closing: bool,
}

impl<C: JackComposition> JackChildQualification<C> {
    pub fn new() -> Self {
        Self {
            owners: ControlledRetirement::retain(ChildOwners { authority: None, qualifier: None, child: ChildReadSourceCustody::new(), read: None, lookup: None, mutations: None }),
            stamp: None,
            qualified: false,
            complete: false,
            closing: false,
        }
    }

    pub fn step<Command: Sync>(
        &mut self, parent: RetainedCloneRef<'_, JackSnapshot>, command: RetainedCloneRef<'_, Command>,
        children: &ChildContentView, operation: &AppOperationContext, source: OriginalSourceBinding, effects: &[GraphEffect],
        cx: &mut StepContext<'_>,
    ) -> Result<Option<Vec<ChildEmitPreparation>>, OriginalTurnFailure> {
        if self.closing || self.complete { return Err(ValueError::invariant("Jack child qualification is terminal").into()); }
        if !source.matches(cx) {
            return Err(ValueError::invariant("Jack child qualification changed its original source binding").into());
        }
        if cx.should_yield() || cx.is_cancelled() { return Ok(None); }
        let (registration, editor) = C::child_editor().ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "Jack content has no registered child editor"))?;
        let child_id = &parent.get().content.child_id;
        let key = MemberKeyRef::root(SLOT, child_id);
        let owners = self.owners.original_mut().ok_or_else(|| ValueError::invariant("Jack child qualification has no active owner"))?;
        if owners.lookup.is_none() {
            let demand = OriginalChildMemberLookup::birth_demand();
            if cx.retained_work_can_enter(demand) {
                let (lookup, progress) = OriginalChildMemberLookup::admit(cx.retained_grant())?;
                owners.lookup = Some(lookup);
                cx.consume_fuel(1);
                cx.consume_retained(progress)?;
            }
            return Ok(None);
        }
        let lookup = owners.lookup.as_mut().ok_or_else(|| ValueError::invariant("Jack child lookup owner is absent"))?;
        lookup.next_demand::<C::ContentSnapshot, _>(parent, children, key, operation)?;
        if !lookup.ready() {
            let demand = lookup.demand::<C::ContentSnapshot>();
            if cx.retained_work_can_enter(demand) {
                let result = registration.read_original(parent, children, key, operation, lookup, cx.retained_grant());
                cx.consume_fuel(1);
                let progress = result.map_err(|error| OriginalTurnFailure::debit_original(error, cx))?.progress;
                cx.consume_retained(progress)?;
            }
            return Ok(None);
        }
        if owners.read.is_none() {
            let demand = ChildReadCursor::birth_demand();
            if cx.retained_work_can_enter(demand) {
                let (read, progress) = ChildReadCursor::admit(cx.retained_grant())?;
                owners.read = Some(read);
                cx.consume_fuel(1);
                cx.consume_retained(progress)?;
            }
            return Ok(None);
        }
        if owners.child.ready() {
            if owners.mutations.is_none() {
                if cx.retained_grant().maximum_items == 0 { return Ok(None); }
                let mutations = C::child_mutations(owners.child.source()?.get(), effects)?;
                cx.consume_fuel(1);
                if mutations.is_empty() {
                    self.complete = true;
                    return Ok(Some(Vec::new()));
                }
                owners.mutations = Some(mutations);
                return Ok(None);
            }
            if owners.qualifier.is_none() {
                let demand = OriginalChildEditorCommandCursor::birth_demand();
                if cx.retained_work_can_enter(demand) {
                    let (qualifier, progress) = OriginalChildEditorCommandCursor::admit(cx.retained_grant())?;
                    owners.qualifier = Some(qualifier);
                    cx.consume_fuel(1);
                    cx.consume_retained(progress)?;
                }
                return Ok(None);
            }
        }
        if let Some(authority) = owners.authority.as_ref() {
            let preparation_demand = ChildEmitPreparation::owned_qualified_batch_birth_demand::<C::ContentMutation>(SLOT.len(), child_id.len())?;
            let header = 2 * size_of::<Vec<ChildEmitPreparation>>();
            let capacity = size_of::<ChildEmitPreparation>();
            let extent = || ValueError::invariant("Jack child preparation queue extent overflow");
            let demand = RetirementDemand {
                copy_bytes: preparation_demand.copy_bytes.checked_add(header).ok_or_else(extent)?,
                capacity_bytes: preparation_demand.capacity_bytes.checked_add(capacity).ok_or_else(extent)?,
                ..preparation_demand
            };
            if !cx.retained_work_can_enter(demand) { return Ok(None); }
            if authority.stamp().is_none_or(|stamp| stamp.capability_identity() != &editor.identity as *const _ as usize) {
                return Err(ValueError::invariant("Jack child command stamp changed its registered editor").into());
            }
            let result = ChildEmitPreparation::admit_owned_batch_qualified::<C::ContentSnapshot, C::ContentMutation>(
                SLOT, child_id, &mut owners.mutations, &mut owners.authority, cx.retained_grant()
            );
            cx.consume_fuel(1);
            let Some((preparation, progress)) = result.map_err(|error| OriginalTurnFailure::debit_original(error, cx))? else { return Ok(None); };
            let output = vec![preparation];
            let progress = progress.checked_add(RetainedCloneProgress {
                copied_bytes: header,
                retained_capacity_bytes: output.capacity() * capacity,
                ..Default::default()
            })?;
            cx.consume_retained(progress)?;
            self.complete = true;
            return Ok(Some(output));
        }

        let raw = erase_child_admission_source(parent, PARENT_SOURCE);
        let missing_read = || ValueError::invariant("Jack child read qualification cursor is absent");
        let missing_command = || ValueError::invariant("Jack child command qualification cursor is absent");
        let demand = lookup.quote_ready_receiving::<C::ContentSnapshot, _>(parent, children, key, operation, |read| {
            if self.stamp.is_none() {
                owners.read.as_ref().ok_or_else(missing_read)?.next_demand(raw, &JACK_CONTENT_REFERENCE, 0, read, &registration.identity)
            } else if !owners.child.ready() {
                owners.child.next_admit_demand()
            } else if self.qualified {
                owners.qualifier.as_ref().ok_or_else(missing_command)?.take_authority_demand()
            } else {
                owners.qualifier.as_ref().ok_or_else(missing_command)?.next_demand()
            }
        })?;
        if !cx.retained_work_can_enter(demand) { return Ok(None); }
        let metadata = lookup.borrow_ready_demand::<C::ContentSnapshot>();
        let turn = lookup.borrow_ready::<C::ContentSnapshot, _>(parent, children, key, operation, cx.retained_grant());
        let borrowed = match turn.result {
            Ok(borrowed) => borrowed,
            Err(error) => {
                cx.consume_work(turn.work);
                return Err(OriginalTurnFailure::debit_original(error, cx));
            }
        };
        let read = borrowed.read.ok_or_else(|| ValueError::invariant("Jack ready child lookup returned no read under its quoted grant"))?;
        let grant = RetainedCloneGrant { maximum_copy_bytes: cx.retained_grant().maximum_copy_bytes - metadata.copy_bytes, ..cx.retained_grant() };
        let stepped = cx.with_original_source_effect(grant, |_, effect| -> Result<RetainedCloneProgress, ValueError> {
            match self.stamp.as_ref() {
                None => {
                    let advance = registration.qualify(raw, &JACK_CONTENT_REFERENCE, 0, &read, owners.read.as_mut().ok_or_else(missing_read)?, effect)?;
                    self.stamp = advance.stamp;
                    Ok(advance.progress)
                }
                Some(stamp) if !owners.child.ready() => Ok(registration.admit_original_source(&read, stamp, &mut owners.child, effect)?.progress()),
                Some(_) if self.qualified => {
                    let advance = owners.qualifier.as_mut().ok_or_else(missing_command)?.take_authority(&owners.child, &read, effect)?;
                    owners.authority = advance.authority;
                    Ok(advance.progress)
                }
                Some(stamp) => {
                    let qualified_parent = parent.project(PARENT_SOURCE, |snapshot| snapshot);
                    let advance = editor.qualify_original(qualified_parent, command, stamp, &read, owners.qualifier.as_mut().ok_or_else(missing_command)?, effect)?;
                    self.qualified = advance.stamp.is_some();
                    Ok(advance.progress)
                }
            }
        })?;
        let (result, performed) = stepped.unwrap_or((Ok(Default::default()), 0));
        cx.consume_work(turn.work.max(performed));
        let merge = |progress: RetainedCloneProgress| -> Result<RetainedCloneProgress, ValueError> {
            let mut whole = borrowed.progress.checked_add(progress)?;
            whole.copied_items = borrowed.progress.copied_items.max(progress.copied_items);
            Ok(whole)
        };
        match result {
            Ok(progress) => cx.consume_retained(merge(progress)?)?,
            Err(error) => {
                let progress = merge(error.retained_progress())?;
                return Err(OriginalTurnFailure::debit_original(error.with_retained_progress(progress), cx));
            }
        }
        Ok(None)
    }

    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { Ok(Default::default()) } else { self.owners.next_demand(body) }
    }

    pub fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        self.owners.step(body, effect)
    }

    /// ♻️ Closes the same read and command sources through their original native issuer.
    pub fn close_turn(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if !self.closing { return Err(ValueError::invariant("Jack child qualification close has not begun").into()); }
        let grant = cx.retained_grant();
        let body = grant.maximum_copy_bytes;
        let demand = self.close_demands(body)?;
        let birth = self.owners.requires_original_birth();
        if !birth && !self.owners.requires_original_control() {
            return semio_framework_job::close_original_effect_step(cx, demand, |effect| self.close_step(body, effect));
        }
        let pending = || InteractiveJobCloseStep::Pending { progress: Default::default() };
        if cx.retirement_should_yield() || !cx.retained_work_can_enter(demand) { return Ok(pending()); }
        let Some((poll, _)) = cx.with_original_source_effect(grant, |cx, effect| cx.with_native_retirement(|native| {
            native.with_original_close(|original| Ok::<_, ValueError>(if birth {
                self.owners.step_original_birth(original, grant)
            } else {
                self.owners.close_original_step(original, body, effect)
            }))
        }))? else { return Ok(pending()); };
        let turn = match poll? {
            RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work },
            RetainedWorkPoll::Complete(turn) => turn,
        };
        let progress = semio_framework_job::receive_original_retained_work(cx, turn)?.progress();
        Ok(if self.terminal_is_empty() { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.owners.terminal_is_empty() }
}
