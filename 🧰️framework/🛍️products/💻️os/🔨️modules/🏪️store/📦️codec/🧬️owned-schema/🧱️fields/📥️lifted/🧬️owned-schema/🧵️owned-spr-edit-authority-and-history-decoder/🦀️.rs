//! 🧵️ Owned SPR edit authority and history decoder: the edit authority operations, the history entry authority, the edit decoder and the repository history entry decoder. See [`super`].

use super::*;

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactOwnedSprEditAuthority<P, Mutation> {
    pub fn new(
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
        mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
    ) -> Self {
        Self {
            operation,
            generation,
            path,
            catalog: std::mem::ManuallyDrop::new(Some(catalog)),
            mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)),
            retirement_factory: std::mem::ManuallyDrop::new(Some(retirement_factory)),
            cursor: OwnedSchemaNestedRecordCursor::try_new(OwnedSchemaRecordSpec { fields: ARTIFACT_OWNED_SPR_EDIT_FIELDS }).expect("SPR edit schema is a validated static catalog"),
            pending: None,
            active: std::mem::ManuallyDrop::new(None),
            completed_arrays: std::array::from_fn(|_| std::mem::ManuallyDrop::new(None)),
            factories_close: std::array::from_fn(|_| None),
            strings: std::array::from_fn(|_| std::mem::ManuallyDrop::new(None)),
            actor: std::mem::ManuallyDrop::new(None),
            actor_close: None,
            forwards: std::mem::ManuallyDrop::new(None),
            inverse: std::mem::ManuallyDrop::new(None),
            sequence_number: None,
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            close_string: 0,
            terminal: false,
        }
    }

    pub fn diagnostic(&self, code: &'static str, offset: u64) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    /// 🌱️ Installs the exact owner for one edit field, or finishes a scalar field inline. Returns
    /// `Some(step)` when the field needed no owner at all.
    fn begin_edit_field(&mut self, field_id: u16, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor) -> Result<Option<ArtifactEnvelopeFieldDecodeStep>, OwnedSchemaDecodeDiagnostic> {
        if Self::string_index(field_id).is_some() {
            if token.kind == OwnedSchemaTokenKind::Null && matches!(field_id, 2 | 10 | 11 | 12) {
                return Ok(Some(ArtifactEnvelopeFieldDecodeStep::TokenComplete));
            }
            if !terminal {
                return Err(self.diagnostic("artifact-spr.edit-string-scalar", token.start));
            }
            let authority = OwnedSchemaStringAuthority::try_new(self.operation, self.generation, token, self.path).map_err(|token| self.diagnostic("artifact-spr.edit-string", token.start))?;
            *self.active = Some(ArtifactOwnedSprEditActive::String { field_id, authority });
            return Ok(None);
        }
        if matches!(field_id, 3 | 4) {
            let authority = ArtifactOwnedSprMutationArrayAuthority::try_new(self.operation, self.generation, self.path, self.catalog.as_ref().expect("live original edit catalog").clone(), self.mutation_factory.as_ref().expect("live original edit mutation factory").clone(), field_id == 4)?;
            *self.active = Some(ArtifactOwnedSprEditActive::Mutations { field_id, authority });
            return Ok(None);
        }
        if field_id == 5 {
            *self.active = Some(ArtifactOwnedSprEditActive::EmptyMetadata(OwnedSchemaEmptyArrayAuthority::new(self.path)));
            return Ok(None);
        }
        if field_id == 8 && terminal && token.kind == OwnedSchemaTokenKind::Number {
            let mut bytes = [0u8; 64];
            let len = usize::try_from(token.end.saturating_sub(token.start)).unwrap_or(usize::MAX);
            if len == 0 || len > bytes.len() || source.copy_token_bytes(token, 0, &mut bytes[..len]) != len {
                return Err(self.diagnostic("artifact-spr.edit-sequence-token", token.start));
            }
            self.sequence_number = std::str::from_utf8(&bytes[..len]).ok().and_then(|value| value.parse().ok());
            if self.sequence_number.is_none() {
                return Err(self.diagnostic("artifact-spr.edit-sequence-value", token.start));
            }
            return Ok(Some(ArtifactEnvelopeFieldDecodeStep::TokenComplete));
        }
        Err(self.diagnostic("artifact-spr.edit-field", token.start))
    }

    /// 🎯️ Routes one cursor-attributed token into the live edit-field owner.
    fn accept_active_edit_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let Some(active) = self.active.as_mut() else {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-field", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        };
        match active {
            ArtifactOwnedSprEditActive::String { field_id, authority } => match authority.step(source, cx) {
                OwnedSchemaStringStep::Pending => return Ok(ArtifactEnvelopeFieldDecodeStep::Pending),
                OwnedSchemaStringStep::Complete => {
                    let Some(index) = Self::string_index(*field_id) else {
                        return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-string-field", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
                    };
                    if index==1 {
                        let grant=cx.retained_grant();
                        if self.strings[1].is_none(){
                            let original=authority.as_str().ok_or(OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-text",offset:token.start,line:0,column:0,path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                            let copied_bytes=original.len().checked_add(size_of::<Option<String>>()).ok_or(OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-overflow",offset:token.start,line:0,column:0,path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                            if grant.maximum_items==0||grant.maximum_depth==0||copied_bytes>grant.maximum_copy_bytes||original.len()>grant.maximum_capacity_bytes{return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);}
                            let mut value=String::new();value.try_reserve_exact(original.len()).map_err(|_|OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-allocation",offset:token.start,line:0,column:0,path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;value.push_str(original);
                            let receipt=RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes:value.capacity(),released_bytes:0};
                            *self.strings[1]=Some(value);cx.consume_retained(receipt).map_err(|error|OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-receipt",offset:token.start,line:0,column:0,path,refusal_kind:error.kind,retained_progress:receipt})?;
                            return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);
                        }
                        let placement=size_of::<Option<semio_framework_value::SharedUtf8>>();
                        if grant.maximum_copy_bytes<placement{return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);}
                        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-placement,..grant};
                        match semio_framework_value::SharedUtf8::admit(self.strings[1].take().expect("original actor text retained"),child){
                            Ok((original,mut receipt))=>{receipt.copied_bytes+=placement;*self.actor=Some(original);cx.consume_retained(receipt).map_err(|error|OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-receipt",offset:token.start,line:0,column:0,path,refusal_kind:error.kind,retained_progress:receipt})?;}
                            Err((_,original))=>{*self.strings[1]=Some(original);return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);}
                        }
                    }else{*self.strings[index] = authority.take_string();}
                }
                OwnedSchemaStringStep::Cancelled => return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-string-cancelled", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }),
                OwnedSchemaStringStep::Fault(diagnostic) => return Err(diagnostic),
            },
            ArtifactOwnedSprEditActive::Mutations { field_id, authority } => match authority.accept(token, terminal, source, cx) {
                Err(diagnostic) => return Err(diagnostic),
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {
                    let index = match *field_id { 3 => 0, 4 => 1, _ => return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-native-array-owner", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }) };
                    if self.completed_arrays[index].is_some() { return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-array-custody", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }); }
                    let Some(values) = authority.take_values() else {
                        return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-mutation-values", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
                    };
                    match (*field_id, values) {
                        (3, ArtifactStoreOperationRowOwners::Forwards(values)) => *self.forwards = Some(values),
                        (4, ArtifactStoreOperationRowOwners::Inverse(values)) => *self.inverse = Some(values),
                        (_, values) => {
                            *authority.values = Some(values);
                            authority.taken = false;
                            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-native-array-owner", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
                        }
                    }
                }
                Ok(step) => return Ok(step),
            },
            ArtifactOwnedSprEditActive::EmptyMetadata(authority) => match authority.accept(token, terminal) {
                Err(diagnostic) => return Err(diagnostic),
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {}
                Ok(step) => return Ok(step),
            },
        }
        if let Some(ArtifactOwnedSprEditActive::Mutations { field_id, authority }) = self.active.take() {
            *self.completed_arrays[usize::from(field_id == 4)] = Some(authority);
        }
        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
    }

    pub fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let diagnostic = self.diagnostic("artifact-spr.edit-close-demand", 0);
        if let Some(active) = self.active.as_ref() {
            let mut demand = match active {
                ArtifactOwnedSprEditActive::Mutations { authority, .. } => authority.close_demands(copy)?,
                _ => RetirementDemand::default(),
            };
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        if let Some(owner) = self.completed_arrays.iter().find_map(|slot| slot.as_ref()) {
            let mut demand = owner.close_demands(copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        if let Some(owner) = self.retirement.as_ref() { return artifact_retirement_box_demands(owner, copy).map_err(|_| diagnostic); }
        if let Some(value) = self.value.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.retirement_factory.as_ref().expect("edit retains value factory").retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if self.inverse.is_some() || self.forwards.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<ArtifactStoreOperationRowsRetirement<Mutation>>(), depth: 2, ..Default::default() }); }
        if let Some(owner)=self.actor_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of_val(&self.actor_close),depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand().map_err(|error|diagnostic.with_native(error))?,capacity_bytes:owner.next_capacity_byte_demand(copy).map_err(|error|diagnostic.with_native(error))?,release_bytes:owner.next_release_byte_demand().map_err(|error|diagnostic.with_native(error))?,depth:owner.next_depth_demand().map_err(|error|diagnostic.with_native(error))?.checked_add(1).ok_or(diagnostic)?});}
        if self.actor.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_value::SharedUtf8>()+size_of_val(&self.actor_close),depth:1,..Default::default()});}
        if let Some(value) = self.strings[self.close_string..].iter().find_map(|slot| slot.as_ref()) { return Ok(RetirementDemand { release_bytes: value.capacity(), depth: 1, ..Default::default() }); }
        if self.retirement_factory.is_some() || self.mutation_factory.is_some() || self.catalog.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factories_close.iter().flatten().next() {
            let mut demand = owner.demands(copy).map_err(|_| diagnostic)?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        Ok(Default::default())
    }

    fn string_index(field_id: u16) -> Option<usize> {
        match field_id {
            1 => Some(0),
            2 => Some(1),
            9 => Some(2),
            10 => Some(3),
            11 => Some(4),
            12 => Some(5),
            _ => None,
        }
    }

    pub fn finish_record(&mut self) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.strings[1].is_some(){return Err(self.diagnostic("artifact-spr.edit-actor-unpublished-text",0));}
        let id = self.strings[0].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-id-missing", 0))?;
        let forwards = self.forwards.take().ok_or_else(|| self.diagnostic("artifact-spr.edit-forwards-missing", 0))?;
        let inverse = self.inverse.take().ok_or_else(|| self.diagnostic("artifact-spr.edit-inverse-missing", 0))?;
        let started_at = self.strings[2].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-started-at-missing", 0))?;
        let sequence_number = self.sequence_number.ok_or_else(|| self.diagnostic("artifact-spr.edit-sequence-missing", 0))?;
        *self.value = Some(Edit {
            id,
            actor: self.actor.take(),
            line: self.strings[5].take(),
            forwards,
            inverse: inverse.into(),
            mutation_meta: Vec::new(),
            verb: self.strings[4].take(),
            sequence_number,
            started_at,
            finished_at: self.strings[3].take(),
        });
        Ok(())
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactOwnedHistoryEntryAuthority<Edit<Mutation>> for ArtifactOwnedSprEditAuthority<P, Mutation> {
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(copy).map(|demand| demand.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.depth) }
    fn accept_token(&mut self, token: OwnedSchemaToken, _terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("artifact-spr.edit-stale", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("artifact-spr.edit-cancelled", token.start));
        }
        let (token, terminal) = match self.pending {
            Some(pending) => pending,
            None => match self.cursor.accept(token, source) {
                OwnedSchemaNestedRecordStep::Pending => return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
                OwnedSchemaNestedRecordStep::FieldToken { field_id, token, terminal } => {
                    if self.active.is_none() {
                        if let Some(step) = self.begin_edit_field(field_id, token, terminal, source)? {
                            return Ok(step);
                        }
                    }
                    (token, terminal)
                }
                OwnedSchemaNestedRecordStep::Complete => {
                    self.finish_record()?;
                    return Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete);
                }
                OwnedSchemaNestedRecordStep::Fault(diagnostic) => return Err(diagnostic),
            },
        };
        self.pending = Some((token, terminal));
        let outcome = self.accept_active_edit_token(token, terminal, source, cx);
        if !matches!(outcome, Ok(ArtifactEnvelopeFieldDecodeStep::Pending)) {
            self.pending = None;
        }
        outcome
    }

    fn take_value(&mut self) -> Option<Edit<Mutation>> {
        let value = self.value.take()?;
        self.terminal = true;
        Some(value)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        let diagnostic = self.diagnostic("artifact-spr.edit-close-authority", 0);
        if grant.maximum_depth < demand.depth { return Err(diagnostic); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.pending = None;
        self.terminal = true;
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(active) = self.active.as_mut() {
            let (step, terminal) = match active {
                ArtifactOwnedSprEditActive::String { authority, .. } => { authority.cancel(); (RetainedCloneStep::Complete(empty), authority.terminal_is_empty()) }
                ArtifactOwnedSprEditActive::Mutations { authority, .. } => { let step = authority.close_step(child)?; (step, authority.terminal_is_empty()) }
                ArtifactOwnedSprEditActive::EmptyMetadata(_) => (RetainedCloneStep::Complete(empty), true),
            };
            admit_retained_clone_close(child, step, terminal, "original edit active field").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if terminal { drop(self.active.take()); }
            return Ok(RetainedCloneStep::Progress(if terminal && step.progress().copied_items == 0 { RetainedCloneProgress { copied_items: 1, ..step.progress() } } else { step.progress() }));
        }
        if let Some(slot) = self.completed_arrays.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().expect("observed original completed mutation array");
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original edit completed field").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if terminal { drop(slot.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (diagnostic).with_native(error)); }
        if let Some(value) = self.value.take() {
            match self.retirement_factory.as_ref().expect("edit retains value factory").retire_owned(value, child) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(diagnostic.with_progress(progress)); } return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.value = Some(value); return Err(diagnostic.with_native(error)); }
            }
        }
        if let Some(values) = self.inverse.take() {
            let factory = self.mutation_factory.as_ref().expect("edit inverse retains factory").clone();
            match admit_artifact_retirement(values, grant, |values| ArtifactStoreOperationRowsRetirement::new(values, factory)) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, values)) => { *self.inverse = Some(values); return Err(diagnostic.with_native(error)); }
            }
        }
        if let Some(values) = self.forwards.take() {
            let factory = self.mutation_factory.as_ref().expect("edit forwards retains factory").clone();
            match admit_artifact_retirement(values, grant, |values| ArtifactStoreOperationRowsRetirement::new(values, factory)) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, values)) => { *self.forwards = Some(values); return Err(diagnostic.with_native(error)); }
            }
        }
        if let Some(owner)=self.actor_close.as_mut(){if owner.terminal_is_empty(){self.actor_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}let step=owner.step(child).map_err(|error| (diagnostic).with_native(error))?;admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original decoded actor lease").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(original)=self.actor.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>self.actor_close=Some(owner),Err((error,original))=>{*self.actor=Some(original);return Err(diagnostic.with_native(error));}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        while self.close_string < self.strings.len() {
            let index = self.close_string;
            self.close_string += 1;
            if self.strings[index].is_some() { drop(self.strings[index].take()); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty })); }
        }
        let factory = if let Some(factory) = self.retirement_factory.take() { Some((0, factory as Arc<dyn semio_framework_value::FactoryRetirement>)) }
            else if let Some(factory) = self.mutation_factory.take() { Some((1, factory as Arc<dyn semio_framework_value::FactoryRetirement>)) }
            else { self.catalog.take().map(|factory| (2, factory as Arc<dyn semio_framework_value::FactoryRetirement>)) };
        if let Some((index, factory)) = factory {
            self.factories_close[index] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(slot) = self.factories_close.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().expect("observed original edit factory");
            let step = owner.step(child).map_err(|error| (diagnostic).with_native(error))?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original edit factory").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if terminal { drop(slot.take()); }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.actor.is_none() && self.actor_close.is_none() && self.completed_arrays.iter().all(|slot| slot.is_none()) && self.factories_close.iter().all(Option::is_none) && self.catalog.is_none() && self.mutation_factory.is_none() && self.retirement_factory.is_none() && self.active.is_none() && self.forwards.is_none() && self.inverse.is_none() && self.value.is_none() && self.retirement.is_none() && self.strings.iter().all(|value| value.is_none())
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> Drop for ArtifactOwnedSprEditAuthority<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (ArtifactOwnedHistoryEntryAuthority::terminal_is_empty(self)), "SPR edit decode reached Drop before exact publication or bounded retirement");
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ArtifactOwnedSprEditDecoder<P, Mutation> {
    #[factory_child]
    pub catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
    #[factory_child]
    pub mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
}

