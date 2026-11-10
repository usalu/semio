//! 🪆️ Original retained clone of a composed child handle: the identity fields are copied, the local-only owner is aliased.
use crate::ArtifactChild;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_value::retained_clone::{admit_retained_clone_close, admit_retained_clone_progress, close_retained_binding, RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, RetainedCursorRelease, RetainedFieldCursor, nested_retained_clone_demands, nested_retained_clone_close_demands, nested_retained_clone_grant, retained_clone_grant_admits, retained_clone_close_demands};
use semio_framework_value::retirement::{record_retirement_work, RetireOwned, RetirementCursor, RetirementStep};
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementOutcome, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetirementPollExit, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{OriginalEffectReply, OriginalEffectReplyComplete};
use std::{any::Any, marker::PhantomData, sync::Arc};

/// 🔗️ One strong alias of a child's immutable local-only materialization; retiring it only releases the alias.
pub struct ArtifactChildLocalOwnerAlias(pub(crate) Arc<dyn Any + Send + Sync>);

#[doc(hidden)]
pub struct AliasRetirement(Option<ArtifactChildLocalOwnerAlias>);
impl RetirementCursor for AliasRetirement {
    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand { depth: usize::from(self.0.is_some()), ..Default::default() })}
    fn close_into<'reply>(&mut self, _body: usize, effect: &mut RetainedAllocationEffect<'_>, reply: OriginalEffectReply<'reply, RetirementStep, OriginalRetirementFailure>) -> Result<OriginalEffectReplyComplete<'reply, RetirementStep, OriginalRetirementFailure>, ValueError> {
        let grant = effect.grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(reply.complete(Ok(RetirementStep::BudgetExhausted)));
        }
        Ok(match record_retirement_work(effect, 1) {
            Ok(work) => { self.0.take(); reply.complete(Ok(RetirementStep::Complete(work))) }
            Err((error, _)) => reply.complete(Err(error.into())),
        })
    }
    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }
    fn next_close_byte_demand(&self) -> Option<usize> {
        Some(0)
    }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> {
        Some(0)
    }
    fn terminal_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

impl RetireOwned for ArtifactChildLocalOwnerAlias {
    type RetirementCursor = AliasRetirement;
    fn retirement_seed(self) -> AliasRetirement {
        AliasRetirement(Some(self))
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
    fn retirement_element_copy_bytes() -> usize {
        size_of::<Self>()
    }
}

/// 🧬️ Resumable clone of one `ArtifactChild<S>`: `childId`, `target`, then the local owner alias, then assembly.
#[doc(hidden)]
pub struct ArtifactChildCloneCursor<S> {
    phase: usize,
    output: Option<ArtifactChild<S>>,
    source: Option<RetainedCloneBinding>,
    spent: bool,
    draining: bool,
    closing: bool,
    close: RetainedCloneClose,
    child_id_cursor: RetainedFieldCursor<String>,
    child_id_value: Option<String>,
    target_cursor: RetainedFieldCursor<ArtifactRef>,
    target_value: Option<ArtifactRef>,
    local_owner_value: Option<ArtifactChildLocalOwnerAlias>,
}

impl<S> Default for ArtifactChildCloneCursor<S> {
    fn default() -> Self {
        Self { phase: 0, output: None, source: None, spent: false, draining: false, closing: false, close: Default::default(), child_id_cursor: Default::default(), child_id_value: None, target_cursor: Default::default(), target_value: None, local_owner_value: None }
    }
}

fn invariant(reason: &'static str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvariantViolated, reason)
}

fn field_demands<T: RetainedClone>(cursor: &RetainedFieldCursor<T>, draining: bool, source: RetainedCloneRef<'_,T>, body:usize, maximum_work_units:u64) -> Result<semio_framework_value::RetirementDemand,ValueError> {
    if draining { return if cursor.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand {depth:1,..Default::default()}) } else { nested_retained_clone_close_demands::<T>(cursor,body) }; }
    if cursor.output_ready() { return Ok(semio_framework_value::RetirementDemand {copy_bytes:size_of::<T>()+size_of::<bool>(),depth:1,..Default::default()}); }
    nested_retained_clone_demands::<T>(cursor,source,body,maximum_work_units)
}

