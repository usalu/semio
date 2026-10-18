//! 📊️ Evaluation baseline and publication of the flow host: applying evaluation outputs, the baseline install and currency, the baseline and evaluation publication steps and the computing chrome. See [`super`].

use super::*;

impl FlowHost {
    /// 📥️ Applies channel-structured eval JSON from an off-thread worker without re-running operators.
    pub fn apply_eval_outputs_json(&mut self, json: &str)->Result<(),ValueError> {
        if self.pending_baseline_publication.is_some(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline publication must advance before another intake"))}
        if is_global_eval_error_json(json) {
            self.dag.paint.clear_computing();
            return Ok(());
        }
        let outputs = outputs_from_channel_eval_json(json);
        let inputs = inputs_from_channel_eval_json(json);
        let channels = EvalChannels { outputs, inputs };
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.previous_snapshot.as_deref(), &snapshot);
        let evaluated_generation = self.operator_registry_generation();
        let converged = self.probe_eval_outputs_converged(&tree, &seeds, &dirty, &channels);
        tree.retire_cold();
        seeds.retire_cold();
        self.last_eval_json = json.to_string();
        if converged {
            self.apply_preview_outputs(&channels.outputs);
            self.apply_export_outputs(&channels.outputs);
            self.begin_baseline_publication(snapshot,channels,evaluated_generation).unwrap_or_else(|_|unreachable!("original baseline intake was checked before constructing owners"));
            self.dag.paint.clear_computing();
        } else {
            channels.retire_cold();
            self.refresh_computing_chrome_from_pending();
        }
        Ok(())
    }

    fn probe_eval_outputs_converged(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>, dirty: &HashSet<String>, channels: &EvalChannels) -> bool {
        let registry = self.operator_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        let mut probe_never_dispatches = |kind: &str, _: &Dictionary| -> Result<Dictionary, EvalError> { Err(EvalError::InvalidInput(format!("apply_eval_outputs_json probed a dispatch for {kind}"))) };
        match evaluator.evaluate_channels_budgeted(tree, seeds, &self.kind_infos, &mut probe_never_dispatches, &self.neural_cache, dirty, Some(channels), EvalStepBudget::PROBE,&|_|true) {
            Ok(BudgetedEval { remaining, channels, .. }) => {
                channels.retire_cold();
                remaining.is_empty()
            }
            Err(_) => false,
        }
    }

