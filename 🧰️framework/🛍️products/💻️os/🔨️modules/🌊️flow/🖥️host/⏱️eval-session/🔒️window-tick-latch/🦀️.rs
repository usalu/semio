//! 🔒️ Window tick latch of the evaluation session: the latch demands, arming, owed ticks, extension bookkeeping, continuation admission and latch retirement. See [`super`].

use super::*;

//#region 🔒️TickLatch
impl FlowEvalSession {
    /// 🪙️ Quotes the exact original latch arena without constructing a replacement row.
    pub fn next_window_tick_latch_copy_byte_demand(&self,window_id:&str)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_copy_byte_demand(&key)}
    }
    pub fn next_window_tick_latch_capacity_byte_demand(&self,window_id:&str,copy:usize)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_capacity_byte_demand(&key,copy)}
    }
    pub fn next_window_tick_latch_release_byte_demand(&self,window_id:&str)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_release_byte_demand(&key)}
    }
    pub fn next_window_tick_latch_depth_demand(&self,window_id:&str)->Result<usize,ValueError>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){Ok(0)}else{self.state.window_tick_latches.next_insert_depth_demand(&key)}
    }

    /// 🌱️ Admits original backing before placing the original Copy-keyed latch.
    pub fn prepare_window_tick_latch(&mut self,window_id:&str,grant:RetainedCloneGrant)->Result<RetainedCloneStep,(ValueError,RetainedCloneProgress)>{
        let key=flow_eval_window_key(window_id);
        if self.state.window_tick_latches.contains_key(&key){return Ok(RetainedCloneStep::Complete(Default::default()))}
        if grant.maximum_items==0{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original latch preparation requires one admitted item"),Default::default()))}
        if self.state.window_tick_latches.next_insert_capacity_byte_demand(&key,grant.maximum_copy_bytes).map_err(|error|(error,RetainedCloneProgress::default()))?>0{
            return self.state.window_tick_latches.reserve_insert_step(&key,grant).map(RetainedCloneStep::Progress);
        }
        match self.state.window_tick_latches.insert_reserved(key,FlowEvalWindowTickLatch::default(),grant){
            Ok((None,progress))=>Ok(RetainedCloneStep::Complete(progress)),
            Ok((Some(_),_))=>unreachable!("original latch preparation cannot displace an occupied row"),
            Err((error,_,_))=>Err((error,Default::default())),
        }
    }

    fn window_tick_latch_mut(&mut self,window_id:&str,grant:RetainedCloneGrant)->Result<&mut FlowEvalWindowTickLatch,ValueError>{
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original latch mutation requires one admitted item"))}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original latch mutation requires source depth"))}
        self.state.window_tick_latches.get_mut(&flow_eval_window_key(window_id)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original latch backing must be admitted before mutation"))
    }

    /// 🔒️ Arms `window_id`'s `flowEvalTick` if — and only if — nothing already owes one.
    ///
    /// This is the ONE gate every arming source goes through: the host refresh poll
    /// (`ArtifactApp::pending_effects`), the `setContributions` install, `setActiveExample`, every
    /// view command, and the chain's own continuations. Answers whether the CALLER owes the effect,
    /// so a refused arm emits nothing rather than duplicating a tick that is already in flight.
    ///
    /// ⏳️ A window waiting on an extension answer is not armed at all — it is marked `owed` instead,
    /// and [`FlowEvalSession::settle_window_extension`] hands the arm to whichever answer lands
    /// last. Ticking a window whose invocation is outstanding recomputes the identical pending
    /// request, which is how one graph turned into 298 invocations against 111 settles in 80 s of
    /// live console (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn arm_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};
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

    /// 🔎️ Whether `window_id` owes a tick that nothing has armed — a window that has never ticked
    /// owes its first one, and a window whose own last tick reported unfinished work owes another
    /// only while no tick and no extension answer is already chasing it. The REFRESH poll's whole
    /// question, and the reason it costs no evaluation at all.
    pub fn window_tick_owed(&self, window_id: &str) -> bool {
        if self.preview_cancelled{return false}
        match self.window_tick_latches.get(&flow_eval_window_key(window_id)) {
            None => true,
            Some(latch) => latch.unfinished && !latch.armed && latch.in_flight == 0,
        }
    }

    /// 🔒️ Arms the tick `window_id` actually owes — [`FlowEvalSession::window_tick_owed`] and
    /// [`FlowEvalSession::arm_window_tick`] as the single question a refresh poll asks.
    pub fn arm_owed_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant) -> Result<(bool,RetainedCloneProgress),ValueError> {
        if self.window_tick_owed(window_id){self.arm_window_tick(window_id,grant)}else{Ok((false,Default::default()))}
    }

    /// ▶️ Marks `window_id`'s armed tick as RUNNING: the effect has been delivered, so the latch is
    /// free for whatever this tick's own outcome decides to arm next.
    /// 🛑 A tick that actually begins is work RESUMING, which is the one thing that retires the
    /// `cancelled` banner an explicit gesture raised — see [`FlowEvalSession::preview_cancelled`].
    pub fn begin_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        if !self.preview_cancellation_terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original preview cancellation must finish before another tick"))}
        self.window_tick_latch_mut(window_id,grant)?.armed=false;
        self.state.preview_cancelled = false;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// ⏳️ Records that `window_id`'s tick parked `count` extension invocations. Those answers own
    /// the continuation from here — the tick that parked them owes no re-arm.
    pub fn note_window_extensions_in_flight(&mut self, window_id: &str, count: usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        latch.in_flight = latch.in_flight.saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 📝️ Records what `window_id`'s tick reported: `unfinished` is the tick's own "there is more to
    /// compute". Only this makes a refresh poll able to answer [`FlowEvalSession::window_tick_owed`]
    /// without evaluating anything.
    pub fn note_window_tick_outcome(&mut self, window_id: &str, unfinished: bool,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        self.window_tick_latch_mut(window_id,grant)?.unfinished=unfinished;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🚧️ Marks `window_id`'s chain as GIVEN UP: an extension answer this process cannot fold (a
    /// faulted `invokeExtension`, an answer that seeds no cache entry) is not slow work, and the
    /// next tick would park the identical request and fault again at the host's own cadence. Nothing
    /// but a gesture — a contributions install, an example switch, an edit — may resume it, and each
    /// of those arms through [`FlowEvalSession::arm_window_tick`] directly.
    pub fn abandon_window_tick(&mut self, window_id: &str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let latch = self.window_tick_latch_mut(window_id,grant)?;
        latch.unfinished = false;
        latch.owed = false;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🧹️ Drops the latch of every window that is no longer attached. A detached window's latch can
    /// never be discharged by a tick — the retained route refuses a window that has left the
    /// roster — so keeping it would make a window that comes back unarmable forever.
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

    /// ✅️ Folds ONE extension answer back into `window_id`'s latch. Answers whether that settle
    /// discharged an arm a sibling answer had already asked for, so the LAST answer of a fan-out
    /// emits exactly one re-arm and the earlier ones emit none.
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

    /// 🔁️ Whether the fold that just settled `window_id`'s answer may run that window's next wave
    /// INLINE, inside the answer's own guest turn, instead of leaving the `previewEval` run job to
    /// dispatch a `flowEvalTick` hop for it.
    ///
    /// 🪜️ The question is deliberately the run job's OWN scheduling question
    /// ([`FlowEvalSession::window_tick_owed`]): a fold may only take over a hop the scheduler would
    /// otherwise have dispatched, never invent one. So the hop count can fall but the chain's shape
    /// cannot change — and a fold that declines simply leaves the latch alone, which IS the park:
    /// the run job's next step reads the same debt and dispatches the round trip as before. There is
    /// no parked-continuation state anywhere, because "not inline" is the pre-existing behaviour.
    ///
    /// 🛑 A CANCELLED session is refused outright. `begin_window_tick` retires the `cancelled` banner
    /// (work resuming is the one thing that may), so an inline continuation on a cancelled chain
    /// would not merely compute one wave too many — it would un-cancel the run the user stopped. A
    /// cancel also defaults every latch, so `window_tick_owed` already answers `false`; the explicit
    /// guard is kept because the two facts must not be one accident apart
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// ⏱️ The wall half is [`flow_eval_inline_continuation_fits`]: a turn with less than
    /// [`FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US`] of its evaluation allowance left parks. BOTH
    /// instants are the caller's, so the rule is pure and a law states the park boundary in
    /// microseconds instead of racing a clock — and so a native test, which installs no clock at all,
    /// can still drive the branch a browser turn takes.
    pub fn inline_continuation_admitted(&self, window_id: &str, turn_started_us: Option<u64>, now_us: Option<u64>) -> bool {
        !self.preview_cancelled() && self.window_tick_owed(window_id) && flow_eval_inline_continuation_fits(turn_started_us, now_us)
    }

    /// 🔎️ How many extension answers `window_id` is still waiting for — readable so a law can state
    /// the in-flight rule rather than infer it.
    pub fn window_extensions_in_flight(&self, window_id: &str) -> u32 {
        self.window_tick_latches.get(&flow_eval_window_key(window_id)).map(|latch| latch.in_flight).unwrap_or(0)
    }

    /// 🔎️ Whether a `flowEvalTick` is currently pending for `window_id`.
    pub fn window_tick_is_armed(&self, window_id: &str) -> bool {
        self.window_tick_latches.get(&flow_eval_window_key(window_id)).is_some_and(|latch| latch.armed)
    }

    /// 🧹️ Abandons every window's latch — what a chain RESTART owes itself. The results the old
    /// chain was chasing have just been swept (a registry replacement, a fresh document), so the
    /// invocations it parked can no longer resume anything and their windows must be free to be
    /// armed again from scratch.
    pub fn clear_window_tick_latches(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        if self.state.window_tick_latches.terminal_is_empty(){return Ok(Default::default())}
        self.state.retirement.preflight(grant)?;
        let source=SessionCollectionOwner::Latches(std::mem::take(&mut self.state.window_tick_latches));
        match self.state.retirement.admit(source,grant){
            Ok(progress)=>Ok(progress),
            Err((error,SessionCollectionOwner::Latches(original)))=>{self.state.window_tick_latches=original;Err(error)},
            Err(_)=>unreachable!("session original latch admission kind"),
        }
    }
}
//#endregion 🔒️TickLatch
