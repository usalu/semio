//! ✉️ Edit decoding phases: the envelope admission, cursor building and the forward, inverse and metadata decoding of the hydration step. See [`super`].

use super::*;

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: ArtifactPack + semio_os_store_text_io::ArtifactStoreSnapshot + semio_os_spr::StoreState,
    M: semio_os_spr::StoreMutationFor<P>+protocol::OpBinaryReceiving,
{
    /// 🛂 Admits one edit envelope.
    pub(super) fn step_admit_envelope(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        let child = match hydration_child_grant(grant, cx.fuel_remaining()) { Ok(child) => child, Err(diagnostic) => return self.reject(diagnostic) };
        let envelope = self.envelope.as_mut().expect("hydration retains its original partially admitted envelope");
        let progress = match envelope.admit_document_next(child) { Ok(progress) => progress, Err(_) => return self.reject(MemberOpenDiagnostic::Initialization) };
        if !progress.fits(child) { return self.reject(MemberOpenDiagnostic::Initialization); }
        let ready = envelope.document_admission_is_complete();
        if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
        if ready { self.phase = Phase::BuildAppliedCursor; }
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🧭 Builds the applied or redo cursor.
    pub(super) fn step_build_applied_cursor(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        if matches!(self.target,PersistedDocumentHydrationTarget::Envelope){return match self.advance_envelope_cursor(cx){Ok(())=>PersistedDocumentHydrationStep::Pending(self.progress()),Err(error)=>{*self.original_failure=Some(error);self.reject(MemberOpenDiagnostic::Replay)}}}
        let fold = self.fold.as_mut().expect("derived history remains retained");
        let applied = self.phase == Phase::BuildAppliedCursor;
        let ids = if applied { &mut fold.applied } else { &mut fold.redo };
        if let Some(id) = ids.get_mut(self.operation_index) {
            let cursor = self.envelope.as_mut().expect("hydrated envelope remains retained").cursor.as_mut().expect("hydrated cursor remains retained");
            if applied { cursor.applied_edit_ids.push(std::mem::take(id)); } else { cursor.redo_edit_ids.push(std::mem::take(id)); }
            self.operation_index += 1;
        } else {
            self.operation_index = 0;
            self.phase = if applied { Phase::BuildRedoCursor } else { Phase::BeginEdit };
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    fn advance_envelope_cursor(&mut self,cx:&mut StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if !self.bind_original_step(cx)?{return Ok(())}
        let applied=self.phase==Phase::BuildAppliedCursor;let fold=self.fold.as_mut().expect("original fold remains");
        let ids=if applied{&mut fold.applied}else{&mut fold.redo};let grant=cx.retained_grant();
        let Some(id)=ids.get_mut(self.operation_index)else{
            let bytes=std::mem::size_of::<usize>()+std::mem::size_of::<Phase>();if cx.should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(())}
            self.operation_index=0;self.phase=if applied{Phase::BuildRedoCursor}else{Phase::BeginEdit};cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;return Ok(())
        };
        let cursor=self.envelope.as_mut().expect("original envelope remains").cursor.as_mut().expect("original cursor remains");
        let target=if applied{&mut cursor.applied_edit_ids}else{&mut cursor.redo_edit_ids};let(demand,work)=target.retained_push_demand()?;
        let extra=std::mem::size_of::<String>()+std::mem::size_of::<Option<String>>()+std::mem::size_of::<usize>();let bytes=demand.copy_bytes.checked_add(extra).ok_or_else(||ValueError::invariant("original cursor transfer extent overflow"))?;let work=work.checked_add(1).ok_or_else(||ValueError::invariant("original cursor transfer work overflow"))?;
        if cx.should_yield()||cx.fuel_remaining()<work||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<=demand.depth{return Ok(())}
        let mut input=Some(std::mem::take(id));let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-extra,maximum_depth:grant.maximum_depth-1,..grant};let turn=target.push_retained(&mut input,child,cx.fuel_remaining()-1);
        if let Some(value)=input{*id=value}else{self.operation_index+=1}
        let performed=turn.work+1;cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
        let (mut progress,failure)=match turn.result{Ok(progress)=>(progress,None),Err(error)=>(error.retained_progress(),Some(error))};
        progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.checked_add(extra).ok_or_else(||ValueError::invariant("original cursor receipt extent overflow"))?;let receipt=cx.consume_retained(progress);
        if let Some(error)=failure{let mut failure=semio_framework_job::OriginalTurnFailure::from(error);failure.effects=progress;failure.receipt_refusal=receipt.err();return Err(failure)}receipt.map_err(Into::into)
    }

    /// ✏ Begins one edit.
    pub(super) fn step_begin_edit(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        if matches!(self.target,PersistedDocumentHydrationTarget::Envelope){return self.step_begin_envelope_edit(cx)}
        let history = self.history.as_ref().expect("decoded history remains retained");
        let Some(source) = history.edits.get(self.edit_index) else {
            self.operation_index = 0;
            self.phase = Phase::HydrateChanges;
            cx.consume_fuel(1);
            return PersistedDocumentHydrationStep::Pending(self.progress());
        };
        if source.actor.as_deref().is_none_or(|actor| actor.trim().is_empty()) { return self.reject(MemberOpenDiagnostic::Replay); }
        let mut forwards = Vec::new();
        let mut inverse = Vec::new();
        let mut mutation_meta = Vec::new();
        if forwards.try_reserve_exact(source.ops.len()).is_err() || inverse.try_reserve_exact(source.inverse.len()).is_err() || mutation_meta.try_reserve_exact(source.meta.as_ref().map_or(0, Vec::len)).is_err() {
            return self.reject(MemberOpenDiagnostic::Capacity);
        }
        let edit = Edit {
            line: source.line.clone(),
            id: source.id.clone(),
            actor: source.actor.clone(),
            forwards,
            inverse: inverse.into(),
            mutation_meta,
            verb: source.verb.clone(),
            sequence_number: self.edit_index as i32 + 1,
            started_at: source.started_at.clone(),
            finished_at: source.finished_at.clone(),
        };
        let runtime = self.runtime.as_mut().expect("hydration runtime remains retained");
        if runtime.seed_mutation(semio_os_spr::MutationId(edit.id.clone())).is_err() {
            *self.pending_edit = Some(edit);
            return self.reject(MemberOpenDiagnostic::Replay);
        }
        runtime.observe_sequence(self.edit_index as i32 + 1);
        *self.pending_messages = Some(semio_os_spr::EditMessages { edit_id: edit.id.clone(), messages: Vec::new() });
        *self.pending_edit = Some(edit);
        self.operation_index = 0;
        self.phase = Phase::DecodeForward;
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// \u{1f4e5}\uFE0F Envelope assembly donates uniquely held edit fields and admits its real typed destinations before decoding.
    fn step_begin_envelope_edit(&mut self,cx:&mut StepContext<'_>)->PersistedDocumentHydrationStep<P,M>{
        if let Err(error)=(||->Result<(),semio_framework_job::OriginalTurnFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;Ok(())})(){*self.original_context_failure=Some(error);return self.reject(MemberOpenDiagnostic::Initialization)}
        if self.original_failure.is_some()||self.original_context_failure.is_some(){return self.reject(MemberOpenDiagnostic::Replay)}
        if self.edit_index>=self.history.as_ref().expect("original History remains").edits.len(){self.operation_index=0;self.phase=Phase::HydrateChanges;cx.consume_fuel(1);return PersistedDocumentHydrationStep::Pending(self.progress())}
        if self.pending_edit.is_none(){
            let grant=cx.retained_grant();let bytes=std::mem::size_of::<Edit<M>>()+std::mem::size_of::<semio_os_spr::HistoryEdit>()+std::mem::size_of::<Option<semio_os_spr::EditMessages>>()+std::mem::size_of_val(&self.original_source);
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return PersistedDocumentHydrationStep::Pending(self.progress())}
            let Some(history)=std::sync::Arc::get_mut(self.history.as_mut().expect("original History remains"))else{*self.original_failure=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"envelope input still has a foreign History alias").into());return self.reject(MemberOpenDiagnostic::Replay)};
            let source=&mut history.edits[self.edit_index];
            if source.actor.as_deref().is_none_or(|actor|actor.trim().is_empty()){return self.reject(MemberOpenDiagnostic::Replay)}
            let sequence=match i32::try_from(self.edit_index).ok().and_then(|n|n.checked_add(1)){Some(n)=>n,None=>return self.reject(MemberOpenDiagnostic::Capacity)};
            self.original_source=Some(semio_framework_job::OriginalSourceBinding::borrow(cx));
            *self.pending_edit=Some(take_envelope_edit(source,sequence));self.semantic_index=0;
            *self.pending_messages=Some(semio_os_spr::EditMessages{edit_id:String::new(),messages:Vec::new()});
            if let Err(error)=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}){*self.original_context_failure=Some(error.into())}cx.consume_fuel(1);
            return PersistedDocumentHydrationStep::Pending(self.progress())
        }
        let source=&self.history.as_ref().expect("original History remains").edits[self.edit_index];let edit=self.pending_edit.as_mut().unwrap();let messages=self.pending_messages.as_mut().expect("message destination remains");
        let mut work=semio_framework_value_persistent::work::RetainedWorkLedger::new(cx.retained_grant());
        let result=(||->Result<bool,&'static str>{
            if !work.reserve_vec(&mut edit.forwards,source.ops.len(),&mut None)?{return Ok(false)}
            if !work.reserve_vec(&mut edit.mutation_meta,source.meta.as_ref().map_or(0,Vec::len),&mut None)?{return Ok(false)}
            if !work.reserve_string(&mut messages.edit_id,edit.id.len(),&mut None)?{return Ok(false)}
            if messages.edit_id.len()<edit.id.len(){
                let start=messages.edit_id.len();let available=work.remaining().maximum_copy_bytes.min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));let mut end=start.saturating_add(available).min(edit.id.len());while end>start&&!edit.id.is_char_boundary(end){end-=1}if end==start{return Ok(false)}
                if !work.string_push(&mut messages.edit_id,&edit.id[start..end])?{return Ok(false)}return Ok(false)
            }
            if !work.copy_bytes(std::mem::size_of::<Phase>()+std::mem::size_of::<usize>()){return Ok(false)}
            Ok(true)
        })();
        let progress=work.receipt();let consumed=work.actual_work()+u64::from(result==Ok(true));let receipt=cx.consume_retained(progress);cx.consume_fuel(consumed);
        if let Err(error)=result{let mut failure=semio_framework_job::OriginalTurnFailure::from(ValueError::literal(ValueRefusalKind::InvalidValue,error));failure.effects=progress;failure.receipt_refusal=receipt.err();*self.original_failure=Some(failure);return self.reject(MemberOpenDiagnostic::Capacity)}
        if let Err(error)=receipt{*self.original_context_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Capacity)}
        if result==Ok(true){self.operation_index=0;self.phase=Phase::DecodeForward}
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// ➡ Decodes the forward or inverse mutation.
    pub(super) fn step_decode_forward(&mut self,cx:&mut StepContext<'_>)->PersistedDocumentHydrationStep<P,M>{
        if self.original_failure.is_some()||self.original_context_failure.is_some(){return self.reject(MemberOpenDiagnostic::Replay)}
        let count={let edit=&self.history.as_ref().expect("original History remains").edits[self.edit_index];if self.phase==Phase::DecodeForward{edit.ops.len()}else{edit.inverse.len()}};
        if self.operation_index==count{self.operation_index=0;self.phase=if self.phase==Phase::DecodeForward{Phase::DecodeInverse}else{Phase::DecodeMetadata};cx.consume_fuel(1);return PersistedDocumentHydrationStep::Pending(self.progress())}
        if !self.step_native_mutation(cx){return if self.original_failure.is_some()||self.original_context_failure.is_some(){self.reject(MemberOpenDiagnostic::Replay)}else{PersistedDocumentHydrationStep::Pending(self.progress())}}
        let inverse=self.phase==Phase::DecodeInverse;let edit=self.pending_edit.as_mut().expect("the typed edit remains owned");
        if inverse&&!edit.inverse.has_reserved_slot(){
            let grant=cx.retained_grant();let copy=match edit.inverse.next_reserve_copy_byte_demand(){Ok(value)=>value,Err(error)=>{*self.original_failure=Some(ValueError::from(error).into());return self.reject(MemberOpenDiagnostic::Capacity)}};
            let capacity=match edit.inverse.next_allocation_bytes(){Ok(value)=>value,Err(error)=>{*self.original_failure=Some(ValueError::from(error).into());return self.reject(MemberOpenDiagnostic::Capacity)}};
            let depth=match edit.inverse.next_reserve_depth_demand(){Ok(value)=>value,Err(error)=>{*self.original_failure=Some(ValueError::from(error).into());return self.reject(MemberOpenDiagnostic::Capacity)}};
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<capacity||grant.maximum_depth<depth{return PersistedDocumentHydrationStep::Pending(self.progress())}
            let Some((turn,performed))=(match cx.with_original_effect(grant,|_,effect|edit.inverse.reserve_one_funded(effect)){Ok(value)=>value,Err(error)=>{*self.original_context_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Capacity)}}) else{return PersistedDocumentHydrationStep::Pending(self.progress())};
            let spent=u64::try_from(performed).unwrap_or(u64::MAX);
            match turn.result{
                Ok(semio_framework_value_persistent::list::PagedListReservationOutcome::Ready(progress))=>{if let Err(error)=cx.consume_retained(progress){*self.original_context_failure=Some(error.into())}cx.consume_fuel(spent)},
                Ok(semio_framework_value_persistent::list::PagedListReservationOutcome::Denied)=>cx.consume_fuel(spent),
                Err(failure)=>{let(error,copied_bytes)=match failure{semio_framework_value_persistent::list::PagedListReservationFailure::Work(error)=>(error,0),semio_framework_value_persistent::list::PagedListReservationFailure::Allocation{copied_bytes,..}=>(ValueError::literal(ValueRefusalKind::AllocationFailed,"retained edit inverse allocation refused"),copied_bytes)};let progress=RetainedCloneProgress{copied_items:1,copied_bytes:copied_bytes.max(copy),..Default::default()};let receipt=cx.consume_retained(progress);cx.consume_fuel(spent);let mut cause=semio_framework_job::OriginalTurnFailure::from(error);cause.effects=progress;cause.receipt_refusal=receipt.err();*self.original_failure=Some(cause)}
            }return PersistedDocumentHydrationStep::Pending(self.progress())
        }
        let bytes=std::mem::size_of::<M>()+std::mem::size_of::<Option<M>>()+std::mem::size_of::<usize>();let grant=cx.retained_grant();
        if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return PersistedDocumentHydrationStep::Pending(self.progress())}
        if !inverse&&edit.forwards.len()==edit.forwards.capacity(){*self.original_failure=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed forward destination lost its admitted slot").into());return self.reject(MemberOpenDiagnostic::Capacity)}
        let operation=self.mutation_output.value.take().expect("the exact original mutation is received");
        if inverse{if let Err(operation)=edit.inverse.push_reserved(operation){*self.mutation_output.value=Some(operation);*self.original_failure=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed inverse destination lost its admitted slot").into());return self.reject(MemberOpenDiagnostic::Capacity)}}else{edit.forwards.push(operation)}
        self.operation_index+=1;if let Err(error)=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}){*self.original_context_failure=Some(error.into())}cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🏷 Decodes the edit metadata.
    pub(super) fn step_decode_metadata(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        if matches!(self.target,PersistedDocumentHydrationTarget::Envelope){return match self.advance_envelope_metadata(cx){Ok(())=>PersistedDocumentHydrationStep::Pending(self.progress()),Err(error)=>{*self.original_failure=Some(error);self.reject(MemberOpenDiagnostic::Replay)}}}
        let source = &self.history.as_ref().expect("decoded history remains retained").edits[self.edit_index];
        let Some(metadata) = source.meta.as_ref() else { return self.reject(MemberOpenDiagnostic::Replay) };
        if metadata.len() != source.ops.len() {
            return self.reject(MemberOpenDiagnostic::Replay);
        }
        if let Some(meta) = metadata.get(self.operation_index) {
            if meta.author_id.as_deref().is_none_or(|actor| actor.trim().is_empty()) { return self.reject(MemberOpenDiagnostic::Replay); }
            let (meta, messages) = match mutation_meta_from_history_op_meta(meta.clone()) {
                Ok(decoded) => decoded,
                Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
            };
            let edit_id = self.pending_edit.as_ref().expect("pending typed edit remains retained").id.as_str();
            if let Some(id) = &meta.mutation_id {
                self.mutation_lookup.as_mut().expect("hydrated mutation index remains retained").insert(id.clone(), (self.edit_index, self.operation_index));
                if self.runtime.as_mut().expect("hydration runtime remains retained").seed_edit_operation(edit_id, id.clone()).is_err() {
                    return self.reject(MemberOpenDiagnostic::Replay);
                }
            }
            self.runtime.as_mut().expect("hydration runtime remains retained").observe_timestamp(meta.timestamp);
            self.pending_edit.as_mut().expect("pending typed edit remains retained").mutation_meta.push(meta);
            self.pending_messages.as_mut().expect("pending message owner remains retained").messages.extend(messages);
            self.operation_index += 1;
        } else {
            self.operation_index = 0;
            self.phase = Phase::FinishEdit;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// ✔ Finishes one edit.
    pub(super) fn step_finish_edit(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let envelope_only=matches!(self.target,PersistedDocumentHydrationTarget::Envelope);
        if envelope_only {
            if self.pending_edit.is_some() {
                match self.advance_envelope_semantics(cx) {
                    Ok(true) => {},
                    Ok(false) => return PersistedDocumentHydrationStep::Pending(self.progress()),
                    Err(error) => { *self.original_failure=Some(error); return self.reject(MemberOpenDiagnostic::Replay); }
                }
                if cx.should_yield() || cx.fuel_remaining()==0 { return PersistedDocumentHydrationStep::Pending(self.progress()); }
                if let Err(error)=cx.original_source_parent().and_then(|parent|parent.validate_borrowed_context(cx)) {
                    *self.original_failure=Some(error.into()); return self.reject(MemberOpenDiagnostic::Replay);
                }
                let turn=self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.edits.admit_owned_row(&mut self.pending_edit,cx.retained_grant());
                let progress=match &turn.result { Ok(progress)=>*progress,Err(error)=>error.retained_progress() };
                let receipt=cx.consume_retained(progress); cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX));
                if let Err(error)=turn.result {
                    let mut failure=semio_framework_job::OriginalTurnFailure::from(error);
                    failure.effects=progress; failure.receipt_refusal=receipt.err();
                    *self.original_failure=Some(failure); return self.reject(MemberOpenDiagnostic::Capacity);
                }
                if let Err(error)=receipt { *self.original_failure=Some(error.into()); return self.reject(MemberOpenDiagnostic::Capacity); }
                return PersistedDocumentHydrationStep::Pending(self.progress());
            }
        } else {
            let mut edit=self.pending_edit.take().expect("completed typed edit remains retained");
            let envelope=self.envelope.as_mut().expect("hydrated envelope remains retained");
            semio_os_store_ledger::stamp_edit_semantics::<P,M>(&mut edit,&envelope.schema);
            self.edit_lookup.as_mut().expect("hydrated edit index remains retained").insert(edit.id.clone(),envelope.vcs.edits.len());
            if let Err(edit)=envelope.vcs.edits.try_push(edit) {
                *self.pending_edit=Some(edit); return self.reject(MemberOpenDiagnostic::Capacity);
            }
        }
        if envelope_only {
            if cx.should_yield()||cx.fuel_remaining()==0{return PersistedDocumentHydrationStep::Pending(self.progress())}
            if let Err(error)=cx.original_source_parent().and_then(|parent|parent.validate_borrowed_context(cx)){
                *self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Replay);
            }
            if !self.empty_messages.terminal_is_empty(){
                let demand=match self.empty_messages.close_demands(cx.retained_grant().maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>{*self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Capacity)}};
                if let Err(error)=cx.with_original_retirement(|cx|semio_framework_job::close_original_effect_step(cx,demand,|effect|{let poll=self.empty_messages.close_step(effect);match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}),semio_framework_value::RetainedWorkPoll::Pending{work}=>semio_framework_value::RetainedWorkPoll::Pending{work}}})){
                    *self.original_failure=Some(error);return self.reject(MemberOpenDiagnostic::Capacity);
                }
                return PersistedDocumentHydrationStep::Pending(self.progress());
            }
            if self.pending_messages.as_ref().is_some_and(|messages|messages.messages.is_empty()){
                let bytes=2*std::mem::size_of::<Option<semio_os_spr::EditMessages>>();let grant=cx.retained_grant();
                if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return PersistedDocumentHydrationStep::Pending(self.progress())}
                *self.empty_messages.value=self.pending_messages.take();
                let receipt=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()});cx.consume_fuel(1);
                if let Err(error)=receipt{*self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Capacity)}
                return PersistedDocumentHydrationStep::Pending(self.progress());
            }
            let ledger=&mut self.envelope.as_mut().expect("hydrated envelope remains retained").edit_messages;
            if self.pending_messages.is_some()||ledger.original_append_pending(){
                let turn=ledger.append_original_step(&mut self.pending_messages,cx.retained_grant(),cx.fuel_remaining());
                let progress=match &turn.result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
                let receipt=cx.consume_retained(progress);cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX));
                if let Err(error)=turn.result{
                    let mut failure=semio_framework_job::OriginalTurnFailure::from(error);
                    failure.effects=progress;failure.receipt_refusal=receipt.err();
                    *self.original_failure=Some(failure);return self.reject(MemberOpenDiagnostic::Capacity);
                }
                if let Err(error)=receipt{*self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Capacity)}
                return PersistedDocumentHydrationStep::Pending(self.progress());
            }
            let bytes=2*std::mem::size_of::<usize>()+std::mem::size_of::<Phase>();let grant=cx.retained_grant();
            if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return PersistedDocumentHydrationStep::Pending(self.progress())}
            self.edit_index+=1;self.record_index+=1;self.phase=Phase::BeginEdit;
            let receipt=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()});cx.consume_fuel(1);
            if let Err(error)=receipt{*self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Capacity)}
            return PersistedDocumentHydrationStep::Pending(self.progress());
        }
        let messages = self.pending_messages.take().expect("completed message owner remains retained");
        if !messages.messages.is_empty() {
            if let Err(messages) = self.envelope.as_mut().expect("hydrated envelope remains retained").edit_messages.admit(messages) {
                *self.active = Some(Box::new(semio_os_spr::command::MutationMessageLedgerRetirement::new(messages.edit_id, messages.messages)));
                return self.reject(MemberOpenDiagnostic::Capacity);
            }
        }
        self.edit_index += 1;
        self.record_index += 1;
        self.phase = Phase::BeginEdit;
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }
}