fn native_reply(run: impl for<'r> FnOnce(OriginalEffectReply<'r, RetainedCloneStep, OriginalRetirementFailure>) -> Result<OriginalEffectReplyComplete<'r, RetainedCloneStep, OriginalRetirementFailure>, ValueError>) -> Result<RetainedCloneStep, OriginalRetirementFailure> {
    let mut slot = None;
    {let reply = OriginalEffectReply::borrow(&mut slot)?; drop(run(reply)?);}
    slot.ok_or_else(|| invariant("retained child field stayed pending on a synchronous recipient"))?
}
fn reply_of<'r>(poll: OriginalRetirementPoll, reply: OriginalEffectReply<'r, RetainedCloneStep, OriginalRetirementFailure>) -> OriginalEffectReplyComplete<'r, RetainedCloneStep, OriginalRetirementFailure> {
    match poll { RetainedWorkPoll::Pending { .. } => reply.pending(), RetainedWorkPoll::Complete(turn) => reply.complete(turn.result) }
}
fn advance_field<T: RetainedClone>(cursor: &mut RetainedFieldCursor<T>, value: &mut Option<T>, draining: &mut bool, source: RetainedCloneRef<'_, T>, body:usize, maximum_work_units:u64, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, OriginalRetirementFailure> {
    let grant = effect.grant();
    if cursor.output_ready() {
        let (mut candidate, mut slot) = (None, None);
        {let reply = OriginalEffectReply::borrow(&mut slot)?; cursor.take(&mut candidate, effect, reply)?;}
        slot.take().ok_or_else(|| invariant("retained child field completion left no reply"))??;
        *value = Some(candidate.ok_or_else(|| invariant("retained child ready field omitted owner"))?);
        cursor.begin_close_paid(effect).result?;
        *draining = cursor.close_started();
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<T>(),..Default::default()}));
    }
    let nested = nested_retained_clone_grant(grant)?;
    let mut inner = effect.nested(nested);
    let step = native_reply(|reply| cursor.advance_into(source, body, maximum_work_units, &mut inner, reply))?;
    Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested, step.progress(), "retained child field")?))
}
fn drain_field<T: RetainedClone>(cursor: &mut RetainedFieldCursor<T>, body: usize, draining: &mut bool, phase: &mut usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, RetirementPollExit> {
    let grant = effect.grant();
    if !cursor.terminal_is_empty() {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let mut inner = effect.nested(nested_retained_clone_grant(grant)?);
        let step = native_reply(|reply| cursor.close_into(body, &mut inner, reply))?;
        let progress = admit_retained_clone_close(grant, step, cursor.terminal_is_empty(), "retained child field scaffold close")?.progress();
        return Ok(RetainedCloneStep::Progress(progress));
    }
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
    *draining = false;
    *phase += 1;
    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
}

fn close_field<T: RetainedClone>(cursor: &mut RetainedFieldCursor<T>, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<RetainedCloneStep>, RetirementPollExit> {
    let grant = effect.grant();
    if cursor.terminal_is_empty() { return Ok(None); }
    if !cursor.close_started() {
        let turn = cursor.begin_close_paid(effect);
        return Ok(Some(RetainedCloneStep::Progress(turn.result?)));
    }
    let mut inner = effect.nested(nested_retained_clone_grant(grant)?);
    let step = native_reply(|reply| cursor.close_into(body, &mut inner, reply))?;
    Ok(Some(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, cursor.terminal_is_empty(), "retained child close")?.progress())))
}

fn settle_outcome(effect: &mut RetainedAllocationEffect<'_>, result: Result<RetainedCloneStep, OriginalRetirementFailure>) -> OriginalRetirementOutcome {
    let result = match result {
        Ok(step) if effect.performed_work() == 0 && step.progress() != RetainedCloneProgress::default() => match effect.performed(1) {
            Ok(true) => Ok(step),
            Ok(false) => Err(OriginalRetirementFailure::from_work_refusal(ValueError::literal(ValueRefusalKind::WorkLimit, "retained child step exceeds its work ceiling").with_retained_progress(step.progress()))),
            Err(error) => Err(OriginalRetirementFailure::from_work_refusal(error.with_retained_progress(step.progress()))),
        },
        other => other,
    };
    RetainedWorkOutcome { result, work: effect.performed_work() }
}

