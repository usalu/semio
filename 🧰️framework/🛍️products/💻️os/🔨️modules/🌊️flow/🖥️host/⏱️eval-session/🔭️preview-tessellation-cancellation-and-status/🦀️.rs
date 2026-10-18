//! 🔭️ Preview tessellation, cancellation and status of the evaluation session: pending tessellation, preview cancellation steps, the preview evaluation and chain status and the tessellate resolution. See [`super`].

use super::*;

impl FlowEvalSession {
    /// 📨 Notes an in-flight tessellate; returns true when the caller should emit `InvokeExtension`.
    pub fn note_pending_tessellate(&mut self, node_hash: u64, handle: String) -> bool {
        if self.preview_mesh_by_handle.contains_key(&handle){return false;}
        if self.preview_diagnostics_by_handle.contains_key(&handle) {
            return false;
        }
        if self.pending_tessellate_by_hash.values().any(|pending| pending == &handle) {
            return false;
        }
        self.tessellate_handle_by_hash.insert(node_hash, handle.clone());
        self.pending_tessellate_by_hash.insert(node_hash, handle);
        true
    }

    /// 📈 Aggregate progress of every tessellation this session has admitted and not yet finished —
    /// what the preview window's status object reports.
    pub fn preview_tessellate_status(&self) -> PreviewTessellateStatus {
        let mut status = PreviewTessellateStatus::default();
        status.in_flight = self.pending_tessellate_by_hash.len() as u32;
        status.diagnostics = self.preview_diagnostics_by_handle.len() as u32;
        for progress in self.tessellate_progress_by_hash.values() {
            status.units_done = status.units_done.saturating_add(progress.units_done);
            status.units_total = status.units_total.saturating_add(progress.units_total);
            status.faces_done = status.faces_done.saturating_add(progress.faces_done);
            status.faces_total = status.faces_total.saturating_add(progress.faces_total);
            if progress.phase!=PreviewTessellatePhase::Complete{status.phase=progress.phase;}
        }
        if status.units_total == 0 && status.in_flight > 0 {
            status.phase = PreviewTessellatePhase::SamplingEdges;
        }
        status
    }

