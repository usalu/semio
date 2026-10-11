//! 🪆️ Original retained clone of a composed child handle: the identity fields are copied, the local-only owner is aliased.
use super::ArtifactChild;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_value::retained_clone::{admit_retained_clone_close, admit_retained_clone_progress, close_retained_binding, RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, RetainedFieldCursor};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, RetirementStep};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::{any::Any, marker::PhantomData, sync::Arc};

/// 🔗️ One strong alias of a child's immutable local-only materialization; retiring it only releases the alias.
pub struct ArtifactChildLocalOwnerAlias(Arc<dyn Any + Send + Sync>);

struct AliasRetirement(Option<ArtifactChildLocalOwnerAlias>);
impl RetirementCursor for AliasRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return RetirementStep::BudgetExhausted;
        }
        self.0.take();
        RetirementStep::Complete
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

fn advance_field<T: RetainedClone>(cursor: &mut RetainedFieldCursor<T>, value: &mut Option<T>, draining: &mut bool, source: RetainedCloneRef<'_, T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    match cursor.advance(source, grant)? {
        RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained child field")?)),
        RetainedCloneStep::Complete(progress) => {
            let progress = admit_retained_clone_progress(grant, progress, "retained child field")?;
            if progress.copied_items >= grant.maximum_items {
                return Ok(RetainedCloneStep::Progress(progress));
            }
            *value = Some(cursor.take().ok_or_else(|| invariant("retained child field completed without an owner"))?);
            let _ = cursor.begin_close();
            *draining = true;
            Ok(RetainedCloneStep::Progress(progress.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
        }
    }
}

fn drain_field<T: RetainedClone>(cursor: &mut RetainedFieldCursor<T>, draining: &mut bool, phase: &mut usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    if !cursor.terminal_is_empty() {
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let step = cursor.close_step(grant)?;
        let progress = admit_retained_clone_close(grant, step, cursor.terminal_is_empty(), "retained child field scaffold close")?.progress();
        return Ok(RetainedCloneStep::Progress(progress));
    }
    if grant.maximum_items == 0 {
        return Ok(RetainedCloneStep::Progress(Default::default()));
    }
    *draining = false;
    *phase += 1;
    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
}

fn close_field<T: RetainedClone>(cursor: &mut RetainedFieldCursor<T>, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, ValueError> {
    if cursor.terminal_is_empty() {
        return Ok(None);
    }
    if cursor.begin_close() {
        return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })));
    }
    let step = cursor.close_step(grant)?;
    Ok(Some(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, cursor.terminal_is_empty(), "retained child close")?.progress())))
}

impl<S: Send + Sync + 'static> RetainedCloneCursor<ArtifactChild<S>> for ArtifactChildCloneCursor<S> {
    fn advance(&mut self, source: RetainedCloneRef<'_, ArtifactChild<S>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.closing {
            return Err(invariant("retained child clone cursor is closing"));
        }
        if self.spent {
            return Err(invariant("retained child clone cursor is spent"));
        }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(progress) = source.bind(&mut self.source, grant)? {
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if self.draining {
            return match self.phase {
                0 => drain_field(&mut self.child_id_cursor, &mut self.draining, &mut self.phase, grant),
                1 => drain_field(&mut self.target_cursor, &mut self.draining, &mut self.phase, grant),
                _ => Err(invariant("retained child drain state is invalid")),
            };
        }
        match self.phase {
            0 => advance_field(&mut self.child_id_cursor, &mut self.child_id_value, &mut self.draining, source.project(1, |source| &source.child_id), grant),
            1 => advance_field(&mut self.target_cursor, &mut self.target_value, &mut self.draining, source.project(2, |source| &source.target), grant),
            2 => {
                let present = source.get().local_owner.is_some();
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: if present { size_of::<ArtifactChildLocalOwnerAlias>() } else { 0 }, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                self.local_owner_value = source.get().local_owner.as_ref().map(|owner| ArtifactChildLocalOwnerAlias(Arc::clone(owner)));
                self.phase += 1;
                Ok(RetainedCloneStep::Progress(progress))
            }
            3 => {
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                self.output = Some(ArtifactChild {
                    child_id: self.child_id_value.take().expect("retained field owner"),
                    target: self.target_value.take().expect("retained field owner"),
                    local_owner: self.local_owner_value.take().map(|alias| alias.0),
                    _snapshot: PhantomData,
                });
                self.phase += 1;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            _ => Err(invariant("retained child clone state is invalid")),
        }
    }

    fn take(&mut self) -> Option<ArtifactChild<S>> {
        let output = self.output.take();
        if output.is_some() {
            self.spent = true;
        }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if !self.closing {
            return Err(invariant("retained child must begin close before granted retirement"));
        }
        if let Some(step) = close_field(&mut self.child_id_cursor, grant)? {
            return Ok(step);
        }
        if let Some(step) = close_field(&mut self.target_cursor, grant)? {
            return Ok(step);
        }
        if !self.close.is_empty() {
            return self.close.step_granted(grant);
        }
        if let Some(step) = self.close.begin_granted(&mut self.child_id_value, grant)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_granted(&mut self.target_value, grant)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_granted(&mut self.local_owner_value, grant)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? {
            return Ok(step);
        }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        if !self.closing {
            return Ok(0);
        }
        if !self.child_id_cursor.terminal_is_empty() {
            return self.child_id_cursor.next_close_depth_demand();
        }
        if !self.target_cursor.terminal_is_empty() {
            return self.target_cursor.next_close_depth_demand();
        }
        self.close.next_owner_depth_with_binding(self.output.is_some() || self.child_id_value.is_some() || self.target_value.is_some() || self.local_owner_value.is_some(), &self.source)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing {
            return Ok(0);
        }
        if !self.child_id_cursor.terminal_is_empty() {
            return self.child_id_cursor.next_close_copy_byte_demand();
        }
        if !self.target_cursor.terminal_is_empty() {
            return self.target_cursor.next_close_copy_byte_demand();
        }
        self.close.next_copy_with_binding(&self.source)
    }

    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, ValueError> {
        if !self.closing {
            return Ok(0);
        }
        if !self.child_id_cursor.terminal_is_empty() {
            return self.child_id_cursor.next_close_capacity_byte_demand(maximum_release_bytes);
        }
        if !self.target_cursor.terminal_is_empty() {
            return self.target_cursor.next_close_capacity_byte_demand(maximum_release_bytes);
        }
        if !self.close.is_empty() {
            return self.close.next_capacity_byte_demand(maximum_release_bytes);
        }
        if self.child_id_value.is_some() {
            return self.close.next_owner_capacity_with_binding::<String>(true, maximum_release_bytes, &self.source);
        }
        if self.target_value.is_some() {
            return self.close.next_owner_capacity_with_binding::<ArtifactRef>(true, maximum_release_bytes, &self.source);
        }
        if self.local_owner_value.is_some() {
            return self.close.next_owner_capacity_with_binding::<ArtifactChildLocalOwnerAlias>(true, maximum_release_bytes, &self.source);
        }
        self.close.next_owner_capacity_with_binding::<ArtifactChild<S>>(self.output.is_some(), maximum_release_bytes, &self.source)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.closing {
            return Ok(0);
        }
        if !self.child_id_cursor.terminal_is_empty() {
            return self.child_id_cursor.next_close_release_byte_demand();
        }
        if !self.target_cursor.terminal_is_empty() {
            return self.target_cursor.next_close_release_byte_demand();
        }
        self.close.next_release_with_binding(&self.source)
    }

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