impl<S:Send+Sync+'static> ArtifactChildCloneCursor<S>{
    fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        #[cfg(test)] selected_close_tests::record(self as*const Self as usize,body);
        if !self.closing{return Ok(Default::default());}
        if !self.child_id_cursor.terminal_is_empty(){return nested_retained_clone_close_demands::<String>(&self.child_id_cursor,body);}
        if !self.target_cursor.terminal_is_empty(){return nested_retained_clone_close_demands::<ArtifactRef>(&self.target_cursor,body);}
        if self.child_id_value.is_some(){return self.close.next_owner_demands::<String>(true,&self.source,body);}
        if self.target_value.is_some(){return self.close.next_owner_demands::<ArtifactRef>(true,&self.source,body);}
        if self.local_owner_value.is_some(){return self.close.next_owner_demands::<ArtifactChildLocalOwnerAlias>(true,&self.source,body);}
        self.close.next_owner_demands::<ArtifactChild<S>>(self.output.is_some(),&self.source,body)
    }
}
impl<S: Send + Sync + 'static> ArtifactChildCloneCursor<S> {
    fn close_action(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, RetirementPollExit> {
        let grant = effect.grant();
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(invariant("retained child must begin close before granted retirement").into()); }
        if !retained_clone_grant_admits(grant, retained_clone_close_demands::<ArtifactChild<S>>(self, grant.maximum_copy_bytes)?) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(step) = close_field(&mut self.child_id_cursor, body, effect)? { return Ok(step); }
        if let Some(step) = close_field(&mut self.target_cursor, body, effect)? { return Ok(step); }
        if !self.close.is_empty() { return RetirementPollExit::resume(self.close.step_granted(body, effect)); }
        if let Some(step) = self.close.begin_granted(&mut self.child_id_value, effect).result? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.target_value, effect).result? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.local_owner_value, effect).result? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, effect).result? { return Ok(step); }
        RetirementPollExit::resume(close_retained_binding(&mut self.source, body, effect))
    }
}
fn exit_failure(exit: RetirementPollExit) -> OriginalRetirementFailure {
    match exit { RetirementPollExit::Failed(failure) => failure, RetirementPollExit::Pending(_) => invariant("retained child field close stayed pending on a synchronous recipient").into() }
}
impl<S: Send + Sync + 'static> RetainedCloneCursor<ArtifactChild<S>> for ArtifactChildCloneCursor<S> {
    fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{ArtifactChildCloneCursor::<S>::close_demands(self,body)}
    fn native_terminal_release(&self) -> Option<RetainedCursorRelease<Self>> { self.terminal_is_empty().then(|| unsafe { RetainedCursorRelease::native(|cursor: &Self| cursor.terminal_is_empty()) }) }
    fn output_ready(&self) -> bool { !self.closing && !self.spent && self.output.is_some() }
    fn close_started(&self) -> bool { self.closing }

    fn advance_demands(&self,source:RetainedCloneRef<'_,ArtifactChild<S>>,body:usize,maximum_work_units:u64)->Result<semio_framework_value::RetirementDemand,ValueError> {
        if self.closing || self.spent { return Err(invariant("retained child clone is closing or spent")); }
        if !source.check_binding(&self.source)? { return Ok(semio_framework_value::RetirementDemand {copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()}); }
        if self.output_ready() { return Ok(Default::default()); }
        match self.phase {
            0=>field_demands(&self.child_id_cursor,self.draining,source.project(1,|child|&child.child_id),body,maximum_work_units),
            1=>field_demands(&self.target_cursor,self.draining,source.project(2,|child|&child.target),body,maximum_work_units),
            2=>Ok(semio_framework_value::RetirementDemand {copy_bytes:if source.get().local_owner.is_some(){size_of::<ArtifactChildLocalOwnerAlias>()}else{0},depth:1,..Default::default()}),
            3=>Ok(semio_framework_value::RetirementDemand {copy_bytes:size_of::<ArtifactChild<S>>(),depth:1,..Default::default()}),
            _=>Err(invariant("retained child clone phase invalid")),
        }
    }
    fn advance_work_demand(&self,source:RetainedCloneRef<'_,ArtifactChild<S>>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
        self.advance_demands(source,body,maximum_work_units)?;
        if !source.check_binding(&self.source)?{return Ok(1)}
        if self.output_ready(){return Ok(0)}
        fn field<T:RetainedClone>(cursor:&RetainedFieldCursor<T>,draining:bool,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{
            if draining{return if cursor.terminal_is_empty(){Ok(1)}else{cursor.close_work_demand(body)}}
            if cursor.output_ready(){return cursor.take_work_demand()?.checked_add(cursor.begin_close_work_demand()?).ok_or_else(||invariant("retained child field work overflow"))}
            cursor.advance_work_demand(source,body,maximum_work_units)
        }
        match self.phase{0=>field(&self.child_id_cursor,self.draining,source.project(1,|child|&child.child_id),body,maximum_work_units),1=>field(&self.target_cursor,self.draining,source.project(2,|child|&child.target),body,maximum_work_units),2|3=>Ok(1),_=>Err(invariant("retained child clone phase invalid"))}
    }
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if !self.closing{return Ok(0)}
        fn field<T:RetainedClone>(cursor:&RetainedFieldCursor<T>,body:usize)->Result<Option<u64>,ValueError>{if cursor.terminal_is_empty(){Ok(None)}else if !cursor.close_started(){cursor.begin_close_work_demand().map(Some)}else{cursor.close_work_demand(body).map(Some)}}
        if let Some(work)=field(&self.child_id_cursor,body)?{return Ok(work)}
        if let Some(work)=field(&self.target_cursor,body)?{return Ok(work)}
        if !self.close.is_empty(){return self.close.next_owner_work::<ArtifactChild<S>>(false,&None,body)}
        if self.child_id_value.is_some(){return self.close.next_owner_work::<String>(true,&None,body)}
        if self.target_value.is_some(){return self.close.next_owner_work::<ArtifactRef>(true,&None,body)}
        if self.local_owner_value.is_some(){return self.close.next_owner_work::<ArtifactChildLocalOwnerAlias>(true,&None,body)}
        self.close.next_owner_work::<ArtifactChild<S>>(self.output.is_some(),&self.source,body)
    }
    fn advance_into<'reply>(&mut self, source: RetainedCloneRef<'_, ArtifactChild<S>>, body:usize, maximum_work_units:u64, effect: &mut RetainedAllocationEffect<'_>, reply: OriginalEffectReply<'reply, RetainedCloneStep, OriginalRetirementFailure>) -> Result<OriginalEffectReplyComplete<'reply, RetainedCloneStep, OriginalRetirementFailure>, ValueError> {
        let grant = effect.grant();
        let result = (|| -> Result<RetainedCloneStep, OriginalRetirementFailure> {
        if self.closing { return Err(invariant("retained child clone cursor is closing").into()); }
        if self.spent { return Err(invariant("retained child clone cursor is spent").into()); }
        let demand=self.advance_demands(source,body,maximum_work_units)?;
        if self.output_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let turn=source.bind(&mut self.source, effect);
        if let Some(progress) = turn.result? { return Ok(RetainedCloneStep::Progress(progress)); }
        if self.draining {
            return match self.phase {
                0 => drain_field(&mut self.child_id_cursor, body, &mut self.draining, &mut self.phase, effect).map_err(exit_failure),
                1 => drain_field(&mut self.target_cursor, body, &mut self.draining, &mut self.phase, effect).map_err(exit_failure),
                _ => Err(invariant("retained child drain state is invalid").into()),
            };
        }
        match self.phase {
            0 => advance_field(&mut self.child_id_cursor, &mut self.child_id_value, &mut self.draining, source.project(1, |source| &source.child_id), body, maximum_work_units, effect),
            1 => advance_field(&mut self.target_cursor, &mut self.target_value, &mut self.draining, source.project(2, |source| &source.target), body, maximum_work_units, effect),
            2 => {
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
                if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.local_owner_value = source.get().local_owner.as_ref().map(|owner| ArtifactChildLocalOwnerAlias(Arc::clone(owner)));
                self.phase += 1;
                Ok(RetainedCloneStep::Progress(progress))
            }
            3 => {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let absent = || invariant("retained field owner");
                self.output = Some(ArtifactChild {
                    child_id: self.child_id_value.take().ok_or_else(absent)?,
                    target: self.target_value.take().ok_or_else(absent)?,
                    local_owner: self.local_owner_value.take().map(|alias| alias.0),
                    _snapshot: PhantomData,
                });
                self.phase += 1;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..Default::default() }))
            }
            _ => Err(invariant("retained child clone state is invalid").into()),
        }
    })();
        Ok(reply.complete(settle_outcome(effect, result).result))
    }

    fn take_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<ArtifactChild<S>>(), depth: 1, ..Default::default() })
    }
    fn take_work_demand(&self) -> Result<u64, ValueError> { Ok(1) }

    fn take<'reply>(&mut self, output: &mut Option<ArtifactChild<S>>, effect: &mut RetainedAllocationEffect<'_>, reply: OriginalEffectReply<'reply, RetainedCloneStep, ValueError>) -> Result<OriginalEffectReplyComplete<'reply, RetainedCloneStep, ValueError>, ValueError> {
        let demand = semio_framework_value::RetirementDemand { copy_bytes: size_of::<ArtifactChild<S>>(), depth: 1, ..Default::default() };
        if self.output.is_none() || output.is_some() { return Ok(reply.complete(Err(invariant("retained child take requires its resident owner and an empty destination")))); }
        if !retained_clone_grant_admits(effect.grant(), demand) || effect.maximum_work_units() == 0 { return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default())))); }
        match effect.performed(1) {
            Ok(true) => {},
            Ok(false) => return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit, "retained child take exceeded its original work")))),
            Err(error) => return Ok(reply.complete(Err(error))),
        }
        *output = self.output.take();
        self.spent = true;
        Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))))
    }

    fn begin_close_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        Ok(if self.closing { Default::default() } else { semio_framework_value::RetirementDemand { copy_bytes: size_of::<bool>(), depth: 1, ..Default::default() } })
    }
    fn begin_close_work_demand(&self) -> Result<u64, ValueError> { Ok(u64::from(!self.closing)) }
    fn begin_close_paid(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<RetainedCloneProgress, ValueError> {
        if self.closing { return RetainedWorkOutcome { result: Ok(Default::default()), work: 0 }; }
        let demand = semio_framework_value::RetirementDemand { copy_bytes: size_of::<bool>(), depth: 1, ..Default::default() };
        if effect.maximum_work_units() == 0 || !retained_clone_grant_admits(effect.grant(), demand) { return RetainedWorkOutcome { result: Ok(Default::default()), work: 0 }; }
        match effect.performed(1) {
            Ok(true) => {},
            Ok(false) => return RetainedWorkOutcome { result: Err(ValueError::literal(ValueRefusalKind::WorkLimit, "retained child close start exceeded its original work")), work: 1 },
            Err(error) => return RetainedWorkOutcome { result: Err(error), work: 1 },
        }
        self.closing = true;
        RetainedWorkOutcome { result: Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }), work: 1 }
    }

    fn begin_close_into<'reply>(&mut self, effect: &mut RetainedAllocationEffect<'_>, reply: OriginalEffectReply<'reply, RetainedCloneStep, OriginalRetirementFailure>) -> Result<OriginalEffectReplyComplete<'reply, RetainedCloneStep, OriginalRetirementFailure>, ValueError> {
        let turn = self.begin_close_paid(effect);
        Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))
    }

    fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll {
        let result = self.close_action(body, effect);
        RetirementPollExit::settle(effect, result)
    }
    fn close_into<'reply>(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>, reply: OriginalEffectReply<'reply, RetainedCloneStep, OriginalRetirementFailure>) -> Result<OriginalEffectReplyComplete<'reply, RetainedCloneStep, OriginalRetirementFailure>, ValueError> {
        Ok(reply_of(self.close_step(body, effect), reply))
    }

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none() && self.child_id_cursor.terminal_is_empty() && self.child_id_value.is_none() && self.target_cursor.terminal_is_empty() && self.target_value.is_none() && self.local_owner_value.is_none()
    }
}

impl<S: Send + Sync + 'static> RetainedClone for ArtifactChild<S> {
    type Cursor = ArtifactChildCloneCursor<S>;
    fn retained_clone_cursor() -> Self::Cursor {
        Default::default()
    }
}

#[cfg(test)]
#[path="🧪️tests/🎯️dispatch/🦀️.rs"]
mod selected_close_tests;
