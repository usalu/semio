//! 🪆️ Original retained clone of a composed child handle: the identity fields are copied, the local-only owner is aliased.
use crate::ArtifactChild;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_value::retained_clone::{admit_retained_clone_close, admit_retained_clone_progress, close_retained_binding, RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, RetainedFieldCursor, nested_retained_clone_demands, nested_retained_clone_close_demands, nested_retained_clone_grant, retained_clone_grant_admits, retained_clone_close_demands};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, RetirementStep};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::{any::Any, marker::PhantomData, sync::Arc};

/// 🔗️ One strong alias of a child's immutable local-only materialization; retiring it only releases the alias.
pub struct ArtifactChildLocalOwnerAlias(Arc<dyn Any + Send + Sync>);

struct AliasRetirement(Option<ArtifactChildLocalOwnerAlias>);
impl RetirementCursor for AliasRetirement {
    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand { depth: usize::from(self.0.is_some()), ..Default::default() })}
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return RetirementStep::BudgetExhausted;
        }
        self.0.take();
        RetirementStep::Complete(1)
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
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(AliasRetirement(Some(self)))
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<AliasRetirement>())
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

fn field_demands<T: RetainedClone>(cursor: &RetainedFieldCursor<T>, draining: bool, source: RetainedCloneRef<'_,T>, body:usize) -> Result<semio_framework_value::RetirementDemand,ValueError> {
    if draining { return if cursor.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand {depth:1,..Default::default()}) } else { nested_retained_clone_close_demands::<T>(cursor,body) }; }
    if cursor.output_ready() { return Ok(semio_framework_value::RetirementDemand {copy_bytes:size_of::<T>(),depth:1,..Default::default()}); }
    nested_retained_clone_demands::<T>(cursor,source,body)
}

fn advance_field<T: RetainedClone>(cursor: &mut RetainedFieldCursor<T>, value: &mut Option<T>, draining: &mut bool, source: RetainedCloneRef<'_, T>, body:usize, grant: RetainedCloneGrant,work:&mut u64) -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
    if cursor.output_ready() {
        let turn=cursor.take(nested_retained_clone_grant(grant)?);*work+=turn.work;*value=Some(turn.result?.ok_or_else(||invariant("retained child ready field omitted owner"))?.0);
        if cursor.begin_close(){*work+=1}*draining=true;
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<T>(),..Default::default()}));
    }
    let nested=nested_retained_clone_grant(grant)?;
    let turn=cursor.advance(source,body,nested);*work+=turn.work;let step=turn.result?;
    Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,step.progress(),"retained child field")?))
}
fn drain_field<T: RetainedClone>(cursor: &mut RetainedFieldCursor<T>, draining: &mut bool, phase: &mut usize, grant: RetainedCloneGrant,work:&mut u64) -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
    if !cursor.terminal_is_empty() {
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let turn=cursor.close_step(nested_retained_clone_grant(grant)?);*work+=turn.work;let step=turn.result?;
        let progress = admit_retained_clone_close(grant, step, cursor.terminal_is_empty(), "retained child field scaffold close")?.progress();
        return Ok(RetainedCloneStep::Progress(progress));
    }
    if grant.maximum_items == 0 {
        return Ok(RetainedCloneStep::Progress(Default::default()));
    }
    *draining = false;
    *phase += 1;
    *work+=1;
    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
}