fn take_envelope_edit<M>(source:&mut semio_os_spr::HistoryEdit,sequence:i32)->Edit<M>{Edit{line:source.line.take(),id:std::mem::take(&mut source.id),actor:source.actor.take(),forwards:Vec::new(),inverse:Vec::new().into(),mutation_meta:Vec::new(),verb:source.verb.take(),sequence_number:sequence,started_at:std::mem::take(&mut source.started_at),finished_at:source.finished_at.take()}}

#[cfg(test)]
mod original_edit_donor_laws{
 use super::*;
 #[test]
 fn typed_edit_moves_the_actual_strings_and_preserves_the_operation_donor(){
  let law:serde_json::Value=serde_json::from_str(include_str!("donor.json")).unwrap();
  for row in law["cases"].as_array().unwrap(){let optional=|name:&str|row[name].as_str().map(str::to_owned);let mut source=semio_os_spr::HistoryEdit{id:row["id"].as_str().unwrap().into(),actor:Some("author".into()),started_at:row["startedAt"].as_str().unwrap().into(),line:optional("line"),verb:optional("verb"),finished_at:optional("finishedAt"),ops:vec![semio_os_spr::OpPayload{binary:Some(vec![7]),text:None}],inverse:Vec::new(),meta:None,lane:None};let id=source.id.as_ptr();let started=source.started_at.as_ptr();let operations=source.ops.as_ptr();let actor=source.actor.as_deref().unwrap().as_ptr();let edit:Edit<()>=take_envelope_edit(&mut source,row["sequence"].as_i64().unwrap()as i32);assert_eq!(edit.id,row["id"].as_str().unwrap());assert_eq!(edit.id.as_ptr(),id);assert_eq!(edit.started_at.as_ptr(),started);assert_eq!(edit.actor.as_deref().unwrap().as_ptr(),actor);assert!(source.id.is_empty()&&source.started_at.is_empty()&&source.actor.is_none()&&source.line.is_none()&&source.verb.is_none()&&source.finished_at.is_none());assert_eq!(source.ops.as_ptr(),operations);assert_eq!(source.ops[0].binary.as_deref(),Some([7].as_slice()));assert!(edit.forwards.is_empty()&&edit.inverse.is_empty()&&edit.mutation_meta.is_empty());}
 }
}