    /// 🧵️ Installs a durable eval baseline from an off-thread driver onto this ephemeral host.
    ///
    /// `registry_generation` is the flow extension registry replacement the driver's baseline was
    /// computed against, and it travels WITH the baseline rather than being read from the live
    /// registry here: an ephemeral host is rebuilt after the install that bumped the generation, so
    /// reading "now" would tell every such host its inherited baseline is current when it is
    /// precisely the one that is not (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn install_eval_baseline(&mut self,snapshot:Option<Arc<TreeSnapshot>>,channels:Option<Arc<EvalChannels>>,registry_generation:u64,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Option<Arc<TreeSnapshot>>,Option<Arc<EvalChannels>>)>{
        let refusal=if grant.maximum_items==0{Some((ValueRefusalKind::WorkLimit,"baseline lease handoff requires one admitted item"))}
            else if grant.maximum_depth==0{Some((ValueRefusalKind::DepthLimit,"baseline lease handoff requires source depth"))}
            else if !self.baseline_publication_terminal_is_empty(){Some((ValueRefusalKind::WorkLimit,"baseline publication must complete before installing another lease"))}
            else if self.current_channels.is_some(){Some((ValueRefusalKind::UnsupportedOwner,"baseline installation requires original output lease handoff"))}else{None};
        if let Some((kind,message))=refusal{return Err((ValueError::literal(kind,message),snapshot,channels))}
        let original=FlowBaselineLeases{snapshot:self.previous_snapshot.take(),channels:self.previous_channels.take(),current:self.current_channels.take()};
        if original.snapshot.is_some()||original.channels.is_some()||original.current.is_some(){self.baseline_retirement=Some(ControlledRetirement::new(FlowPublicationDisplaced{publication:None,text:None,display:None,exports:None,pending:Vec::new(),remaining:Vec::new(),snapshot:None,leases:original}).unwrap_or_else(|_|unreachable!("original baseline leases are supported")));}
        self.previous_snapshot=snapshot;self.current_channels=channels.clone();self.previous_channels=channels;self.baseline_registry_generation=registry_generation;
        Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
    }

    /// 🔎️ Reads the original active output owner or its immutable installed baseline lease.
    pub fn output_channels(&self,widget_id:&str)->Option<&Dictionary>{
        self.current_channels.as_deref().or(self.previous_channels.as_deref())?.outputs.get(widget_id)
    }
    /// 📖️ Borrows actual output rows without restoring a copied map.
    pub fn output_entries(&self)->impl Iterator<Item=(&String,&Dictionary)>{
        self.current_channels.as_deref().or(self.previous_channels.as_deref()).into_iter().flat_map(|channels|channels.outputs.iter())
    }

    pub(super) fn displace_dictionary(&mut self, value: Dictionary) {
        let grant = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 4096 };
        if let Err((_, value)) = self.displaced.push_dictionary(value, grant) { value.retire_cold(); }
    }

    /// 🧹️ Hands every value displaced since the last drain to the artifact's bounded retirement
    /// ladder. Called at the START of an evaluation tick, so a root the CURRENT tick's `NeuralCache`
    /// or `previous_channels` still reads is never torn down under it — the frontier therefore holds
    /// at most one tick's displacement, and what is left when the host closes is taken over by
    /// [`FlowHostRetirement::new`].
    pub(in super::super) fn drain_displaced(&mut self) {
        neural::retirement::retire_value_cold(std::mem::take(&mut self.displaced));
    }

    /// 📥️ Retains the original baseline and export arenas inline before funded retirement.
    pub(super) fn displace_eval_state(&mut self)->Result<(),ValueError>{
        if self.pending_baseline_publication.is_some()||self.baseline_retirement.is_some(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline publication must finish before displacement"))}
        let original=FlowPublicationDisplaced{publication:None,text:None,display:None,pending:std::mem::take(&mut self.pending_extension_evals),remaining:std::mem::take(&mut self.pending_remaining),exports:Some(std::mem::take(&mut self.export_payloads)),snapshot:None,leases:FlowBaselineLeases{snapshot:self.previous_snapshot.take(),channels:self.previous_channels.take(),current:self.current_channels.take()}};
        self.baseline_retirement=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original baseline fields declare typed retirement")));Ok(())
    }

    /// 🧵️ Captures this host's eval baseline for persistence on a durable driver.
    pub fn eval_baseline(&self) -> (Option<Arc<TreeSnapshot>>, Option<Arc<EvalChannels>>) {
        (self.previous_snapshot.clone(), self.previous_channels.clone())
    }

    /// 📥️ Retains the evaluator's genuine computed owners before admitting shared publication.
    pub fn begin_baseline_publication(&mut self,snapshot:TreeSnapshot,channels:EvalChannels,generation:u64)->Result<(),(ValueError,TreeSnapshot,EvalChannels)>{
        self.begin_eval_publication(snapshot,channels,generation,true)
    }

    pub(in super::super) fn begin_eval_publication(&mut self,snapshot:TreeSnapshot,channels:EvalChannels,generation:u64,converged:bool)->Result<(),(ValueError,TreeSnapshot,EvalChannels)>{
        if !self.baseline_publication_terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original evaluation publication is already pending"),snapshot,channels))}
        self.pending_baseline_publication=Some(FlowBaselinePublication{revision:self.evaluation_revision,display:Some(FlowDisplayCursor::new()),snapshot:Some(snapshot),channels:Some(channels),shared_snapshot:None,shared_channels:None,generation,converged,pending:Vec::new(),remaining:Vec::new()});Ok(())
    }
    /// 🌊️ Moves the evaluator's original wave and remaining rows together with its channel authority.
    pub(in super::super) fn begin_work_publication(&mut self,output:semio_framework_os_flow_host_source::work::FlowEvaluationCompletion)->Result<(),(ValueError,semio_framework_os_flow_host_source::work::FlowEvaluationCompletion)>{
        if !self.baseline_publication_terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original Work publication is already pending"),output))}
        let semio_framework_os_flow_host_source::work::FlowEvaluationCompletion{snapshot,channels,remaining,pending_extensions,generation}=output;
        self.pending_baseline_publication=Some(FlowBaselinePublication{revision:self.evaluation_revision,display:Some(FlowDisplayCursor::new()),snapshot:Some(snapshot),channels:Some(channels),shared_snapshot:None,shared_channels:None,generation,converged:remaining.is_empty(),pending:pending_extensions,remaining});Ok(())
    }
    pub fn pending_evaluation_ids(&self)->&[String]{&self.pending_remaining}
    /// 📏️ Borrows each independent currency of the actual publication owner.
    pub fn next_baseline_publication_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.baseline_retirement.as_ref(){return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
        let Some(owner)=self.pending_baseline_publication.as_ref()else{return Ok(Default::default())};
        if owner.revision!=self.evaluation_revision{return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(display)=owner.display.as_ref(){if !display.complete(){return display.demands(&self.host_snapshot,owner.channels.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original display channel source was transferred"))?,&self.kind_infos)}}
        let capacity_bytes=if owner.converged&&owner.snapshot.is_some(){semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<TreeSnapshot>()}else if owner.channels.is_some(){semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<EvalChannels>()}else{0};
        Ok(RetirementDemand{capacity_bytes,depth:1,..Default::default()})
    }
    /// 🎟️ Creates one admitted Arc header or transfers the same original baseline owners.
    pub fn baseline_publication_step_progress(&self)->RetainedCloneProgress{self.baseline_publication_progress}

    pub fn baseline_publication_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.baseline_publication_progress=Default::default();
        let empty=RetainedCloneProgress::default();
        if self.baseline_publication_terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        let demand=self.next_baseline_publication_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original baseline publication exceeds admitted depth"))}
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(owner)=self.baseline_retirement.as_mut(){
            let result=owner.step(grant);self.baseline_publication_progress=owner.step_progress();let step=result.map_err(|error|error.with_retained_progress(self.baseline_publication_progress))?;
            let progress=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,owner.terminal_is_empty(),"Flow original baseline source")?.progress();
            if owner.terminal_is_empty(){self.baseline_retirement=None;}
            return Ok(if self.baseline_publication_terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
        }
        if self.pending_baseline_publication.as_ref().is_some_and(|owner|owner.revision!=self.evaluation_revision){let original=FlowPublicationDisplaced{publication:self.pending_baseline_publication.take(),text:None,display:None,exports:None,pending:Vec::new(),remaining:Vec::new(),snapshot:None,leases:FlowBaselineLeases{snapshot:None,channels:None,current:None}};self.baseline_retirement=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original canceled publication source")));self.baseline_publication_progress=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok(RetainedCloneStep::Progress(self.baseline_publication_progress))}
        let owner=self.pending_baseline_publication.as_mut().unwrap();
        if let Some(display)=owner.display.as_mut(){if !display.complete(){let result=display.step(&self.host_snapshot,owner.channels.as_ref().unwrap(),&self.kind_infos,grant);match result{Ok(progress)=>{self.baseline_publication_progress=progress;return Ok(RetainedCloneStep::Progress(progress))},Err(error)=>{self.baseline_publication_progress=error.retained_progress();return Err(error)}}}}
        if owner.converged{if let Some(source)=owner.snapshot.take(){owner.shared_snapshot=Some(FlowPublicationShared(Arc::new(source)));self.baseline_publication_progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..empty};return Ok(RetainedCloneStep::Progress(self.baseline_publication_progress))}}
        if let Some(source)=owner.channels.take(){owner.shared_channels=Some(FlowPublicationShared(Arc::new(source)));self.baseline_publication_progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..empty};return Ok(RetainedCloneStep::Progress(self.baseline_publication_progress))}
        let (display,text)=match owner.display.take(){Some(mut display)=>{let text=display.take_text();(Some(display),text)},None=>(None,None)};
        let previous_text=text.map(|text|std::mem::replace(&mut self.last_eval_json,text));
        let original=FlowPublicationDisplaced{publication:None,text:previous_text,display,exports:None,pending:std::mem::take(&mut self.pending_extension_evals),remaining:std::mem::take(&mut self.pending_remaining),snapshot:owner.snapshot.take(),leases:FlowBaselineLeases{
            snapshot:if owner.converged{self.previous_snapshot.take()}else{None},channels:if owner.converged{self.previous_channels.take()}else{None},current:self.current_channels.take(),
        }};
        if original.text.is_some()||original.display.is_some()||original.pending.capacity()!=0||original.remaining.capacity()!=0||original.snapshot.is_some()||original.leases.snapshot.is_some()||original.leases.channels.is_some()||original.leases.current.is_some(){self.baseline_retirement=Some(ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("original publication sources are supported")));}
        self.pending_extension_evals=std::mem::take(&mut owner.pending);
        self.pending_remaining=std::mem::take(&mut owner.remaining);
        self.current_channels=owner.shared_channels.take().map(|lease|lease.0);
        if owner.converged{self.previous_snapshot=owner.shared_snapshot.take().map(|lease|lease.0);self.previous_channels=self.current_channels.clone();self.baseline_registry_generation=owner.generation;}
        self.pending_baseline_publication=None;
        let progress=RetainedCloneProgress{copied_items:1,..empty};self.baseline_publication_progress=progress;
        Ok(if self.baseline_publication_terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    /// 🏁️ Publication completes only after all displaced original leases close.
    pub fn baseline_publication_terminal_is_empty(&self)->bool{self.pending_baseline_publication.is_none()&&self.baseline_retirement.is_none()}

    /// 🔢 The flow extension registry replacement this host's eval baseline was computed against.
    pub fn eval_baseline_registry_generation(&self) -> u64 {
        self.baseline_registry_generation
    }

    /// 🔢 Whether the incremental baseline may still be believed — i.e. whether the operator table
    /// it was dispatched through is the one installed right now.
    ///
    /// `compute_dirty_set` diffs the TREE, and a contributed operator arriving in a registry
    /// replacement changes no tree at all: the operator table is process-wide state swapped out
    /// underneath a live host. So a stale generation must make the WHOLE tree dirty and drop the
    /// previous channels, not merely refuse the skip — a refused skip that still diffs against the
    /// old snapshot computes an empty dirty set and dispatches nothing, which is the same stale
    /// answer by a longer road (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn baseline_is_current(&self) -> bool {
        self.baseline_registry_generation == self.operator_registry_generation()
    }

    /// ⏮️ The snapshot to diff this evaluation against — `None` once the registry it was computed
    /// against has been replaced, which makes every node dirty.
    pub(in super::super) fn current_baseline_snapshot(&self) -> Option<&TreeSnapshot> {
        self.previous_snapshot.as_deref().filter(|_| self.baseline_is_current())
    }

    /// ⏮️ The channels this evaluation may free-ride on, under the same condition.
    pub(in super::super) fn current_baseline_channels(&self) -> Option<&EvalChannels> {
        self.previous_channels.as_deref().filter(|_| self.baseline_is_current())
    }

    /// ⚡️ Whether the incremental fast path may skip this evaluation entirely: nothing is dirty, a
    /// full channel baseline is held, and outputs are published.
    ///
    /// One predicate, read by every caller that owns a fast path ([`FlowHost::evaluate_step`] and
    /// [`FlowHost::pending_eval_widget_ids`]), so the two can never drift apart.
    pub(in super::super) fn baseline_answers_everything(&self, dirty: &HashSet<String>) -> bool {
        dirty.is_empty() && self.current_baseline_channels().is_some() && self.output_entries().next().is_some()
    }

    /// ⚙️ Probes pending nodes and paints active/stale computing chrome on the DAG canvas.
    pub fn refresh_computing_chrome_from_pending(&mut self) {
        let remaining = self.pending_eval_widget_ids();
        if remaining.is_empty() {
            self.dag.paint.clear_computing();
            return;
        }
        let active = remaining.first().map(|id| id.as_str());
        let stale = remaining.get(1..).unwrap_or(&[]).to_vec();
        set_computing_progress(&mut self.dag.graph, &mut self.dag.paint, active, &stale);
    }

    /// ⚙️ Marks one actively computing widget and downstream widgets as stale.
    pub fn set_node_statuses(&mut self,statuses:&DagNodeStatuses){set_node_statuses(&mut self.dag.graph, &mut self.dag.paint, statuses);}

    pub fn set_computing_progress(&mut self, active_widget_id: Option<&str>, stale_widget_ids: &[String]) {
        set_computing_progress(&mut self.dag.graph, &mut self.dag.paint, active_widget_id, stale_widget_ids);
    }

    /// ✅️ Clears computing chrome from all widgets.
    pub fn clear_computing_widget_ids(&mut self) {
        self.dag.paint.clear_computing();
    }
}
