//! 🔒️ Window tick latch of the evaluation session: the latch demands, arming, owed ticks, extension bookkeeping, continuation admission and latch retirement. See [`super`].

use super::*;
pub use semio_os_flow_host_source::window_key::{FlowEvalWindowKey,FlowEvalWindowKeyWork,FlowEvalWindowKeyAdvance};

//#region 🔒️TickLatch
impl FlowEvalSession {
    /// 🪙️ Quotes the exact original latch arena without constructing a replacement row.
    pub fn next_window_tick_latch_copy_byte_demand(&self,window_id:&str)->Result<usize,ValueError>{self.next_window_tick_key_copy_byte_demand(FlowEvalWindowKey::cold(window_id))}
    pub fn next_window_tick_latch_capacity_byte_demand(&self,window_id:&str,copy:usize)->Result<usize,ValueError>{self.next_window_tick_key_capacity_byte_demand(FlowEvalWindowKey::cold(window_id),copy)}
    pub fn next_window_tick_latch_release_byte_demand(&self,window_id:&str)->Result<usize,ValueError>{self.next_window_tick_key_release_byte_demand(FlowEvalWindowKey::cold(window_id))}
    pub fn next_window_tick_latch_depth_demand(&self,window_id:&str)->Result<usize,ValueError>{self.next_window_tick_key_depth_demand(FlowEvalWindowKey::cold(window_id))}

    /// 🪜️ Drives the staged latch backing admission to completion inside one grant, so the first arming source never mutates an unadmitted latch.
    pub fn admit_window_tick_latch(&mut self,window_id:&str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let mut total=RetainedCloneProgress::default();
        for _ in 0..8{
            let step=self.prepare_window_tick_latch(window_id,grant).map_err(|(error,_)|error)?;
            let progress=step.progress();
            total=RetainedCloneProgress{copied_items:total.copied_items.saturating_add(progress.copied_items),copied_bytes:total.copied_bytes.saturating_add(progress.copied_bytes),retained_capacity_bytes:total.retained_capacity_bytes.saturating_add(progress.retained_capacity_bytes),released_bytes:total.released_bytes.saturating_add(progress.released_bytes)};
            if matches!(step,RetainedCloneStep::Complete(_)){return Ok(total)}
        }
        Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original latch preparation did not complete within its admission turns"))
    }

    /// 🌱️ Admits original backing before placing the original Copy-keyed latch.
    pub fn prepare_window_tick_latch(&mut self,window_id:&str,grant:RetainedCloneGrant)->Result<RetainedCloneStep,(ValueError,RetainedCloneProgress)>{self.prepare_window_tick_latch_original(window_id,grant).result.map_err(|error|{let p=error.retained_progress();(error,p)})}

    /// ⏱️ Carries the real page reservation or atomic Copy-row insertion work.
    pub fn prepare_window_tick_latch_original(&mut self,window_id:&str,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{self.prepare_window_tick_key_original(FlowEvalWindowKey::cold(window_id),grant)}

    fn window_tick_latch_mut(&mut self,window_id:&str,grant:RetainedCloneGrant)->Result<&mut FlowEvalWindowTickLatch,ValueError>{self.window_tick_key_latch_mut(FlowEvalWindowKey::cold(window_id),grant)}

    /// 🔒️ Arms one tick, or records debt while an extension answer is in flight.
    pub fn arm_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        let admitted=self.admit_window_tick_latch(window_id,grant)?;
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        let progress=RetainedCloneProgress{copied_items:admitted.copied_items.saturating_add(1),copied_bytes:admitted.copied_bytes,retained_capacity_bytes:admitted.retained_capacity_bytes,released_bytes:admitted.released_bytes};
        if latch.armed {
            return Ok((false,progress));
        }
        if latch.in_flight > 0 {
            latch.owed = true;
            return Ok((false,progress));
        }
        latch.armed = true;
        latch.owed = false;
        Ok((true,progress))
    }

    /// 🔎️ Reports unfinished, unarmed work with no extension answer in flight.
    pub fn window_tick_owed(&self, window_id: &str) -> bool {
        if self.preview_cancelled{return false}
        let owed = match self.window_tick_latches.get(&flow_eval_window_key(window_id)) {
            None => true,
            Some(latch) => latch.unfinished && !latch.armed && latch.in_flight == 0,
        };
        owed
    }

