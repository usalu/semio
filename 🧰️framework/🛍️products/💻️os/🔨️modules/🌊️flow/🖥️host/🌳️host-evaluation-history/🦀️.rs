//! 🌳️ `FlowHost` evaluation steps, tree building, dag synchronisation, node editing, clusters and the undo/redo history.

use super::*;
use semio_framework_os_infinite::{dag_draw_lod_label, dag_pick_targets_at_screen_json, dag_set_canvas_palette};

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
            tree.retire_cold();
            seeds.retire_cold();
            return Ok(Vec::new());
        }
        // 🔢 Read BEFORE the registry it describes: a replacement landing between the two reads
        // then stamps the baseline with the OLDER generation, which over-dirties the next step
        // (extra work, never a stale answer). Reading it after could stamp a generation the
        // evaluation never used.
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
                // 🧹️ The converged channel set is this host's CLAIM on the process-wide geometry
                // store, never its authority over it. Retiring straight from here pruned the kernel
                // to ONE host's handles, so any other live evaluation — a second preview window's
                // session, a viewer instance, a headless oracle — lost the shapes its own cached
                // nodes still name, and the very next evaluation that reused one of those cached
                // dictionaries faulted `missing handle: <digest>` on a node whose inputs had not
                // moved at all. That is what made a moved slider settle on an empty payload while
                // the census reported the chain done. The one authority is
                // the supplied geometry authority, over the MERGED claim of every live session, and
                // [`FlowEvalSession::capture_baseline_from`] publishes this host's claim into it on
                // exactly this convergence. A session-free host owns no claim and therefore retires
                // nothing (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
                // `📓️slider-reevaluation-correctness-2026-09-15.md`).
                // 🔒️ Only advance the snapshot/channels/generation triple together, and only on
                // success — a failed evaluation keeps diffing against the last known-good state
                // next time, which is always a safe (never under-dirty) baseline.
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
        // 🧹️ `self.outputs` republishes every SEEDED widget's channel too, so a plain `extend`
        // displaces those seed dictionaries and lets `HashMap::insert`'s returned `Option<Dictionary>`
        // drop — and `Dictionary` fail-closes on a bare drop
        // (`🧠️neural/⚙️engine/🦀️.rs`'s `Drop`). Retire what is displaced
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
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
        let dirty = compute_dirty_set(Some(&baseline), &TreeSnapshot::capture(&tree, &seeds));
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
    pub(super) fn build_tree(&self) -> Tree {
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

    /// 📝️ Renders the compiled DAG fixture as wire-literal text.
    pub fn compiled_wire_literal(&self) -> String {
        dag_host_snapshot_to_wire_literal(&self.build_dag_host_snapshot_v1())
    }

    pub fn build_seeds(&self) -> HashMap<String, Dictionary> {
        let mut seeds = HashMap::new();
        for widget in &self.host_snapshot.widgets {
            match widget {
                Widget::InputSlider { id, value, .. } => {
                    seeds.insert(id.clone(), channel_output("number", Dictionary::with_schema("number").insert("value", NeuralValue::Atom(Atom::Decimal(*value)))));
                }
                Widget::InputNote { id, text } => {
                    seeds.insert(id.clone(), channel_output("text", Dictionary::with_schema("text").insert("value", NeuralValue::Atom(Atom::String(text.clone())))));
                }
                Widget::InputImage { id, src } => {
                    seeds.insert(id.clone(), channel_output("image", Dictionary::with_schema("image").insert("dataUrl", NeuralValue::Atom(Atom::String(src.clone())))));
                }
                _ => {}
            }
        }
        seeds
    }

    pub(super) fn apply_preview_outputs(&mut self, outputs: &HistoryFoldIndex<String, Dictionary>) {
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::OutputPreview { id, preview, .. } = widget {
                if let Some(out) = outputs.get(id) {
                    std::mem::replace(preview, out.clone()).retire_cold();
                } else if let Some(syn) = self.host_snapshot.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        std::mem::replace(preview, preview_dict_from_connection(src, &syn.from_port, &syn.to_port)).retire_cold();
                    }
                }
            }
        }
        self.sync_dag_display_from_widgets();
        self.dag.graph.fit_preview_sizes();
    }

    pub(super) fn apply_export_outputs(&mut self, outputs: &HistoryFoldIndex<String, Dictionary>) {
        for widget in &self.host_snapshot.widgets {
            if let Widget::OutputExport { id, .. } = widget {
                if let Some(out) = outputs.get(id) {
                    self.export_payloads.insert(id.clone(), out.clone());
                } else if let Some(syn) = self.host_snapshot.synapses.iter().find(|s| s.to == *id) {
                    if let Some(src) = outputs.get(&syn.from) {
                        let payload = preview_dict_from_connection(src, &syn.from_port, &syn.to_port);
                        self.export_payloads.insert(id.clone(), payload);
                    }
                }
            }
        }
    }

    pub fn export_payload_json(&self, widget_id: &str) -> Result<String, FlowCoreError> {
        let payload = self.export_payloads.get(widget_id).cloned().unwrap_or_default();
        let json = semio_framework_pack_json::to_json_string(&payload);
        payload.retire_cold();
        Ok(json)
    }

    /// 📤️ Returns and clears a pending export control click from the last pointer hit.
    pub fn take_pending_export_click(&mut self) -> Option<String> {
        self.dag.pointer.take_pending_export_click()
    }

    pub(super) fn sync_dag_display_from_widgets(&mut self) {
        for widget in &self.host_snapshot.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.graph.host_snapshot.nodes.iter_mut().find(|n| n.id == *id) else {
                continue;
            };
            match (widget, &mut node.kind) {
                (Widget::InputSlider { value, .. }, DagNodeKind::Slider { value: dag_value, .. }) => {
                    *dag_value = *value;
                }
                (Widget::InputNote { text, .. }, DagNodeKind::Note { text: dag_text, .. }) => {
                    *dag_text = text.clone();
                }
                (Widget::InputImage { src, .. }, DagNodeKind::Image { src: dag_src, .. }) => {
                    *dag_src = src.clone();
                }
                (Widget::OutputPreview { preview, expanded, .. }, DagNodeKind::Preview { content, expanded: dag_expanded, .. }) => {
                    *content = dag_preview_content_from_dict(preview);
                    *dag_expanded = expanded.iter().cloned().collect();
                }
                (Widget::OutputAction { action, .. }, DagNodeKind::Action { label, .. }) => {
                    *label = action.clone();
                }
                (Widget::OutputExport { format, .. }, DagNodeKind::Export { label, format: dag_format, .. }) => {
                    *label = format.to_uppercase();
                    *dag_format = format.clone();
                }
                _ => {}
            }
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn sync_dag_ghost(&mut self) {
        self.dag.pointer.set_ghost_node(self.ghost_node.clone());
    }

    pub fn rebuild_dag(&mut self) {
        let fixture = self.build_dag_host_snapshot_v1();
        if self.pending_change || self.gesture_active {
            let kept: BTreeSet<&str> = fixture.edges.iter().map(|edge| edge.id.as_str()).collect();
            let dropped: Vec<FlowMutation> = self.host_snapshot.synapses.iter().filter(|synapse| !kept.contains(synapse.id.as_str())).map(|synapse| FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse.id.clone() })).collect();
            self.pending_leaves.extend(dropped);
        }
        let theme = self.dag.viewport.canvas_theme;
        let automatic_lod = self.dag.viewport.automatic_lod();
        let forced_draw_lod = self.dag.viewport.forced_draw_lod_label().map(str::to_string);
        let ghost = self.ghost_node.clone();
        self.dag.replace_host_snapshot_without_layout(fixture);
        self.dag.viewport.canvas_theme = theme;
        self.dag.viewport.set_viewport(self.viewport_w, self.viewport_h, self.viewport_dpr);
        self.dag.viewport.set_automatic_lod(automatic_lod);
        if let Some(label) = forced_draw_lod {
            self.dag.viewport.set_forced_draw_lod_label(&label);
        }
        self.dag.pointer.set_ghost_node(ghost);
        self.dag.minimap.set_minimap_widget_visible(true);
        self.sync_preview_dimmed();
        self.sync_from_dag();
    }

    fn sync_preview_dimmed(&mut self) {
        let off = self.preview_off_widget_ids();
        set_dimmed(&mut self.dag.graph, &mut self.dag.paint, &off);
    }

    /// 🎯️ Projects selected semantic widget identities.
    pub fn selected_widget_ids(&self)->Vec<String>{self.dag.graph.selected_node_ids()}

    /// 🎯️ Projects typed node, edge and handle selection.
    pub fn selection_domains(&self)->DagSelectionDomains{self.dag.graph.selection_domains()}

    /// 🖱️ Hovered widget id when the pointer is over a node or port handle.
    pub fn hovered_widget_id(&self) -> Option<String> {
        hovered_node_id(&self.dag.viewport, &self.dag.graph)
    }

    /// 🎯️ All pick targets under a screen point as JSON for DOM disambiguation menus.
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        dag_pick_targets_at_screen_json(&self.dag.viewport, &self.dag.graph, sx, sy)
    }

    /// 🎯️ Screen-space geometry for a live entity (`domain`/`id` in the pick-target grammar) —
    /// see `DagHost::entity_screen_json`. Powers introduction-demonstration semantic targeting.
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        entity_screen_json(&self.dag.viewport, &self.dag.graph, domain, id)
    }

    /// 🖱️ Projects admitted widget hover and live wire refusal facts.
    pub fn hover_facts(&self)->semio_framework_os_infinite::board::schema::dag_input::DagHoverFacts{self.dag.graph.hover_facts()}

    /// 🔌️ Projects selected typed widget channels.
    pub fn selected_channels(&self)->Vec<dag::DagChannelRef>{self.dag.graph.selected_channels()}

    /// ✅️ Replaces selection from admitted semantic domains.
    pub fn set_selection_domains(&mut self,domains:&DagSelectionDomains){self.dag.graph.set_selection_domains(domains);}

    /// ✅️ Replaces the node domain selection directly.
    pub fn set_selection(&mut self,ids:&[String]){self.dag.graph.set_selection(ids);}

    /// 📦️ Screen-space union bounds of the current selection for DOM overlays.
    pub fn selection_union_bounds_screen_json(&self) -> String {
        selection_union_bounds_screen_json(&self.dag.viewport, &self.dag.graph)
    }

    /// 📐️ Aligns or distributes the selection and journals what it moved as node-graph gesture records (design §13.3,
    /// one per distinct offset), which the renderer drains ([`Self::take_graph_edits_json`]) and dispatches; an align whose
    /// records outgrow one dispatch is refused whole and every node stays where it was.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), FlowCoreError> {
        self.begin_change();
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        let baseline = self.dag.graph.node_positions();
        self.dag.graph.align_selection(mode)?;
        let refused = self.dag.graph.journal_moves_since(&dag::dag_drag_gesture_id(self.interaction_revision), &baseline).is_err();
        if refused {
            self.dag.graph.restore_node_positions(&baseline);
        }
        self.sync_from_dag();
        if !refused {
            let aligned = self.dag.graph.selected_node_ids();
            let moved = self.layout_leaf(aligned.iter().map(String::as_str));
            self.note_leaves(moved);
        }
        self.checked_change()?;
        Ok(())
    }

    /// 🖱️ Sets hover to a widget id, or clears hover.
    pub fn set_hover(&mut self, widget_id: Option<&str>) {
        self.dag.graph.set_hover(widget_id);
    }

    /// 🔌️ Sets hover to a widget channel, or clears hover.
    pub fn set_hover_channel(&mut self, widget_id: Option<&str>, port_id: Option<&str>) {
        set_hover_channel(&mut self.dag.viewport, &mut self.dag.graph, widget_id, port_id);
    }

    /// 🔌️ Replaces admitted typed channel selection.
    pub fn set_selected_channels(&mut self,channels:&[dag::DagChannelRef]){set_selected_channels(&mut self.dag.viewport, &mut self.dag.graph, channels);}

    /// 🌫️ Widget ids with preview disabled.
    pub fn preview_off_widget_ids(&self) -> Vec<String> {
        self.host_snapshot
            .widgets
            .iter()
            .filter_map(|widget| match widget {
                Widget::Neuron { id, preview: false, .. } => Some(id.clone()),
                _ => None,
            })
            .collect()
    }

    /// 🌫️ Sets preview-off neurons from a JSON array of widget ids.
    pub fn set_preview_off_json(&mut self, json: &str) {
        let ids: Vec<String> = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::Neuron { id, preview, .. } = widget {
                *preview = !ids.contains(id);
            }
        }
        self.sync_preview_dimmed();
    }

    /// 👁️ Toggles preview on a neuron widget.
    pub fn toggle_preview(&mut self, widget_id: &str) -> Result<(), FlowCoreError> {
        let Some(widget) = self.host_snapshot.widgets.iter_mut().find(|w| widget_id_for(w) == widget_id) else {
            return Err(FlowCoreError::UnknownWidget(widget_id.to_string()));
        };
        let Widget::Neuron { preview, .. } = widget else {
            return Err(FlowCoreError::WidgetNotNeuron(widget_id.to_string()));
        };
        *preview = !*preview;
        self.sync_preview_dimmed();
        Ok(())
    }

    pub(super) fn sync_from_dag(&mut self) {
        self.evaluation_revision=self.evaluation_revision.wrapping_add(1);
        let dag_ids: BTreeSet<String> = self.dag.graph.host_snapshot.nodes.iter().map(|node| node.id.clone()).collect();
        self.host_snapshot.widgets.retain(|widget| dag_ids.contains(widget_id_for(widget)));
        for node in &self.dag.graph.host_snapshot.nodes {
            self.host_snapshot.layout.insert(node.id.clone(), WidgetLayout { x: node.x, y: node.y });
        }
        for widget in &mut self.host_snapshot.widgets {
            let id = widget_id_for(widget);
            let Some(node) = self.dag.graph.host_snapshot.nodes.iter().find(|n| n.id == *id) else {
                continue;
            };
            match (widget, &node.kind) {
                (Widget::InputSlider { value, .. }, DagNodeKind::Slider { value: dag_value, .. }) => {
                    *value = *dag_value;
                }
                (Widget::InputNote { text, .. }, DagNodeKind::Note { text: dag_text, .. }) => {
                    *text = dag_text.clone();
                }
                (Widget::InputImage { src, .. }, DagNodeKind::Image { src: dag_src, .. }) => {
                    *src = dag_src.clone();
                }
                (Widget::OutputPreview { expanded, .. }, DagNodeKind::Preview { expanded: dag_expanded, .. }) => {
                    std::mem::replace(expanded, dag_expanded.iter().cloned().collect()).retire_cold();
                }
                (Widget::OutputAction { action, .. }, DagNodeKind::Action { label, .. }) => {
                    *action = label.clone();
                }
                (Widget::OutputExport { format, .. }, DagNodeKind::Export { format: dag_format, .. }) => {
                    *format = dag_format.clone();
                }
                _ => {}
            }
        }
        self.host_snapshot.synapses = self
            .dag.graph
            .host_snapshot
            .edges
            .iter()
            .map(|edge| {
                let (from, from_port) = parse_port_endpoint(&edge.source, "");
                let (to, to_port) = parse_port_endpoint(&edge.target, "");
                SynapseSpec { id: edge.id.clone(), from, to, from_port, to_port }
            })
            .collect();
        self.host_snapshot.camera = CameraJson { x: self.dag.graph.host_snapshot.camera.x, y: self.dag.graph.host_snapshot.camera.y, zoom: self.dag.graph.host_snapshot.camera.zoom };
    }

    pub(super) fn build_dag_host_snapshot_v1(&self) -> DagHostSnapshot {
        let mut seen = BTreeSet::new();
        let nodes: Vec<DagNodeSpec> =
            self.host_snapshot.widgets.iter().enumerate().filter(|(_, widget)| seen.insert(widget_id_for(widget).to_string())).map(|(i, w)| widget_to_dag_node(w, i, &self.host_snapshot.layout, &self.host_snapshot.synapses, &self.kind_infos, widget_node_size(w, &self.host_snapshot.synapses, &self.kind_infos))).collect();
        let existing: Vec<(String, String)> = self.host_snapshot.synapses.iter().map(|s| (s.from.clone(), s.to.clone())).collect();
        let edges: Vec<DagHostSnapshotEdge> = self
            .host_snapshot
            .synapses
            .iter()
            .filter_map(|syn| {
                if would_create_cycle(&existing.iter().filter(|(a, b)| !(a == &syn.from && b == &syn.to)).cloned().collect::<Vec<_>>(), &syn.from, &syn.to) {
                    return None;
                }
                let from_port = resolve_synapse_port(&syn.from, &syn.from_port, PortSide::Output, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos).unwrap_or_else(|| syn.from_port.clone());
                let to_port = resolve_synapse_port(&syn.to, &syn.to_port, PortSide::Input, &self.host_snapshot.widgets, &self.host_snapshot.synapses, &self.kind_infos).unwrap_or_else(|| syn.to_port.clone());
                Some(DagHostSnapshotEdge { id: syn.id.clone(), source: format!("{}@{}", syn.from, from_port), target: format!("{}@{}", syn.to, to_port), route_style: EdgeRouteStyle::default(), properties: PropertyBag::new() })
            })
            .collect();
        DagHostSnapshot { schema: "dag.hostDocument".into(), camera: semio_framework_os_infinite_directed_presentation::DagCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom }, nodes, edges }
    }

    pub(super) fn screen_to_world_point(&self, sx: f64, sy: f64) -> canvas::Point {
        use canvas::camera::{screen_to_world, Camera, Viewport};
        use canvas::Point;
        let cam = Camera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.viewport_w, height: self.viewport_h, dpr: self.viewport_dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    pub(super) fn next_widget_id(&mut self, descriptor: &WidgetDescriptor) -> String {
        let (id, serial) = generated_widget_id(descriptor, self.host_snapshot.widgets.iter().map(widget_id_for));
        self.next_widget_serial = serial;
        id
    }

    pub fn set_slider_value(&mut self, widget_id: &str, value: f64) {
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::InputSlider { id, .. } = widget {
                if id == widget_id {
                    set_widget_slider_value(widget, value);
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.refresh_computing_chrome_from_pending();
        self.finish_change();
    }

    pub fn slider_overlay_state_json(&self) -> Result<String, FlowCoreError> {
        Ok(slider_overlay_state_json(&self.dag.viewport, &self.dag.graph)?)
    }

    pub fn set_note_text(&mut self, widget_id: &str, text: &str) {
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::InputNote { id, text: note } = widget {
                if id == widget_id {
                    *note = text.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.dag.graph.fit_note_sizes();
        self.refresh_computing_chrome_from_pending();
        self.finish_change();
    }

    /// ✏️ Begins inline note editing for a widget at a world-space click.
    pub fn begin_note_edit(&mut self, widget_id: &str, world_x: f64, world_y: f64) {
        self.begin_gesture();
        self.edited_note = Some(widget_id.to_string());
        begin_note_edit(&mut self.dag.graph, &mut self.dag.notes, widget_id, world_x, world_y);
    }

    /// ✏️ Inserts text into the active note editor.
    pub fn note_insert_text(&mut self, chunk: &str) {
        if !note_insert_text(&mut self.dag.graph, &mut self.dag.notes, chunk) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Backspaces in the active note editor.
    pub fn note_backspace(&mut self) {
        if !note_backspace(&mut self.dag.graph, &mut self.dag.notes) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Deletes forward in the active note editor.
    pub fn note_delete_forward(&mut self) {
        if !note_delete_forward(&mut self.dag.graph, &mut self.dag.notes) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Moves the active note caret.
    pub fn note_move_caret(&mut self, direction: &str, extend: bool) {
        if !note_move_caret(&mut self.dag.graph, &mut self.dag.notes, direction, extend) {
            return;
        }
        self.sync_from_dag();
    }

    /// ✏️ Commits inline note editing into fixture history.
    pub fn note_commit_edit(&mut self) {
        self.dag.notes.note_commit_edit();
        self.sync_from_dag();
        self.commit_gesture_history();
    }

    /// ✏️ Toggles native caret visibility while editing a note.
    pub fn set_note_caret_visible(&mut self, visible: bool) {
        self.dag.notes.set_note_caret_visible(visible);
    }

    pub fn schemas_json(&self) -> Result<String, FlowCoreError> {
        let refs = self.operator_registry().schema_refs();
        Ok(semio_framework_pack_json::to_json_string(&refs))
    }

    pub fn set_variable_name(&mut self, widget_id: &str, name: &str) {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::Variable { id, name: variable_name, .. } = widget {
                if id == widget_id {
                    *variable_name = trimmed.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.rebuild_dag();
        self.finish_change();
    }

    pub fn set_variable_schema(&mut self, widget_id: &str, schema: &str) {
        let trimmed = schema.trim();
        if trimmed.is_empty() {
            return;
        }
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::Variable { id, schema: variable_schema, .. } = widget {
                if id == widget_id {
                    *variable_schema = trimmed.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.rebuild_dag();
        self.finish_change();
    }

    pub fn set_image_src(&mut self, widget_id: &str, src: &str) {
        self.begin_change();
        for widget in &mut self.host_snapshot.widgets {
            if let Widget::InputImage { id, src: image } = widget {
                if id == widget_id {
                    *image = src.to_string();
                }
            }
        }
        let changed = self.changed_widget_leaf(widget_id);
        self.note_leaves(changed);
        self.sync_dag_display_from_widgets();
        self.dag.graph.fit_preview_sizes();
        self.refresh_computing_chrome_from_pending();
        self.finish_change();
    }

    pub fn preview_text(&self) -> String {
        self.host_snapshot
            .widgets
            .iter()
            .find_map(|w| match w {
                Widget::OutputPreview { preview, .. } => Some(preview_content_summary(&dag_preview_content_from_dict(preview))),
                _ => None,
            })
            .unwrap_or_else(|| "—".into())
    }

    pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), FlowCoreError> {
        let mut accepted = |_| true;
        let mut control = semio_framework_value::NativeDecodeControl::new(64 * 1024, &mut accepted);
        let overlay = semio_framework_os_infinite::board::io::text::palette::decode_board_palette_overlay_json(json, &mut control).map_err(|error| FlowCoreError::Json(error.into_message()))?;
        dag_set_canvas_palette(&mut self.dag.viewport, &mut self.dag.paint, &overlay);
        Ok(())
    }

    pub fn set_canvas_theme_dark(&mut self, dark: bool) {
        self.dag.viewport.canvas_theme = dag::CanvasPalette::from_board_palette(if dark { &ui_styling::BOARD_DARK } else { &ui_styling::BOARD_LIGHT });
    }

    pub fn paint_scene(&self, scene: &mut canvas::Scene, width: u32, height: u32, dpr: f64) {
        self.dag.paint_scene(scene, width, height, dpr);
    }

    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.dag.viewport.set_automatic_lod(enabled);
    }

    pub fn set_proximity_distance(&mut self, world: f64) {
        self.dag.graph.set_proximity_distance(world);
    }

    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        self.dag.viewport.set_forced_draw_lod_label(label);
    }

    pub fn set_grid_visible(&mut self, visible: bool) {
        self.dag.viewport.set_grid_visible(visible);
    }

    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.dag.viewport.set_grid_snap_enabled(enabled);
    }

    pub fn set_grid_factor(&mut self, factor: f64) -> Result<(), FlowCoreError> {
        self.dag.viewport.set_grid_factor(factor)?;
        Ok(())
    }

    pub fn focus_selection_camera(&self, pad: f64) -> Option<CameraJson> {
        focus_selection_camera(&self.dag.viewport, &self.dag.graph, pad).map(|camera| CameraJson { x: camera.x, y: camera.y, zoom: camera.zoom })
    }

    pub fn draw_lod_label(&self) -> &'static str {
        dag_draw_lod_label(&self.dag.viewport, &self.dag.graph)
    }

    pub fn label_overlay_paint_state_json(&self) -> Result<String, FlowCoreError> {
        Ok(self.dag.label_overlay_paint_state_json()?)
    }

    /// 💥️ Returns and clears a pending cluster explode target from the last pointer hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.dag.pointer.take_pending_cluster_explode()
    }

    /// 🧩️ Collapses the selected widgets into one cluster neuron.
    pub fn collapse_selection(&mut self, selected_ids: &[String]) -> Result<String, FlowCoreError> {
        if selected_ids.len() < 2 {
            return Err(FlowCoreError::CollapseNeedsTwoWidgets);
        }
        let selected: BTreeSet<String> = selected_ids.iter().cloned().collect();
        if !selected.iter().all(|id| self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id)) {
            return Err(FlowCoreError::CollapseUnknownWidgets);
        }
        if selected.iter().any(|id| self.host_snapshot.widgets.iter().any(|widget| widget_id_for(widget) == id && matches!(widget, Widget::Cluster { .. }))) {
            return Err(FlowCoreError::CollapseContainsClusters);
        }
        self.begin_change();
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| selected.contains(&synapse.from) || selected.contains(&synapse.to)).map(|synapse| synapse.id.clone()).collect();
        let unplaced: Vec<FlowLayoutEntry> = selected.iter().filter(|id| self.host_snapshot.layout.contains_key(id.as_str())).map(|id| FlowLayoutEntry { id: id.clone(), layout: None }).collect();
        let mut crossing_external = Vec::new();
        for synapse in &self.host_snapshot.synapses {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if (from_selected || to_selected) && !(from_selected && to_selected) {
                crossing_external.push(synapse.clone());
            }
        }
        let boundary_variables = boundary_variable_widget_ids(&selected, &crossing_external, &self.host_snapshot.widgets);
        let mut inner_neurons = Vec::new();
        let mut inner_layout = BTreeMap::new();
        for widget in &self.host_snapshot.widgets {
            let id = widget_id_for(widget).to_string();
            if !selected.contains(&id) {
                continue;
            }
            if boundary_variables.contains(&id) {
                continue;
            }
            if let Some(neuron) = widget_to_inner_neuron(widget) {
                inner_neurons.push(neuron);
            }
            if let Some(layout) = self.host_snapshot.layout.get(&id) {
                inner_layout.insert(id, layout.clone());
            }
        }
        let mut inner_synapses = Vec::new();
        let mut retained_external = Vec::new();
        for synapse in &self.host_snapshot.synapses {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if from_selected && to_selected {
                if boundary_variables.contains(&synapse.from) || boundary_variables.contains(&synapse.to) {
                    continue;
                }
                inner_synapses.push(Synapse { id: synapse.id.clone(), from: synapse.from.clone(), to: synapse.to.clone(), from_port: synapse.from_port.clone(), to_port: synapse.to_port.clone() });
            } else if from_selected || to_selected {
            } else {
                retained_external.push(synapse.clone());
            }
        }
        let mut used_channels = BTreeSet::new();
        let mut input_serial = 0usize;
        let mut output_serial = 0usize;
        let mut boundary_index = 0usize;
        let mut cluster_external = Vec::new();
        let channels=self.current_channels.clone().or_else(||self.previous_channels.clone());
        let empty_outputs=HistoryFoldIndex::new();
        let outputs=channels.as_deref().map_or(&empty_outputs,|channels|&channels.outputs);
        let kind_infos = self.kind_infos.clone();
        let widgets = self.host_snapshot.widgets.clone();
        let synapses_snapshot = self.host_snapshot.synapses.clone();
        for synapse in crossing_external {
            let from_selected = selected.contains(&synapse.from);
            let to_selected = selected.contains(&synapse.to);
            if to_selected && !from_selected {
                let inner_target = if boundary_variables.contains(&synapse.to) {
                    self.host_snapshot.synapses.iter().find(|entry| entry.from == synapse.to && selected.contains(&entry.to)).map_or_else(|| (synapse.to.clone(), synapse.to_port.clone()), |entry| (entry.to.clone(), entry.to_port.clone()))
                } else {
                    (synapse.to.clone(), synapse.to_port.clone())
                };
                let (channel, schema) = if let Some((name, schema)) = variable_widget_meta(&widgets, &synapse.to) {
                    let schema = if schema.is_empty() { infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &synapse.from, &synapse.from_port) } else { schema };
                    (name, schema)
                } else {
                    let channel = unique_generated_boundary_name("input", &mut input_serial, &used_channels);
                    let schema = infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &synapse.from, &synapse.from_port);
                    (channel, schema)
                };
                used_channels.insert(channel.clone());
                boundary_index += 1;
                let boundary_id = format!("__in_{boundary_index}");
                inner_neurons.push(Neuron::with_kind(&boundary_id, INPUT_KIND, contract_boundary_params(&channel, &schema)));
                inner_synapses.push(Synapse { id: format!("{boundary_id}_link"), from: boundary_id, to: inner_target.0, from_port: String::new(), to_port: inner_target.1 });
                cluster_external.push(SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: String::new(), from_port: synapse.from_port.clone(), to_port: channel });
            } else if from_selected && !to_selected {
                let inner_source = if boundary_variables.contains(&synapse.from) {
                    self.host_snapshot.synapses.iter().find(|entry| entry.to == synapse.from && selected.contains(&entry.from)).map_or_else(|| (synapse.from.clone(), synapse.from_port.clone()), |entry| (entry.from.clone(), entry.from_port.clone()))
                } else {
                    (synapse.from.clone(), synapse.from_port.clone())
                };
                let (channel, schema) = if let Some((name, schema)) = variable_widget_meta(&widgets, &synapse.from) {
                    let schema = if schema.is_empty() { infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &inner_source.0, &inner_source.1) } else { schema };
                    (name, schema)
                } else {
                    let channel = unique_generated_boundary_name("output", &mut output_serial, &used_channels);
                    let schema = infer_port_schema(&outputs, &kind_infos, &widgets, &synapses_snapshot, &inner_source.0, &inner_source.1);
                    (channel, schema)
                };
                used_channels.insert(channel.clone());
                boundary_index += 1;
                let boundary_id = format!("__out_{boundary_index}");
                inner_neurons.push(Neuron::with_kind(&boundary_id, OUTPUT_KIND, contract_boundary_params(&channel, &schema)));
                inner_synapses.push(Synapse { id: format!("{boundary_id}_link"), from: inner_source.0, to: boundary_id, from_port: inner_source.1, to_port: String::new() });
                cluster_external.push(SynapseSpec { id: synapse.id.clone(), from: String::new(), to: synapse.to.clone(), from_port: channel, to_port: synapse.to_port.clone() });
            }
        }
        let (sum_x, sum_y, layout_count) = selected.iter().filter_map(|id| self.host_snapshot.layout.get(id)).fold((0.0, 0.0, 0usize), |(sx, sy, count), layout| (sx + layout.x, sy + layout.y, count + 1));
        let count = layout_count.max(1) as f64;
        let cluster_x = sum_x / count;
        let cluster_y = sum_y / count;
        self.next_widget_serial += 1;
        let cluster_id = format!("cluster_{}", self.next_widget_serial);
        let inner_tree = Tree { neurons: inner_neurons, synapses: inner_synapses };
        let cluster = Widget::Cluster {
            id: cluster_id.clone(),
            name: "Cluster".into(),
            tree: inner_tree,
            flow: FlowGui { camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: inner_layout.into_iter().map(|(id, layout)| (id, FlowNodeGui { layout, chrome: NodeChrome::Plain { preview: true } })).collect(), previews: vec![] },
        };
        self.host_snapshot.widgets.retain(|widget| !selected.contains(widget_id_for(widget)));
        let clustered = FlowMutation::AddWidget(AddWidget { index: u32::try_from(self.host_snapshot.widgets.len()).unwrap_or(u32::MAX), widget: cluster.clone() });
        self.host_snapshot.widgets.push(cluster);
        for id in &selected {
            self.host_snapshot.layout.remove(id);
        }
        self.host_snapshot.layout.insert(cluster_id.clone(), WidgetLayout { x: cluster_x, y: cluster_y });
        let retained_count = retained_external.len();
        self.host_snapshot.synapses = retained_external;
        for synapse in cluster_external {
            if synapse.to.is_empty() {
                self.host_snapshot.synapses.push(SynapseSpec { id: synapse.id, from: synapse.from, to: cluster_id.clone(), from_port: synapse.from_port, to_port: synapse.to_port });
            } else {
                self.host_snapshot.synapses.push(SynapseSpec { id: synapse.id, from: cluster_id.clone(), to: synapse.to, from_port: synapse.from_port, to_port: synapse.to_port });
            }
        }
        let unplaced = (!unplaced.is_empty()).then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: unplaced }));
        let rewired: Vec<FlowMutation> = self.host_snapshot.synapses[retained_count..].iter().enumerate().map(|(offset, synapse)| FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(retained_count + offset).unwrap_or(u32::MAX), synapse: synapse.clone() })).collect();
        let placed = self.layout_leaf([cluster_id.as_str()]);
        self.note_leaves(
            severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain(selected.iter().map(|id| FlowMutation::RemoveWidget(RemoveWidget { id: id.clone() }))).chain([clustered]).chain(placed).chain(rewired),
        );
        self.rebuild_dag();
        self.checked_change()?;
        Ok(cluster_id)
    }

    /// 💥️ Explodes a cluster back into its inner widgets.
    pub fn explode_cluster(&mut self, cluster_id: &str) -> Result<(), FlowCoreError> {
        let cluster_index = self.host_snapshot.widgets.iter().position(|widget| matches!(widget, Widget::Cluster { id, .. } if id == cluster_id)).ok_or_else(|| FlowCoreError::UnknownCluster(cluster_id.to_string()))?;
        // 🧹️ The working copy is BORROWED from one retired clone, not destructured out of it: a
        // cluster's `Tree` params and its `FlowUi` node map both fail closed on a bare drop, so
        // owning `tree`/`flow` as loose locals aborted the process at the end of this function
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        let exploded = self.host_snapshot.widgets[cluster_index].clone();
        let Widget::Cluster { tree, flow, .. } = &exploded else {
            exploded.retire_cold();
            return Err(FlowCoreError::WidgetNotCluster(cluster_id.to_string()));
        };
        let cluster_layout = self.host_snapshot.layout.get(cluster_id).cloned().unwrap_or(WidgetLayout { x: 0.0, y: 0.0 });
        self.begin_change();
        let severed: Vec<String> = self.host_snapshot.synapses.iter().filter(|synapse| synapse.from == cluster_id || synapse.to == cluster_id).map(|synapse| synapse.id.clone()).collect();
        let had_layout = self.host_snapshot.layout.contains_key(cluster_id);
        let mut boundary_channels: HashMap<String, (String, String)> = HashMap::new();
        for neuron in &tree.neurons {
            if neuron.kind == INPUT_KIND {
                let channel = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
                boundary_channels.insert(channel.clone(), (format!("{cluster_id}/{}", neuron.id), channel));
            } else if neuron.kind == OUTPUT_KIND {
                let channel = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
                boundary_channels.insert(channel.clone(), (format!("{cluster_id}/{}", neuron.id), channel));
            }
        }
        let mut restored_widgets = Vec::new();
        for neuron in &tree.neurons {
            let namespaced_id = format!("{cluster_id}/{}", neuron.id);
            if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                let widget = neuron_to_exploded_widget(neuron);
                let widget = match widget {
                    Widget::Variable { name, schema, .. } => Widget::Variable { id: namespaced_id.clone(), name, schema },
                    other => other,
                };
                let layout = flow.nodes.get(&neuron.id).map_or(WidgetLayout { x: 0.0, y: 0.0 }, |node| node.layout.clone());
                self.host_snapshot.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
                restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
                continue;
            }
            let mut widget = neuron_to_exploded_widget(neuron);
            match &mut widget {
                Widget::Neuron { id, .. } | Widget::InputSlider { id, .. } | Widget::InputNote { id, .. } | Widget::InputImage { id, .. } | Widget::Variable { id, .. } => *id = namespaced_id.clone(),
                _ => {}
            }
            let layout = flow.nodes.get(&neuron.id).map_or(WidgetLayout { x: 0.0, y: 0.0 }, |node| node.layout.clone());
            self.host_snapshot.layout.insert(namespaced_id.clone(), WidgetLayout { x: cluster_layout.x + layout.x, y: cluster_layout.y + layout.y });
            restored_widgets.push((namespaced_id, neuron.id.clone(), widget));
        }
        let id_map: HashMap<String, String> = restored_widgets.iter().map(|(namespaced, original, _)| (original.clone(), namespaced.clone())).collect();
        // 🧹️ The cluster widget carries a `FlowUi` whose `OrderedMap<FlowNodeGui>` and a `Tree`
        // whose `Dictionary` params both fail closed on a bare drop, so the exploded shell is
        // RETIRED rather than dropped (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        self.host_snapshot.widgets.remove(cluster_index).retire_cold();
        self.host_snapshot.layout.remove(cluster_id);
        let mut restored_ids = Vec::with_capacity(restored_widgets.len());
        let mut restored_leaves = Vec::with_capacity(restored_widgets.len());
        for (namespaced_id, _, widget) in restored_widgets {
            restored_leaves.push(FlowMutation::AddWidget(AddWidget { index: u32::try_from(self.host_snapshot.widgets.len()).unwrap_or(u32::MAX), widget: widget.clone() }));
            restored_ids.push(namespaced_id);
            self.host_snapshot.widgets.push(widget);
        }
        let mut next_synapses = Vec::new();
        let mut rewired = Vec::new();
        for synapse in &self.host_snapshot.synapses {
            if synapse.to == cluster_id {
                if let Some((variable_id, variable_port)) = boundary_channels.get(&synapse.to_port) {
                    rewired.push(next_synapses.len());
                    next_synapses.push(SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: variable_id.clone(), from_port: synapse.from_port.clone(), to_port: variable_port.clone() });
                    continue;
                }
            } else if synapse.from == cluster_id {
                if let Some((variable_id, variable_port)) = boundary_channels.get(&synapse.from_port) {
                    rewired.push(next_synapses.len());
                    next_synapses.push(SynapseSpec { id: synapse.id.clone(), from: variable_id.clone(), to: synapse.to.clone(), from_port: variable_port.clone(), to_port: synapse.to_port.clone() });
                    continue;
                }
            } else {
                next_synapses.push(synapse.clone());
            }
        }
        for synapse in &tree.synapses {
            let Some(from) = id_map.get(&synapse.from) else { continue };
            let Some(to) = id_map.get(&synapse.to) else { continue };
            let from_port = tree
                .neurons
                .iter()
                .find(|neuron| neuron.id == synapse.from && neuron.kind == INPUT_KIND)
                .and_then(|neuron| neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str())).map_or_else(|| synapse.from_port.clone(), str::to_string);
            let to_port = tree
                .neurons
                .iter()
                .find(|neuron| neuron.id == synapse.to && neuron.kind == OUTPUT_KIND)
                .and_then(|neuron| neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str())).map_or_else(|| synapse.to_port.clone(), str::to_string);
            let id = self.next_synapse_id();
            rewired.push(next_synapses.len());
            next_synapses.push(SynapseSpec { id, from: from.clone(), to: to.clone(), from_port, to_port });
        }
        let reconnected: Vec<FlowMutation> = rewired.into_iter().map(|index| FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(index).unwrap_or(u32::MAX), synapse: next_synapses[index].clone() })).collect();
        self.host_snapshot.synapses = next_synapses;
        exploded.retire_cold();
        let unplaced = had_layout.then(|| FlowMutation::ChangeLayout(ChangeLayout { entries: vec![FlowLayoutEntry { id: cluster_id.to_string(), layout: None }] }));
        let placed = self.layout_leaf(restored_ids.iter().map(String::as_str));
        self.note_leaves(
            severed.into_iter().map(|id| FlowMutation::RemoveSynapse(RemoveSynapse { id })).chain(unplaced).chain([FlowMutation::RemoveWidget(RemoveWidget { id: cluster_id.to_string() })]).chain(restored_leaves).chain(placed).chain(reconnected),
        );
        self.rebuild_dag();
        self.checked_change()?;
        Ok(())
    }

    // #region History
    /// 🧾️ Lazily seeds the undo/redo store from `baseline`.
    ///
    /// ⚠️ `baseline` is CONSUMED only on the first call — the store is seeded once, and every later
    /// `flush_pending_change` hands in a fresh `FlowHostSnapshot` clone this function does not need. A
    /// `FlowHostSnapshot` owns the fail-closed `OrderedMap<WidgetLayout>` root, so that surplus clone is
    /// RETIRED through the artifact's own bounded frontier instead of dropped; the bare drop aborted
    /// the pool worker on the second discrete edit of any session
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️unit-suite-3d-2026-09-09.md` §5).
    pub fn history_store_from_baseline(&mut self, baseline: FlowHostSnapshot) -> Option<&mut FlowStore> {
        if self.history_store.is_some() {
            baseline.retire_cold();
            return self.history_store.as_mut();
        }
        let Some(producer)=self.history_store_factory.as_ref() else{baseline.retire_cold();return None};
        let store=(producer.construct)(baseline).ok()?;
        self.history_store = Some(store);
        self.history_store.as_mut()
    }

    /// 🧾️ Appends the concrete leaves a gesture site just emitted to the edit in progress.
    pub fn note_leaves(&mut self, leaves: impl IntoIterator<Item = FlowMutation>) {
        self.pending_leaves.extend(leaves);
    }

    /// 🗑️ Retires the leaves of an edit that is not recorded; a widget leaf owns fail-closed roots that refuse a bare drop.
    pub(super) fn discard_pending_leaves(&mut self) {
        std::mem::take(&mut self.pending_leaves).into_iter().for_each(retire_flow_mutation);
    }

    /// 🩹 The `change-widget` leaf carrying `widget_id`'s current content, when the widget exists.
    pub(super) fn changed_widget_leaf(&self, widget_id: &str) -> Option<FlowMutation> {
        self.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == widget_id).map(|widget| FlowMutation::ChangeWidget(ChangeWidget { id: widget_id.to_string(), widget: widget.clone() }))
    }

    /// 📐️ The `change-layout` leaf assigning each of `ids` its current layout; an id without a layout is skipped.
    pub(super) fn layout_leaf<'a>(&self, ids: impl IntoIterator<Item = &'a str>) -> Option<FlowMutation> {
        let entries: Vec<FlowLayoutEntry> = ids.into_iter().filter_map(|id| self.host_snapshot.layout.get(id).map(|layout| FlowLayoutEntry { id: id.to_string(), layout: Some(layout.clone()) })).collect();
        (!entries.is_empty()).then(|| FlowMutation::ChangeLayout(ChangeLayout { entries }))
    }

    /// 🔗️ The leaves of the wire, slider and move edits the open pointer gesture journalled in the dag (the gesture's own
    /// narration, read without draining); a port insert is recorded where `add_*_port` runs, and a connect that displaced
    /// the wire already feeding its target removes it first.
    fn gesture_journal_leaves(&self, baseline: &FlowHostSnapshot) -> Vec<FlowMutation> {
        let edits = self.dag.graph.graph_edits();
        self.journal_leaves(edits.get(self.journal_mark..).unwrap_or(edits), baseline)
    }

    /// 🔗️ The leaves one gesture's journal `rows` stand for, given the `baseline` before the gesture and the live content after it.
    pub fn journal_leaves(&self, rows: &[dag::DagGraphEdit], baseline: &FlowHostSnapshot) -> Vec<FlowMutation> {
        let mut leaves = Vec::new();
        for edit in rows {
            match edit {
                dag::DagGraphEdit::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                    let displaced = baseline.synapses.iter().filter(|synapse| synapse.to == *target_node_id && synapse.to_port == *target_port_id && self.host_snapshot.synapses.iter().all(|now| now.id != synapse.id));
                    leaves.extend(displaced.map(|synapse| FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse.id.clone() })));
                    let created = self.host_snapshot.synapses.iter().enumerate().find(|(_, synapse)| synapse.from == *source_node_id && synapse.from_port == *source_port_id && synapse.to == *target_node_id && synapse.to_port == *target_port_id);
                    leaves.extend(created.map(|(index, synapse)| FlowMutation::AddSynapse(AddSynapse { index: u32::try_from(index).unwrap_or(u32::MAX), synapse: synapse.clone() })));
                }
                dag::DagGraphEdit::Disconnect { synapse_id } => {
                    if baseline.synapses.iter().any(|synapse| synapse.id == *synapse_id) {
                        leaves.push(FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse_id.clone() }));
                    }
                }
                dag::DagGraphEdit::Move { node_ids, .. } => leaves.extend(self.layout_leaf(node_ids.iter().map(String::as_str))),
                dag::DagGraphEdit::SetSlider { node_id, .. } => leaves.extend(self.changed_widget_leaf(node_id)),
                dag::DagGraphEdit::InsertPort { .. } => {}
            }
        }
        leaves
    }

    /// 🧾️ Records the leaves emitted since the edit was armed as ONE store transaction (seeding the store from `baseline` on
    /// first use), retiring them cold when there is nothing to record; an edit that emitted no leaf leaves no history row.
    /// A refused transaction is never ignored: the history restarts at the live content (no partial row survives) and the
    /// refusal is kept as [`FlowCoreError::HistoryRefused`] for [`FlowHost::take_history_fault`].
    pub fn record_history_edit(&mut self, baseline: FlowHostSnapshot) {
        let operations = std::mem::take(&mut self.pending_leaves);
        #[cfg(test)]
        {
            if !operations.is_empty() {
                self.recorded.push(operations.clone());
            }
        }
        let recorded = !operations.is_empty();
        let refusal = match self.history_store_from_baseline(baseline) {
            Some(store) if recorded => history_identity!(|identity| ::semio_framework_async::poll::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: operations, transaction: None }, &mut identity))).err().map(|error| format!("{error:?}")),
            _ => {
                operations.into_iter().for_each(retire_flow_mutation);
                None
            }
        };
        if let Some(reason) = refusal {
            self.reset_history_store();
            self.history_fault = Some(FlowCoreError::HistoryRefused(reason));
        }
    }

    /// 🚦️ Closes a discrete edit: records its leaves now unless a gesture is still coalescing them.
    fn finish_change(&mut self) {
        if !self.gesture_active {
            self.flush_pending_change();
        }
    }

    /// 🚦️ [`Self::finish_change`] that hands a refused history row back to the gesture that caused it.
    pub fn checked_change(&mut self) -> Result<(), FlowCoreError> {
        self.finish_change();
        self.history_outcome()
    }

    /// 🧯️ Hands over the refusal of the last history row, if any; the gestures that cannot return it (slider and text setters,
    /// pointer release, note commit) leave it here for their caller to collect.
    pub fn take_history_fault(&mut self) -> Option<FlowCoreError> {
        self.history_fault.take()
    }

    /// 🧯️ [`Self::take_history_fault`] as a `Result`.
    pub fn history_outcome(&mut self) -> Result<(), FlowCoreError> {
        self.take_history_fault().map_or(Ok(()), Err)
    }

    /// 🧾️ Flushes an armed-but-not-yet-recorded discrete mutation into `history_store` as the concrete flow leaves its gesture site emitted — the standard `semio_framework_os_kernel::os_store::ArtifactStore`/`Mutation`/
    /// `MutationDiff` mechanism (see `🔖️Mutations`) driving undo/redo here instead of the old
    /// hand-rolled `Vec<FlowHostSnapshot>` snapshot stack. An armed edit that emitted no leaf leaves no history row.
    fn flush_pending_change(&mut self) {
        if self.pending_change {
            self.pending_change = false;
            let baseline = self.pending_history_baseline.take().unwrap_or_else(|| self.host_snapshot.clone());
            self.record_history_edit(baseline);
        }
    }

    /// ↩️ Arms a checkpoint for the mutation about to happen, unless a gesture (`begin_gesture`) is
    /// currently coalescing several mutations into one.
    pub fn begin_change(&mut self) {
        self.evaluation_revision=self.evaluation_revision.wrapping_add(1);
        if !self.gesture_active {
            self.flush_pending_change();
            self.arm_history_baseline();
            self.pending_change = true;
        }
    }

    /// 🧾️ Arms a fresh undo baseline, RETIRING the one it replaces.
    ///
    /// 🩸️ A baseline is a `FlowHostSnapshot`, which owns the fail-closed `OrderedMap<WidgetLayout>` root,
    /// and overwriting the field simply DROPPED the old one: `panicked at 🗂️ordered/🦀️.rs:81:
    /// ordered-map root must be explicitly retired before drop`, aborting the whole pool worker.
    /// Reached on 6118 by the FOURTH middle-button pan of one session — a gesture whose release never
    /// committed leaves its baseline armed, and the next gesture's `begin_gesture` overwrote it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-node-graph-gestures-2026-09-13.md` §4).
    /// The same hazard sat on `begin_change` and on the history reset inside `apply_fixture`.
    fn arm_history_baseline(&mut self) {
        if let Some(stale) = self.pending_history_baseline.take() {
            stale.retire_cold();
        }
        self.pending_history_baseline = Some(self.host_snapshot.clone());
    }

    /// 🖐️ Starts a coalescing gesture (drag, inline note edit): flushes anything already armed first,
    /// then suppresses further `begin_change` checkpoints until `commit_gesture_history`.
    pub fn begin_gesture(&mut self) {
        self.flush_pending_change();
        self.arm_history_baseline();
        self.journal_mark = self.dag.graph.graph_edits().len();
        self.gesture_active = true;
    }

    /// 🧾️ Closes a coalescing gesture, recording ONE invertible edit when the gesture actually
    /// changed content.
    ///
    /// 🩸️ A gesture that changed nothing — every plain CLICK on the graph — still holds a
    /// `FlowHostSnapshot` baseline, and a `FlowHostSnapshot` owns the fail-closed `OrderedMap<WidgetLayout>`
    /// root: letting it fall out of scope aborted the whole pool worker with `ordered-map root must
    /// be explicitly retired before drop` on the FIRST click. It was unreachable from wgpu only
    /// because no pointer ever reached the graph; the retention fix reaches it on press one (ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-node-graph-surface-retention-2026-09-13.md`).
    /// A no-op baseline is therefore RETIRED, exactly as `history_store_from_baseline` retires the
    /// surplus clone it does not need.
    pub fn commit_gesture_history(&mut self) {
        if self.gesture_active {
            self.gesture_active = false;
            let mut baseline = self.pending_history_baseline.take().unwrap_or_else(|| self.host_snapshot.clone());
            if let Err(refusal) = self.journal_gesture_moves(&baseline) {
                self.discard_pending_leaves();
                baseline.camera = self.host_snapshot.camera.clone();
                std::mem::replace(&mut self.host_snapshot, baseline).retire_cold();
                self.rebuild_dag();
                self.dag.graph.carry_journal_refusal(refusal);
                return;
            }
            if let Some(id) = self.edited_note.take() {
                let edited = self.changed_widget_leaf(&id).filter(|_| baseline.widgets.iter().find(|widget| widget_id_for(widget) == id).is_some_and(|before| self.host_snapshot.widgets.iter().find(|widget| widget_id_for(widget) == id) != Some(before)));
                self.note_leaves(edited);
            }
            let journalled = self.gesture_journal_leaves(&baseline);
            self.note_leaves(journalled);
            if self.pending_leaves.is_empty() {
                baseline.retire_cold();
                return;
            }
            self.record_history_edit(baseline);
        }
    }

    /// ✋️ Narrates a released gesture that moved nodes as the node-graph gesture record of design §13.3 — the moved widgets
    /// (in widget order) and their ONE offset from the gesture's baseline, one record per distinct offset (a grid snap can
    /// land off-grid starts on different offsets) — so the guest commits the drag as relative leaves, exactly the record
    /// the wgpu bounded path writes (`DagHost::plan_graph_edits`). Wires, inline sliders and port inserts are journalled
    /// where they happen; all move rows are admitted together or the caller restores the baseline.
    fn journal_gesture_moves(&mut self, baseline: &FlowHostSnapshot) -> Result<(), dag::DagJournalRefusal> {
        let displacements = self.host_snapshot.widgets.iter().filter_map(|widget| {
            let id = widget_id_for(widget);
            let (landed, start) = (self.host_snapshot.layout.get(id)?, baseline.layout.get(id)?);
            Some((id.to_string(), landed.x - start.x, landed.y - start.y))
        });
        let gesture_id = dag::dag_drag_gesture_id(self.interaction_revision);
        self.dag.graph.journal_moves(&gesture_id, displacements)
    }

    /// ↩️ Restores the previous fixture content snapshot, keeping the current camera.
    pub fn undo(&mut self) -> bool {
        self.flush_pending_change();
        let camera = self.host_snapshot.camera.clone();
        let Some(store) = self.history_store.as_mut() else {
            return false;
        };
        if history_identity!(|identity| ::semio_framework_async::poll::resolve_ready(store.dispatch(ArtifactCommand::Undo, &mut identity))).is_err() {
            return false;
        }
        let Ok(mut restored) = store.snapshot() else {
            return false;
        };
        restored.camera = camera;
        std::mem::replace(&mut self.host_snapshot, restored).retire_cold();
        self.rebuild_dag();
        true
    }

    /// ↪️ Re-applies a fixture content snapshot undone earlier, keeping the current camera.
    pub fn redo(&mut self) -> bool {
        let camera = self.host_snapshot.camera.clone();
        let Some(store) = self.history_store.as_mut() else {
            return false;
        };
        if history_identity!(|identity| ::semio_framework_async::poll::resolve_ready(store.dispatch(ArtifactCommand::Redo, &mut identity))).is_err() {
            return false;
        }
        let Ok(mut restored) = store.snapshot() else {
            return false;
        };
        restored.camera = camera;
        std::mem::replace(&mut self.host_snapshot, restored).retire_cold();
        self.rebuild_dag();
        true
    }

    /// ↩️ Whether a content undo step is available.
    pub fn can_undo(&self) -> bool {
        !self.pending_leaves.is_empty() || self.history_store.as_ref().is_some_and(|store| !store.applied_edit_ids().is_empty())
    }

    /// ↪️ Whether a content redo step is available.
    pub fn can_redo(&self) -> bool {
        self.history_store.as_ref().is_some_and(|store| !store.redo_edit_ids().is_empty())
    }
    // #endregion History
}