impl<P: Send + Sync + 'static, Mutation: Send + Sync + 'static> ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>> for ArtifactOwnedSprEditDecoder<P, Mutation> {
    fn begin_entry(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
    ) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<Edit<Mutation>>> {
        Box::new(ArtifactOwnedSprEditAuthority::new(operation, generation, path, self.catalog.clone(), self.mutation_factory.clone(), retirement_factory))
    }
}

/// 📡️ Builds the repository-owned retained `.spr`/`.ops` edit decoder around an
/// exact domain catalog. Domain snapshots and mutations never cross this seam as serde owners.
pub fn artifact_owned_spr_edit_history_decoder<P: Send + Sync + 'static, Mutation: Send + Sync + 'static>(
    catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
    mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
) -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>>> {
    Arc::new(ArtifactOwnedSprEditDecoder { catalog, mutation_factory })
}

impl<T: FromValue + Send + Sync + 'static> ArtifactOwnedHistoryEntryDecoder<T> for ArtifactRepositoryHistoryEntryDecoder<T> {
    fn begin_entry(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,
    ) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<T>> {
        Box::new(ArtifactRepositoryHistoryEntryAuthority::new(operation, generation, path, retirement_factory))
    }
}

pub struct ArtifactRepositoryHistoryEntryAuthority<T> {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub path: OwnedSchemaPath,
    pub raw: std::mem::ManuallyDrop<Option<Box<[u8; ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES]>>>,
    pub(super) raw_len: usize,
    pub value: std::mem::ManuallyDrop<Option<T>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    pub factory_close: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    pub terminal: bool,
}

impl<T> ArtifactRepositoryHistoryEntryAuthority<T> {
    pub fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: OwnedSchemaPath, retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>) -> Self {
        Self { operation, generation, path, raw: std::mem::ManuallyDrop::new(Some(Box::new([0; ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES]))), raw_len: 0, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None), retirement_factory: std::mem::ManuallyDrop::new(Some(retirement_factory)), factory_close: std::mem::ManuallyDrop::new(None), terminal: false }
    }

    pub fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if let Some(owner) = self.retirement.as_ref() { return artifact_retirement_box_demands(owner, copy); }
        if let Some(value) = self.value.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.retirement_factory.as_ref().expect("history value retains its factory").retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if self.raw_len != 0 { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if self.raw.is_some() { return Ok(RetirementDemand { release_bytes: size_of::<[u8; ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES]>(), depth: 1, ..Default::default() }); }
        if self.retirement_factory.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factory_close.as_ref() {
            let mut demand = owner.demands(copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history decoder factory depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }

    pub fn diagnostic(&self, code: &'static str, offset: u64) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }
}