    /// 🔒️ Arms only the tick that the window currently owes.
    pub fn arm_owed_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        if self.window_tick_owed(window_id){self.arm_window_tick(window_id,grant)}else{Ok((false,Default::default()))}
    }

    /// ▶️ Starts an armed tick and clears the cancellation banner.
    pub fn begin_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {self.begin_window_tick_key(FlowEvalWindowKey::cold(window_id),grant)}

    /// ⏳️ Records parked extension invocations that own the continuation.
    pub fn note_window_extensions_in_flight(&mut self, window_id: &str, count: usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {self.note_window_key_extensions_in_flight(FlowEvalWindowKey::cold(window_id),count,grant)}

    /// 📝️ Records whether the window has unfinished work.
    pub fn note_window_tick_outcome(&mut self, window_id: &str, unfinished: bool,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {let admitted=self.admit_window_tick_latch(window_id,grant)?;let own=self.note_window_tick_key_outcome(FlowEvalWindowKey::cold(window_id),unfinished,grant)?;own.checked_add(admitted)}

    /// 🚧️ Stops a faulted chain until a new gesture arms it.
    pub fn abandon_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        latch.unfinished = false;
        latch.owed = false;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🧹️ Retires one detached window latch per granted turn.
    pub fn retain_window_tick_latches(&mut self,window_ids:&[&str],cursor:&mut usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if grant.maximum_depth<self.state.window_tick_latches.next_extract_slot_depth_demand(*cursor)?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"session latch pruning requires original arena depth"))}
        let slot=*cursor;
        if slot>=self.state.window_tick_latches.slot_count(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        self.state.window_tick_latches.extract_slot_if(slot,|key,_|window_ids.iter().any(|id|flow_eval_window_key(id)==*key));
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        *cursor+=1;
        Ok(if *cursor==self.state.window_tick_latches.slot_count(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    /// ✅️ Settles one answer and transfers any owed arm after the last answer.
    pub fn settle_window_extension(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        latch.in_flight = latch.in_flight.saturating_sub(1);
        if latch.in_flight == 0 && latch.owed && !latch.armed {
            latch.armed = true;
            latch.owed = false;
            return Ok((true,RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }

    /// 🔁️ Allows an owed, uncancelled continuation only within the caller’s time budget.
    pub fn inline_continuation_admitted(&self, window_id: &str, turn_started_us: Option<u64>, now_us: Option<u64>) -> bool {
        !self.preview_cancelled() && self.window_tick_owed(window_id) && flow_eval_inline_continuation_fits(turn_started_us, now_us)
    }

    /// 🔎️ Counts extension answers still owed to the window.
    pub fn window_extensions_in_flight(&self, window_id: &str) -> u32 {
        self.window_tick_latches.get(&flow_eval_window_key(window_id)).map(|latch| latch.in_flight).unwrap_or(0)
    }

    /// 🔎️ Whether a `flowEvalTick` is currently pending for `window_id`.
    pub fn window_tick_is_armed(&self, window_id: &str) -> bool {
        self.window_tick_latches.get(&flow_eval_window_key(window_id)).is_some_and(|latch| latch.armed)
    }

    /// 🧹️ Retires the previous chain’s latch arena before restart.
    pub fn clear_window_tick_latches(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,ValueError> {
        if self.state.window_tick_latches.terminal_is_empty(){return Ok(Some(Default::default()))}
        self.state.retirement.preflight(effect.grant())?;
        let mut source=Some(SessionCollectionOwner::Latches(std::mem::take(&mut self.state.window_tick_latches)));
        let turn=self.state.retirement.admit(&mut source,effect);
        match turn.result{
            Ok(Some(progress))=>Ok(Some(progress)),
            result=>{let Some(SessionCollectionOwner::Latches(original))=source else{unreachable!("session original latch admission kind")};self.state.window_tick_latches=original;result.map(|_|None)},
        }
    }
}
impl FlowEvalSession{
    /// 🔑️ Uses the genuine bounded original window-key output.
    pub fn next_window_tick_key_copy_byte_demand(&self,key:FlowEvalWindowKey)->Result<usize,ValueError>{Ok(semio_os_flow_host_source::window_latch::flow_window_latch_demands(&self.state.window_tick_latches,key,0)?.copy_bytes)}
    /// 🔑️ Uses the genuine bounded original window-key output.
    pub fn next_window_tick_key_capacity_byte_demand(&self,key:FlowEvalWindowKey,copy:usize)->Result<usize,ValueError>{Ok(semio_os_flow_host_source::window_latch::flow_window_latch_demands(&self.state.window_tick_latches,key,copy)?.capacity_bytes)}
    /// 🔑️ Uses the genuine bounded original window-key output.
    pub fn next_window_tick_key_release_byte_demand(&self,key:FlowEvalWindowKey)->Result<usize,ValueError>{Ok(semio_os_flow_host_source::window_latch::flow_window_latch_demands(&self.state.window_tick_latches,key,0)?.release_bytes)}
    /// 🔑️ Uses the genuine bounded original window-key output.
    pub fn next_window_tick_key_depth_demand(&self,key:FlowEvalWindowKey)->Result<usize,ValueError>{Ok(semio_os_flow_host_source::window_latch::flow_window_latch_demands(&self.state.window_tick_latches,key,0)?.depth)}
    /// 🔑️ Uses the genuine bounded original window-key output.
    pub fn prepare_window_tick_key_original(&mut self,key:FlowEvalWindowKey,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{semio_os_flow_host_source::window_latch::prepare_flow_window_latch(&mut self.state.window_tick_latches,key,grant)}
    /// 🔑️ Uses the genuine bounded original window-key output.
    fn window_tick_key_latch_mut(&mut self,key:FlowEvalWindowKey,grant:RetainedCloneGrant)->Result<&mut FlowEvalWindowTickLatch,ValueError>{
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original latch mutation requires one admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original latch mutation requires source depth"))}
        self.state.window_tick_latches.get_mut(&key.fingerprint()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original latch backing must be admitted before mutation"))
    }
    /// 🔑️ Uses the genuine bounded original window-key output.
    pub fn begin_window_tick_key(&mut self, key:FlowEvalWindowKey,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        if !self.preview_cancellation_terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original preview cancellation must finish before another tick"))}
        self.window_tick_key_latch_mut(key,grant)?.armed=false;
        self.state.preview_cancelled = false;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }
    /// 🔑️ Uses the genuine bounded original window-key output.
    pub fn note_window_key_extensions_in_flight(&mut self, key:FlowEvalWindowKey, count: usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let latch = self.window_tick_key_latch_mut(key,grant)?;
        latch.in_flight = latch.in_flight.saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }
    /// 📝️ Records a pre-admitted original keyed latch without hashing the text again.
    pub fn note_window_tick_key_outcome(&mut self, key:FlowEvalWindowKey, unfinished: bool,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let admitted=RetainedCloneProgress::default();
        self.window_tick_key_latch_mut(key,grant)?.unfinished=unfinished;
        Ok(RetainedCloneProgress{copied_items:admitted.copied_items.saturating_add(1),copied_bytes:admitted.copied_bytes,retained_capacity_bytes:admitted.retained_capacity_bytes,released_bytes:admitted.released_bytes})
    }
}
impl FlowEvalSession{
 /// 📏️ Quotes the same fixed keyed window-publication record event.
 pub fn window_evaluation_publication_demands()->RetirementDemand{semio_os_flow_host_source::window_latch::flow_window_publication_demands()}
 /// 📤️ Commits the actual original invocation wave and its window status together.
 pub fn publish_window_evaluation_key_original(&mut self,key:FlowEvalWindowKey,count:usize,unfinished:bool,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{if self.closing||!self.preview_cancellation_terminal_is_empty(){return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original window evaluation waits for live session and cancellation closure")),work:0}}let state=&mut *self.state;semio_os_flow_host_source::window_latch::publish_flow_window_evaluation(&mut state.window_tick_latches,&mut state.preview_cancelled,key,count,unfinished,grant)}
}
//#endregion 🔒️TickLatch

impl FlowEvalSession{
 /// ✅️ Settles the actual keyed reply token once and transfers only its genuine existing continuation.
 pub fn settle_window_reply_key_original(&mut self,key:FlowEvalWindowKey,give_up:bool,g:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<(bool,RetainedCloneProgress)>{if self.closing||!self.preview_cancellation_terminal_is_empty(){return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original reply settlement waits for live session and cancellation closure")),work:0}}semio_os_flow_host_source::window_latch::settle_flow_window_reply(&mut self.state.window_tick_latches,key,give_up,g)}
}