    /// 🛑️ Records cancellation while retaining every original source until its admitted turn.
    pub fn cancel_preview_evaluation(&mut self,_window_id:&str,grant:RetainedCloneGrant)->Result<(usize,RetainedCloneProgress),ValueError>{
        if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original preview session is closing"))}
        if self.preview_cancelled{return Ok((0,Default::default()))}
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"preview cancellation requires an admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"preview cancellation requires admitted depth"))}
        let count=self.state.pending_tessellate_by_hash.len();
        self.state.preview_cancelled=true;self.state.tick_scheduled=false;self.state.pending_host_cancelled=self.state.pending_host.is_some();self.state.preview_cancellation_cursor=Some(0);self.state.preview_cancellation_phase=0;
        Ok((count,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }

    pub fn next_preview_cancellation_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.state.preview_cancellation_cursor.is_none(){return Ok(Default::default())}
        match self.state.preview_cancellation_phase{
            0=>self.retirement.admission_demands(),
            4=>self.geometry_port.as_ref().map_or(Ok(Default::default()),|port|Ok(RetirementDemand{copy_bytes:port.next_cancel_copy_byte_demand()?,capacity_bytes:port.next_cancel_capacity_byte_demand(copy)?,release_bytes:port.next_cancel_release_byte_demand()?,depth:port.next_cancel_depth_demand()?})),
            5=>self.retirement.demands(copy),
            _=>Ok(RetirementDemand{depth:1,..Default::default()}),
        }
    }

    /// ⏱️ Hands off exact sources, resets one actual row, or advances the original geometry cancellation.
    pub fn preview_cancellation_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.state.preview_cancellation_progress=Default::default();
        let Some(cursor)=self.state.preview_cancellation_cursor else{return Ok(RetainedCloneStep::Complete(Default::default()))};
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let demand=self.next_preview_cancellation_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"preview cancellation requires its original cursor depth"))}
        if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        match self.state.preview_cancellation_phase{
            0=>{
                self.retirement.preflight(grant)?;
                let original=SessionPreviewCancellationSources::take(&mut self.state);
                match self.retirement.admit(SessionCollectionOwner::Cancellation(original),grant){Ok(receipt)=>progress=receipt,Err((error,SessionCollectionOwner::Cancellation(original)))=>{original.restore(&mut self.state);return Err(error)},Err(_)=>unreachable!("original cancellation source changed")}
                self.state.preview_cancellation_phase=1;
            },
            1=>{
                if cursor<self.state.tessellate_progress_by_hash.slot_count(){if let Some((_,row))=self.state.tessellate_progress_by_hash.slot_entry_mut(cursor){if row.phase!=PreviewTessellatePhase::Complete{row.phase=PreviewTessellatePhase::Cancelled;}}self.state.preview_cancellation_cursor=Some(cursor+1);}
                else{self.state.preview_cancellation_phase=2;self.state.preview_cancellation_cursor=Some(0);}
            },
            2=>{
                if cursor<self.state.window_tick_latches.slot_count(){if let Some((_,row))=self.state.window_tick_latches.slot_entry_mut(cursor){*row=FlowEvalWindowTickLatch::default();}self.state.preview_cancellation_cursor=Some(cursor+1);}
                else{self.state.preview_cancellation_phase=3;}
            },
            3=>{if let Some(port)=self.geometry_port.as_mut(){port.begin_cancel()?;self.state.preview_cancellation_phase=4;}else{self.state.preview_cancellation_phase=5;}},
            4=>{
                let state=&mut *self.state;
                let port=state.geometry_port.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original cancelling geometry source disappeared"))?;
                let result=port.cancel_step(grant);progress=port.cancel_step_progress();state.preview_cancellation_progress=progress;let step=result.map_err(|error|error.with_retained_progress(progress))?;
                semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,port.cancel_terminal_is_empty(),"Flow session original geometry cancellation")?;
                if port.cancel_terminal_is_empty(){state.preview_cancellation_phase=5;}
            },
            5=>{let result=self.retirement.step(grant);progress=self.retirement.step_progress();self.state.preview_cancellation_progress=progress;let step=result?;progress=step.progress();if self.retirement.terminal_is_empty(){self.state.preview_cancellation_cursor=None;}},
            _=>unreachable!("original preview cancellation phase"),
        }
        self.state.preview_cancellation_progress=progress;
        Ok(if self.preview_cancellation_terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    pub fn preview_cancellation_step_progress(&self)->RetainedCloneProgress{self.state.preview_cancellation_progress}

    pub fn preview_cancellation_terminal_is_empty(&self)->bool{self.state.preview_cancellation_cursor.is_none()}

    /// 🛑 The `tessellateCancel` request body the cancelling command sends to the geometry extension.
    /// Deliberately addresses NO handle: the session keys its ledger by `nodeHash`, which is a
    /// one-way digest of `(handle, tolerance bits)`, so the tolerance half of the extension's job key
    /// is not recoverable here — and a cancel that can only retire SOME of the jobs it means to
    /// retire is worse than the whole-registry branch the capability already offers
    /// (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`'s `tessellateCancel` handler).
    pub fn preview_cancel_invocation_request_json(window_id: &str, window_kind_id: &str) -> String {
        format!("{{\"windowId\":{},\"windowKindId\":{}}}", semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::String(window_id.to_string())), semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::String(window_kind_id.to_string())))
    }

    /// 🧊️ Decodes the explicit diagnostic envelope; normal response work retains its original source.
    pub fn resolve_preview_eval_cold(&mut self,node_hash:u64,envelope_json:&str)->Result<PreviewEvalOutcome,ValueError>{
        let envelope=semio_framework_pack_json::parse(envelope_json,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(semio_framework_pack_json::JsonError::into_value_error)?;
        if let Some(pending)=envelope.get("pending"){
            let text=|key|pending.get(key).and_then(semio_framework_pack_json::Value::as_str).map(str::to_owned).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original pending response metadata is absent"));
            let hash=pending.get("nodeHash").and_then(semio_framework_pack_json::Value::as_u64).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original pending response hash is absent"))?;
            return Ok(PreviewEvalOutcome::Pending(neural::PendingExtensionEval{neuron_id:text("neuronId")?,extension_id:text("extensionId")?,operator_id:text("operatorId")?,node_hash:hash,input_json:text("inputJson")?}));
        }
        let done=envelope.get("done").and_then(semio_framework_pack_json::Value::as_bool).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original evaluation response completion is absent"))?;
        let phase=PreviewEvalPhase::from_job_tag(envelope.get("phase").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original evaluation response phase is absent"))?);
        let units_done=envelope.get("unitsDone").and_then(semio_framework_pack_json::Value::as_u64).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original evaluation response done units are absent"))?;
        let units_total=envelope.get("unitsTotal").and_then(semio_framework_pack_json::Value::as_u64).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original evaluation response total units are absent"))?;
        if !done{self.eval_progress_by_hash.insert(node_hash,PreviewEvalProgress{units_done,units_total,phase});return Ok(PreviewEvalOutcome::Working)}
        self.eval_progress_by_hash.remove(&node_hash);
        if matches!(phase,PreviewEvalPhase::Cancelled){return Ok(PreviewEvalOutcome::Cancelled)}
        let output_json=envelope.get("outputJson").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original completed evaluation payload is absent"))?.to_owned();Ok(PreviewEvalOutcome::Complete{output_json})
    }

    /// 📈 Aggregate progress of every budgeted evaluation this session has admitted and not yet
    /// finished — what the preview window's status object reports for the `evaluate` half of the
    /// work.
    pub fn preview_eval_status(&self) -> PreviewEvalStatus {
        let mut status = PreviewEvalStatus { in_flight: self.eval_progress_by_hash.len() as u32, ..PreviewEvalStatus::default() };
        for progress in self.eval_progress_by_hash.values() {
            status.units_done = status.units_done.saturating_add(progress.units_done);
            status.units_total = status.units_total.saturating_add(progress.units_total);
            status.phase = progress.phase;
        }
        status
    }

    /// ⛓️ The CHAIN's own progress — the ledger that is live for the WHOLE of an evaluation, which
    /// neither finer ledger is.
    ///
    /// 🩸️ [`FlowEvalSession::preview_tessellate_status`] reads `pending_tessellate_by_hash` and
    /// [`FlowEvalSession::preview_eval_status`] reads `eval_progress_by_hash`. Both are empty at
    /// every hop boundary of a `flowEvalTick` chain: a tessellation is admitted and answered inside
    /// one hop, and a budgeted evaluation only lands in its ledger once an extension has already
    /// answered `done: false`. An `evaluate` round trip parked at the geometry extension — which is
    /// where a boolean preview spends its whole slow half — is recorded in NEITHER. So every status
    /// a preview window published between hops was byte-identical `phase: "idle", inFlight: 0,
    /// ratio: 1.0` while the kernel was busy, on both renderers: 54 identical publications across a
    /// 23 s evaluation, no pill text, no progress, nothing to read
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md`).
    ///
    /// ⛓️ What IS live throughout is the per-window tick latch — `armed` (a hop is outstanding),
    /// `in_flight` (an extension answer is owed), `owed` (an answer asked for a continuation) and
    /// `unfinished` (the window's own last tick said there is more) — plus `tick_scheduled`. The
    /// monotone denominator is the session's own published per-node census: a node leaves
    /// `queued`/`computing` exactly once per chain and never returns to it, so
    /// `nodes_done / nodes_total` only ever grows while one evaluation runs.
    pub fn preview_chain_status(&self) -> PreviewChainStatus {
        let in_flight: u32 = self.window_tick_latches.values().map(|latch| latch.in_flight).sum();
        let working = self.tick_scheduled || self.window_tick_latches.values().any(|latch| latch.armed || latch.owed || latch.unfinished || latch.in_flight > 0);
        let mut status = PreviewChainStatus { in_flight, working, ..PreviewChainStatus::default() };
        if let Some(widgets) = semio_framework_pack_json::parse(&self.status_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_object().cloned()) {
            for (_, entry) in widgets.iter() {
                status.nodes_total = status.nodes_total.saturating_add(1);
                if !matches!(entry.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("queued" | "computing")) {
                    status.nodes_done = status.nodes_done.saturating_add(1);
                }
            }
        }
        status
    }

    /// 🛑 The `evaluateCancel` request body the cancelling command sends to the geometry extension.
    /// Whole-registry, for the same reason [`FlowEvalSession::preview_cancel_invocation_request_json`]
    /// is: a session cancels its preview, not one named operator hop.
    pub fn preview_eval_cancel_invocation_request_json(window_id: &str, window_kind_id: &str) -> String {
        Self::preview_cancel_invocation_request_json(window_id, window_kind_id)
    }

    /// 🛑 Whether the user's explicit cancel is the last thing that happened to this evaluation —
    /// what makes a surface able to publish `phase: "cancelled"` even when the gesture landed before
    /// any tessellation had been admitted.
    pub fn preview_cancelled(&self) -> bool {
        self.preview_cancelled
    }

    /// ⏳️ How many extension answers this session is waiting for across EVERY window it publishes
    /// into — the session-wide half of "is there work in flight", which the per-window latch can only
    /// answer one window at a time. A surface's `cancellable` is this OR a tessellation in flight: an
    /// `evaluate` round trip admits no tessellation at all, so the tessellation ledger alone reports
    /// `idle` through the whole (slowest) part of a boolean preview.
    pub fn extensions_in_flight(&self) -> u32 {
        self.window_tick_latches.values().map(|latch| latch.in_flight).sum()
    }

    /// 📥️ Advances the genuine original completed receiver into the session-owned canonical cache.
    pub fn next_preview_tessellate_demands(&self,receiver:&FlowPreviewCompletedReceiver,copy:usize)->Result<RetirementDemand,ValueError>{receiver.next_demands(&self.preview_mesh_by_handle,copy)}
    pub fn resolve_preview_tessellate(&mut self,receiver:&mut FlowPreviewCompletedReceiver,address:&FlowPreviewCompletedAddress<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original preview session is closing").into())}
        receiver.step(address,&mut self.state.preview_mesh_by_handle,cx)
    }
}