fn close_field<T: RetainedClone>(cursor: &mut RetainedFieldCursor<T>, grant: RetainedCloneGrant,work:&mut u64) -> Result<Option<RetainedCloneStep>, semio_framework_value::OriginalRetirementFailure> {
    if cursor.terminal_is_empty() {
        return Ok(None);
    }
    if cursor.begin_close() {
        *work+=1;return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })));
    }
    let turn=cursor.close_step(nested_retained_clone_grant(grant)?);*work+=turn.work;let step=turn.result?;
    Ok(Some(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, cursor.terminal_is_empty(), "retained child close")?.progress())))
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
impl<S: Send + Sync + 'static> RetainedCloneCursor<ArtifactChild<S>> for ArtifactChildCloneCursor<S> {
    fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{ArtifactChildCloneCursor::<S>::close_demands(self,body)}
    fn output_ready(&self) -> bool { !self.closing && !self.spent && self.output.is_some() }
    fn close_started(&self) -> bool { self.closing }

    fn advance_demands(&self,source:RetainedCloneRef<'_,ArtifactChild<S>>,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {
        if self.closing || self.spent { return Err(invariant("retained child clone is closing or spent")); }
        if !source.check_binding(&self.source)? { return Ok(semio_framework_value::RetirementDemand {copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()}); }
        if self.output_ready() { return Ok(Default::default()); }
        match self.phase {
            0=>field_demands(&self.child_id_cursor,self.draining,source.project(1,|child|&child.child_id),body),
            1=>field_demands(&self.target_cursor,self.draining,source.project(2,|child|&child.target),body),
            2=>Ok(semio_framework_value::RetirementDemand {copy_bytes:if source.get().local_owner.is_some(){size_of::<ArtifactChildLocalOwnerAlias>()}else{0},depth:1,..Default::default()}),
            3=>Ok(semio_framework_value::RetirementDemand {copy_bytes:size_of::<ArtifactChild<S>>(),depth:1,..Default::default()}),
            _=>Err(invariant("retained child clone phase invalid")),
        }
    }
    fn advance_work_demand(&self,source:RetainedCloneRef<'_,ArtifactChild<S>>,body:usize)->Result<u64,ValueError>{
        self.advance_demands(source,body)?;
        if !source.check_binding(&self.source)?{return Ok(1)}
        if self.output_ready(){return Ok(0)}
        fn field<T:RetainedClone>(cursor:&RetainedFieldCursor<T>,draining:bool,source:RetainedCloneRef<'_,T>,body:usize)->Result<u64,ValueError>{
            if draining{return if cursor.terminal_is_empty(){Ok(1)}else{cursor.close_work_demand(body)}}
            if cursor.output_ready(){return cursor.take_work_demand()?.checked_add(u64::from(!cursor.close_started())).ok_or_else(||invariant("retained child field work overflow"))}
            cursor.advance_work_demand(source,body)
        }
        match self.phase{0=>field(&self.child_id_cursor,self.draining,source.project(1,|child|&child.child_id),body),1=>field(&self.target_cursor,self.draining,source.project(2,|child|&child.target),body),2|3=>Ok(1),_=>Err(invariant("retained child clone phase invalid"))}
    }
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if !self.closing{return Ok(0)}
        fn field<T:RetainedClone>(cursor:&RetainedFieldCursor<T>,body:usize)->Result<Option<u64>,ValueError>{if cursor.terminal_is_empty(){Ok(None)}else if !cursor.close_started(){Ok(Some(1))}else{cursor.close_work_demand(body).map(Some)}}
        if let Some(work)=field(&self.child_id_cursor,body)?{return Ok(work)}
        if let Some(work)=field(&self.target_cursor,body)?{return Ok(work)}
        if !self.close.is_empty(){return self.close.next_owner_work::<ArtifactChild<S>>(false,&None,body)}
        if self.child_id_value.is_some(){return self.close.next_owner_work::<String>(true,&None,body)}
        if self.target_value.is_some(){return self.close.next_owner_work::<ArtifactRef>(true,&None,body)}
        if self.local_owner_value.is_some(){return self.close.next_owner_work::<ArtifactChildLocalOwnerAlias>(true,&None,body)}
        self.close.next_owner_work::<ArtifactChild<S>>(self.output.is_some(),&self.source,body)
    }
    fn advance(&mut self, source: RetainedCloneRef<'_, ArtifactChild<S>>, body:usize, grant: RetainedCloneGrant) -> semio_framework_value::OriginalRetirementOutcome {
        let mut work=0;let result = (|| -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        if self.closing {
            return Err(invariant("retained child clone cursor is closing").into());
        }
        if self.spent {
            return Err(invariant("retained child clone cursor is spent").into());
        }
        let demand=self.advance_demands(source,body)?;
        if self.output_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let turn=source.bind(&mut self.source, grant);work+=turn.work;if let Some(progress) = turn.result? {
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if self.draining {
            return match self.phase {
                0 => drain_field(&mut self.child_id_cursor, &mut self.draining, &mut self.phase, grant,&mut work),
                1 => drain_field(&mut self.target_cursor, &mut self.draining, &mut self.phase, grant,&mut work),
                _ => Err(invariant("retained child drain state is invalid").into()),
            };
        }
        match self.phase {
            0 => advance_field(&mut self.child_id_cursor, &mut self.child_id_value, &mut self.draining, source.project(1, |source| &source.child_id), body, grant,&mut work),
            1 => advance_field(&mut self.target_cursor, &mut self.target_value, &mut self.draining, source.project(2, |source| &source.target), body, grant,&mut work),
            2 => {
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                work+=1;self.local_owner_value = source.get().local_owner.as_ref().map(|owner| ArtifactChildLocalOwnerAlias(Arc::clone(owner)));
                self.phase += 1;
                Ok(RetainedCloneStep::Progress(progress))
            }
            3 => {
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                work+=1;self.output = Some(ArtifactChild {
                    child_id: self.child_id_value.take().expect("retained field owner"),
                    target: self.target_value.take().expect("retained field owner"),
                    local_owner: self.local_owner_value.take().map(|alias| alias.0),
                    _snapshot: PhantomData,
                });
                self.phase += 1;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..Default::default() }))
            }
            _ => Err(invariant("retained child clone state is invalid").into()),
        }
    })();
        semio_framework_value::RetainedWorkOutcome { result, work }
    }

    fn take_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<ArtifactChild<S>>(), depth: 1, ..Default::default() })
    }

    fn take(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<Option<(ArtifactChild<S>, RetainedCloneProgress)>> {
        let demand = semio_framework_value::RetirementDemand { copy_bytes: size_of::<ArtifactChild<S>>(), depth: 1, ..Default::default() };
        if !retained_clone_grant_admits(grant, demand) { return semio_framework_value::RetainedWorkOutcome { result: Ok(None), work: 0 }; }
        let output = self.output.take();
        if output.is_some() {
            self.spent = true;
        }
        let work=u64::from(output.is_some());
        semio_framework_value::RetainedWorkOutcome { result: Ok(output.map(|child| (child, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))), work }
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::OriginalRetirementOutcome {
        let mut work=0;let result = (|| -> Result<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure> {
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if !self.closing {
            return Err(invariant("retained child must begin close before granted retirement").into());
        }
        if !retained_clone_grant_admits(grant,retained_clone_close_demands::<ArtifactChild<S>>(self,grant.maximum_copy_bytes)?) {return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(step) = close_field(&mut self.child_id_cursor, grant,&mut work)? {
            return Ok(step);
        }
        if let Some(step) = close_field(&mut self.target_cursor, grant,&mut work)? {
            return Ok(step);
        }
        if !self.close.is_empty() {
            let turn=self.close.step_granted(grant);work+=turn.work;return turn.result;
        }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.child_id_value, grant);work+=turn.work;turn.result}? {
            return Ok(step);
        }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.target_value, grant);work+=turn.work;turn.result}? {
            return Ok(step);
        }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.local_owner_value, grant);work+=turn.work;turn.result}? {
            return Ok(step);
        }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.output, grant);work+=turn.work;turn.result}? {
            return Ok(step);
        }
        {let turn=close_retained_binding(&mut self.source, grant);work+=turn.work;turn.result}
    })();
        semio_framework_value::RetainedWorkOutcome { result, work }
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
