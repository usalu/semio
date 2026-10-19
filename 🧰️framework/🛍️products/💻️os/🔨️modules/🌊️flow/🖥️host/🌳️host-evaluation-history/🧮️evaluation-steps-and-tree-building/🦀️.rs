//! 🧮️ Evaluation steps and tree building of the flow host: the bounded evaluation steps, blocked ports, dirty widget ids, the status trees and the tree and cluster tree built from the DAG. See [`super`].

use super::*;

impl FlowHost {
    pub fn evaluate_internal(&mut self) {
        self.evaluate_cold_step(EvalStepBudget::UNBOUNDED,&|_|true);
    }

    /// 🧵️ Advances one genuine source, evaluator, publication, or custody event with the caller's wallet.
    pub fn evaluate_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<FlowEvaluationStep,neural::OperatorWorkFailure>{
        self.evaluation_progress=Default::default();if cx.is_cancelled()||cx.should_yield(){return Ok(FlowEvaluationStep::Working)}let grant=cx.retained_grant();let demand=self.next_evaluation_demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(FlowEvaluationStep::Working)}
        if !self.baseline_publication_terminal_is_empty(){let result=self.baseline_publication_step(grant);self.evaluation_progress=self.baseline_publication_step_progress();if self.evaluation_progress!=Default::default()||result.is_err(){cx.consume_fuel(1)}match result{Ok(_)=>cx.consume_retained(self.evaluation_progress).map_err(semio_framework_job::OriginalTurnFailure::from)?,Err(error)=>return Err(semio_framework_job::OriginalTurnFailure::debit_original(error.with_retained_progress(self.evaluation_progress),cx).into())}return Ok(FlowEvaluationStep::Working)}
        let generation=self.operator_registry_generation();
        if let Some(work)=self.pending_evaluation.as_mut(){
            if !work.source_matches(self.evaluation_revision,generation){work.begin_close()}
            if work.is_closing(){let completed=work.output_was_transferred()&&work.source_matches(self.evaluation_revision,generation);let result=semio_framework_job::close_original_retained_step(cx,demand,|grant|work.close_step(grant));self.evaluation_progress=work.step_progress();result?;if work.terminal_is_empty(){self.pending_evaluation=None;return Ok(if completed{FlowEvaluationStep::Complete}else{FlowEvaluationStep::Working})}return Ok(FlowEvaluationStep::Working)}
            let result=work.step(&self.host_snapshot,cx);self.evaluation_progress=work.step_progress();
            if let Some(output)=result?{match self.begin_work_publication(output){Ok(())=>self.pending_evaluation.as_mut().unwrap().begin_close(),Err((error,output))=>{self.pending_evaluation.as_mut().unwrap().restore_completion(output);return Err(error.into())}}}
            return Ok(FlowEvaluationStep::Working)
        }
        let registry=self.operator_registry.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original normal Host requires its preborn registry authority"))?.clone();
        self.pending_evaluation=Some(FlowEvaluationWork::new(self.kind_infos.clone(),registry,self.previous_channels.clone(),self.evaluation_revision,generation));self.evaluation_progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_fuel(1);cx.consume_retained(self.evaluation_progress).map_err(semio_framework_job::OriginalTurnFailure::from)?;Ok(FlowEvaluationStep::Working)
    }
    pub fn evaluation_step_progress(&self)->RetainedCloneProgress{self.evaluation_progress}
    pub fn has_pending_evaluation(&self)->bool{self.pending_evaluation.is_some()||!self.baseline_publication_terminal_is_empty()}
    pub fn next_evaluation_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if !self.baseline_publication_terminal_is_empty(){return self.next_baseline_publication_demands(copy)}
        self.pending_evaluation.as_ref().map_or(Ok(RetirementDemand{depth:1,..Default::default()}),|work|if work.is_closing()||!work.source_matches(self.evaluation_revision,self.operator_registry_generation())||work.fault().is_some(){work.next_close_demands(copy)}else{work.next_demands(&self.host_snapshot,copy)})
    }

    pub fn evaluate_cold_step(&mut self, budget: EvalStepBudget,source_required:&dyn Fn(u64)->bool) -> Result<Vec<String>,ValueError> {
        if !self.baseline_publication_terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original baseline publication must advance before another evaluation"))}
        self.drain_displaced();
        self.pending_extension_evals.clear();
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.current_baseline_snapshot(), &snapshot);
        if self.baseline_answers_everything(&dirty) {
                        snapshot.retire_cold();
tree.retire_cold();
            seeds.retire_cold();
            return Ok(Vec::new());
        }
        let evaluated_generation = self.operator_registry_generation();
        let registry = self.operator_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        self.neural_cache.begin_epoch();
        let previous = self.current_baseline_channels();
        let mut dispatch = |kind: &str, input: &Dictionary| registry.as_ref().dispatch(kind, input);
        let budgeted = evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut dispatch, &self.neural_cache, &dirty, previous, budget,source_required);
        tree.retire_cold();
        seeds.retire_cold();
        match budgeted {
            Ok(BudgetedEval { channels, remaining, pending_extensions }) => {
                self.pending_extension_evals = pending_extensions;
                self.apply_preview_outputs(&channels.outputs);
                self.apply_export_outputs(&channels.outputs);
                self.last_eval_json = build_channel_eval_json(&self.host_snapshot, &channels, &self.kind_infos);
                if !remaining.is_empty(){
                    self.begin_eval_publication(snapshot,channels,evaluated_generation,false).unwrap_or_else(|_|unreachable!("original evaluation publication was checked before intake"));
                    return Ok(remaining);
                }
                self.neural_cache.sweep();
                self.begin_baseline_publication(snapshot,channels,evaluated_generation).unwrap_or_else(|_|unreachable!("original evaluation intake was checked before constructing owners"));
                Ok(Vec::new())
            }
            Err(err) => {
                self.neural_cache.sweep();
                if self.last_eval_json.is_empty() || is_global_eval_error_json(&self.last_eval_json) {
                    self.last_eval_json = semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".to_string(), semio_framework_pack_json::Value::String(err.to_string()))]));
                }
                Ok(Vec::new())
            }
        }
    }

    /// 🔌️ Consumes the wave of contributed-extension eval requests the last budgeted step parked.
    /// Empty when nothing is owed; otherwise every member may be invoked at once.
    pub fn take_pending_extension_evals(&mut self) -> Vec<neural::PendingExtensionEval> {
        std::mem::take(&mut self.pending_extension_evals)
    }

    /// 👀️ Probes which widget ids still need evaluation without computing anything (`budget = 0`) —
    /// used to decide whether a tick chain must be (re)armed and what to mark as computing/stale.
    pub fn eval_baseline_snapshot(&self) -> Option<&TreeSnapshot> {
        self.previous_snapshot.as_deref()
    }

    pub fn widget_blocked_ports(&self, widget_id: &str) -> Vec<String> {
        let Some(operator_info) = self.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == widget_id).and_then(|widget| widget_operator_info(widget, &self.kind_infos)) else {
            return Vec::new();
        };
        let missing = self.blocked_input_ports(widget_id, &operator_info);
        ColdRetire::retire_cold(operator_info);
        missing
    }

    /// 🚧️ The declared inputs of `operator_info` that are wired from a source with no published output yet.
    fn blocked_input_ports(&self, widget_id: &str, operator_info: &OperatorInfo) -> Vec<String> {
        if operator_info.variadic_input.is_some() {
            return Vec::new();
        }
        let tree = self.build_tree();
        let mut outputs = self.build_seeds();
        for (widget_id, channels) in self.output_entries() {
            if let Some(displaced) = outputs.insert(widget_id.clone(), channels.clone()) {
                displaced.retire_cold();
            }
        }
        let mut missing = Vec::new();
        for channel in &operator_info.inputs {
            if channel.name == "*" {
                continue;
            }
            if channel.cardinality != neural::Cardinality::ExactlyOne {
                continue;
            }
            if channel.default.is_some() {
                continue;
            }
            let wired = tree.synapses.iter().any(|syn| syn.to == widget_id && syn.to_port == channel.name);
            if !wired {
                continue;
            }
            let source_ready = tree.synapses.iter().filter(|syn| syn.to == widget_id && syn.to_port == channel.name).all(|syn| outputs.contains_key(&syn.from));
            if !source_ready {
                missing.push(channel.name.clone());
            }
        }
        tree.retire_cold();
        outputs.retire_cold();
        missing
    }

    /// 🧭️ Which widgets an edit from `previous` to this document leaves DIRTY, in declaration order —
    /// the incremental contract as a number a law can read, without a contributed operator table and
    /// without running a single dispatch.
    ///
    /// ⚖️ This is exactly the set [`FlowHost::evaluate_step`] recomputes and its complement is exactly
    /// the set that free-rides on the previous channels: one `compute_dirty_set` over the same two
    /// [`TreeSnapshot`]s, so a law stating "moving `height` re-evaluates the extrude branch and
    /// nothing else" is stating what the evaluator does rather than a parallel rule that could drift
    /// from it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn dirty_widget_ids_since(&self, previous: &FlowHost) -> Vec<String> {
        let (before_tree, before_seeds) = (previous.build_tree(), previous.build_seeds());
        let baseline = TreeSnapshot::capture(&before_tree, &before_seeds);
        before_tree.retire_cold();
        before_seeds.retire_cold();
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let current = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(Some(&baseline), &current);
        current.retire_cold();
        baseline.retire_cold();
        tree.retire_cold();
        seeds.retire_cold();
        self.host_snapshot.widgets.iter().map(widget_id_for).filter(|id| dirty.contains(*id)).map(str::to_string).collect()
    }

    pub fn build_tree_for_status(&self) -> Tree {
        self.build_tree()
    }

    pub fn build_seeds_for_status(&self) -> HashMap<String, Dictionary> {
        self.build_seeds()
    }

    pub fn pending_eval_widget_ids(&self) -> Vec<String> {
        let tree = self.build_tree();
        let seeds = self.build_seeds();
        let snapshot = TreeSnapshot::capture(&tree, &seeds);
        let dirty = compute_dirty_set(self.current_baseline_snapshot(), &snapshot);
                snapshot.retire_cold();
if self.baseline_answers_everything(&dirty) {
            tree.retire_cold();
            seeds.retire_cold();
            return Vec::new();
        }
        let registry = self.operator_registry();
        let evaluator = Evaluator::new(registry.as_ref());
        let previous = self.current_baseline_channels();
        let mut probe_never_dispatches = |kind: &str, _: &Dictionary| -> Result<Dictionary, EvalError> { Err(EvalError::InvalidInput(format!("pending_eval_widget_ids probed a dispatch for {kind}"))) };
        let pending = match evaluator.evaluate_channels_budgeted(&tree, &seeds, &self.kind_infos, &mut probe_never_dispatches, &self.neural_cache, &dirty, previous, EvalStepBudget::PROBE,&|_|true) {
            Ok(BudgetedEval { remaining, channels, .. }) => {
                channels.retire_cold();
                remaining
            }
            Err(_) => Vec::new(),
        };
        tree.retire_cold();
        seeds.retire_cold();
        pending
    }

    // #region 🌳️TreeBuilding
    pub(in super::super) fn build_tree(&self) -> Tree {
        let fixture = self.build_dag_host_snapshot_v1();
        let (nodes, edges) = dag_host_snapshot_execution_rows(&fixture);
        Self::tree_from_dag(&nodes, &edges)
    }

    // #region 🔗️DagTreeConversion
    pub fn tree_from_dag(nodes: &[WireNode], edges: &[WireEdge]) -> Tree {
        let neurons = nodes
            .iter()
            .map(|node| {
                let mut params = Self::dictionary_from_property_bag(&node.properties);
                if node.kind == CLUSTER_KIND {
                    if let Some(PropertyValue::String(name)) = node.properties.get("name") {
                        params = params.insert("name", NeuralValue::Atom(Atom::String(name.clone())));
                    }
                }
                let nested = if node.kind == CLUSTER_KIND { Self::cluster_tree_from_node(node).map(Box::new) } else { None };
                Neuron { id: node.id.clone(), kind: node.kind.clone(), params, tree: nested }
            })
            .collect();
        let synapses = edges.iter().enumerate().map(|(index, edge)| Synapse { id: format!("synapse-{index}"), from: edge.from.clone(), to: edge.to.clone(), from_port: edge.from_port.clone(), to_port: edge.to_port.clone() }).collect();
        Tree { neurons, synapses }
    }

    fn cluster_tree_from_node(node: &WireNode) -> Option<Tree> {
        cluster_tree_from_property(node.properties.get("clusterTree")?).ok()
    }

    fn dictionary_from_property_bag(bag: &PropertyBag) -> Dictionary {
        let mut dict = Dictionary::new();
        for (key, value) in bag {
            dict = dict.insert(key, Self::property_value_to_neural(value));
        }
        dict
    }

    fn property_value_to_neural(value: &PropertyValue) -> NeuralValue {
        match value {
            PropertyValue::String(s) => NeuralValue::Atom(Atom::String(s.clone())),
            PropertyValue::Number(n) => NeuralValue::Atom(Atom::Decimal(*n)),
            PropertyValue::Bool(b) => NeuralValue::Atom(Atom::Boolean(*b)),
            PropertyValue::Null => NeuralValue::Atom(Atom::Null),
            PropertyValue::Array(items) => {
                let mut dict = Dictionary::new();
                for (index, row) in items.iter().enumerate() {
                    dict = dict.insert(index.to_string(), Self::property_value_to_neural(row));
                }
                NeuralValue::Dictionary(dict)
            }
            PropertyValue::Object(map) => {
                let mut dict = Dictionary::new();
                for (key, row) in map {
                    dict = dict.insert(key, Self::property_value_to_neural(row));
                }
                NeuralValue::Dictionary(dict)
            }
        }
    }
    // #endregion 🔗️DagTreeConversion
    // #endregion 🌳️TreeBuilding
}
