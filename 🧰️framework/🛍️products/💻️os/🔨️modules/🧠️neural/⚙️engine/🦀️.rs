//! 🧠️ Headless neural engine: dictionary in, dictionary out.

use std::collections::{HashMap, HashSet, VecDeque};
use protocol::causal::transition::HistoryFoldIndex;
/// 🔮️ Test-only: production moved to `ToValue`/`FromValue` below (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass — see `📓️orderedmap-tenth-seam.md`).
#[cfg(test)]
use serde::Deserialize;
#[cfg(test)]
use protocol::value::{DslValue, FromValue, ToValue, ValueError};
#[cfg(test)]
use semio_framework_value::ValueType;

pub use semio_framework_os_kernel_neural_engine_model::{Atom, CLUSTER_KIND, Cardinality, ChannelSpec, ColdDictionaryBuilder, ColdOwner, ColdRetire, ColdValueOwner, Dictionary, EvalChannels, EvalError, FieldSpec, INPUT_KIND, Key, NeuralEngineError, Neuron, OUTPUT_KIND, Operator, OperatorInfo, OperatorJob, OperatorJobStep, OperatorPendingPlan, OperatorPlanAdmission, OperatorProgress, OperatorWorkFailure, OriginalDictionaryLookup, PendingOperatorMetadata, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, SCHEMA_KEY, Schema, SchemaRef, Synapse, Tree, TreeSnapshot, VALUE_TYPE_BOOLEAN, VALUE_TYPE_GEOMETRY, VALUE_TYPE_LIST, VALUE_TYPE_NUMBER, VALUE_TYPE_POINT, VALUE_TYPE_TEXT, VALUE_TYPE_VECTOR, Value, ValueRetirement, VariadicSpec, channel_output, cluster_operator_info, cold, compute_dirty_set, contract_channel, derive_channel_names, node_hash, operator_signature, retirement, validate_neuron_inputs, validate_operator_outputs};
pub use semio_framework_os_kernel_neural_engine_registry::{OperatorFinishCursor, OperatorImpl, OperatorPlanCursor, OperatorPlanStep, OperatorRecord, Registry, RegistryIdentity, RegistryLeaseRetirement, RegistryRetirement, SchemaComponent, SharedRegistry, SharedRegistryAdmissionStep, SharedRegistryIssuer, produced_channel_id, schema_component_info};
pub use semio_framework_os_kernel_neural_engine_budgeted::{BudgetedEval, BudgetedEvalState, BudgetedEvalStep, BudgetedInputMerge, BudgetedOwnedEntry, BudgetedTreeCopy, EvalStepBudget, EvalStepDeadline, NeuralCache, NeuralCacheInsert, NeuralCacheRetirement, PendingExtensionEval, budgeted_remaining, cache_insert, eval_error_dictionary, evaluate_cached_output, input, topology, tree_copy, waits_on_parked};
pub use semio_framework_os_kernel_neural_engine_registry as registry;
pub use semio_framework_os_kernel_neural_engine_budgeted as evaluation;

// #region 🔖️Evaluator

/// 🔄️ Topological evaluation over a neural tree.
pub struct Evaluator<'a> {
    registry: &'a Registry,
}

impl<'a> Evaluator<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self { registry }
    }

    pub fn evaluate(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>) -> Result<HistoryFoldIndex<String, Dictionary>, EvalError> {
        let EvalChannels { outputs, inputs } = self.evaluate_channels(tree, seeds, &HistoryFoldIndex::new())?;
        inputs.retire_cold(); Ok(outputs)
    }

    pub fn evaluate_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HistoryFoldIndex<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
    ) -> Result<HistoryFoldIndex<String, Dictionary>, EvalError> {
        let EvalChannels { outputs, inputs } = self.evaluate_channels_with(tree, seeds, operator_infos, dispatch)?;
        inputs.retire_cold(); Ok(outputs)
    }

    pub fn evaluate_channels(&self, tree: &Tree, seeds: &HashMap<String, Dictionary>, operator_infos: &HistoryFoldIndex<String, OperatorInfo>) -> Result<EvalChannels, EvalError> {
        self.evaluate_channels_with(tree, seeds, operator_infos, &|kind, input| self.registry.dispatch(kind, input))
    }

    pub fn evaluate_channels_sequential_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HistoryFoldIndex<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
    ) -> Result<EvalChannels, EvalError> {
        let cache = ColdOwner::new(NeuralCache::new());
        cache.begin_epoch();
        let result = self.evaluate_channels_sequential_cached(tree, seeds, operator_infos, dispatch, &cache, &HashSet::new(), None);
        cache.sweep();
        result
    }

    #[allow(clippy::too_many_arguments, reason = "incremental cache eval needs tree+seeds+infos+dispatch+cache+dirty+previous together; splitting into a params struct would ripple into flow/core/rs call sites outside this ticket's scope")]
    pub fn evaluate_channels_sequential_cached(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HistoryFoldIndex<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
        cache: &NeuralCache,
        dirty: &HashSet<String>,
        previous: Option<&EvalChannels>,
    ) -> Result<EvalChannels, EvalError> {
        self.evaluate_channels_budgeted(tree, seeds, operator_infos, dispatch, cache, dirty, previous, EvalStepBudget::UNBOUNDED,&|_|true).map(|budgeted| budgeted.channels)
    }

    /// ⏳️ Sequential topo walk that stops after computing `budget.dispatches` cache-missed (i.e.
    /// actually dispatched) neurons OR once `budget.deadline` has passed, returning the
    /// not-yet-computed neuron ids as `remaining` so a caller can resume with another budgeted call
    /// — used to spread a heavy evaluation across many cheap ticks instead of blocking a thread for
    /// the whole graph. [`EvalStepBudget::PROBE`] is a pure probe: nothing is dispatched,
    /// `remaining` reports every neuron that would still need work.
    ///
    /// 🧯️ A dispatch that FAILS is the node's answer for that input exactly like a success, so it is
    /// seeded into `cache` too — the same rule a contributed extension's error answer already follows
    /// (`seed_node_cache`). Left uncached, a failing node costing more than one tick's deadline was
    /// re-dispatched first on every resumed tick, the walk never reached the nodes behind it, and the
    /// run ticked forever: `sphere-cut-with-torus` at the tangent radius 2.5 re-ran its refused
    /// `brep.bool.cut` for 900 s while `brep.measure.volume` stayed `computing`.
    #[allow(clippy::too_many_arguments, reason = "mirrors evaluate_channels_sequential_cached's params plus a budget; see that method's reason")]
    pub fn evaluate_channels_budgeted(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HistoryFoldIndex<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
        cache: &NeuralCache,
        dirty: &HashSet<String>,
        previous: Option<&EvalChannels>,
        budget: EvalStepBudget,
        source_required: &dyn Fn(u64)->bool,
    ) -> Result<BudgetedEval, EvalError> {
        let order = topo_order(tree)?;
        let mut outputs = ColdOwner::new(seeds.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<HistoryFoldIndex<String, Dictionary>>());
        let mut inputs = ColdOwner::new(HistoryFoldIndex::<String, Dictionary>::new());
        let mut spent = 0usize;
        let mut parked: HashSet<String> = HashSet::new();
        let mut pending_extensions: Vec<PendingExtensionEval> = Vec::new();
        let mut stopped_at = order.len();
        for (index, neuron_id) in order.iter().enumerate() {
            if !dirty.contains(neuron_id) {
                if let Some(prev) = previous {
                    if let (Some(out), Some(inp)) = (prev.outputs.get(neuron_id), prev.inputs.get(neuron_id)) {
                        outputs.insert(neuron_id.clone(), out.clone()).retire_cold();
                        inputs.insert(neuron_id.clone(), inp.clone()).retire_cold();
                        continue;
                    }
                }
            }
            if waits_on_parked(tree, &parked, neuron_id) {
                parked.insert(neuron_id.clone());
                continue;
            }
            let neuron = tree.neurons.iter().find(|n| n.id == *neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
            let operator_info = operator_info_for_neuron(neuron, operator_infos, self.registry.operator_info(&neuron.kind));
            let input = ColdOwner::new(collect_neuron_input(tree, &outputs, neuron_id, operator_info)?);
            inputs.insert(neuron_id.clone(), input.clone()).retire_cold();
            if let Some(seed) = seeds.get(neuron_id) {
                outputs.insert(neuron_id.clone(), seed.clone()).retire_cold();
                continue;
            }
            if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                outputs.insert(neuron_id.clone(), input.merge(&neuron.params)).retire_cold();
                continue;
            }
            // 🚧️ A budget-exhausted cache miss (cluster or operator) stops the walk here; this
            // neuron and everything from `order[index..]` becomes `remaining`. Exhaustion is either
            // the dispatch count or the wall-clock deadline — see [`EvalStepBudget`]. Clusters have
            // no single cache key of their own (their inner neurons are cached individually), so a
            // cluster is conservatively always charged as a miss.
            if let Some(sub_tree) = neuron.tree.as_deref() {
                if budget.exhausted(spent) {
                    stopped_at = index;
                    break;
                }
                let out = self.evaluate_cluster_sequential(sub_tree, &input, operator_infos, dispatch, cache)?;
                outputs.insert(neuron_id.clone(), out).retire_cold();
                spent += 1;
                continue;
            }
            let literal = ColdOwner::new(unwired_params(tree, neuron));
            let merged = ColdOwner::new(input.merge(&literal));
            let key = node_hash(&neuron.kind, &merged);
            let is_miss = !cache.contains(key);
            if is_miss && budget.exhausted(spent) {
                stopped_at = index;
                break;
            }
            let out = if let Some(cached) = cache.get(key) {
                cached
            } else {
                match dispatch(&neuron.kind, &merged) {
                    Err(EvalError::PendingExtension { extension_id, operator_id, node_hash }) => {
                        // 🌊️ The wave gathers rather than returns: this neuron's answer has to come
                        // from its plugin, but every LATER neuron that does not depend on it can
                        // still be walked in this same call, so its own request rides the same round
                        // trip instead of costing a whole further hop.
                        pending_extensions.push(PendingExtensionEval { neuron_id: neuron_id.clone(), extension_id, operator_id, node_hash, input_json: if source_required(node_hash) {semio_framework_pack_json::to_json_string(&*merged)}else{String::new()} });
                        parked.insert(neuron_id.clone());
                        spent += 1;
                        continue;
                    }
                    Err(err) => {
                        let fault = eval_error_dictionary(&err);
                        cache.seed(key, fault.clone());
                        fault
                    }
                    Ok(dict) => {
                        cache.seed(key, dict.clone());
                        dict
                    }
                }
            };
            outputs.insert(neuron_id.clone(), out).retire_cold();
            if is_miss {
                spent += 1;
            }
        }
        let remaining = budgeted_remaining(&order, stopped_at, &parked, dirty);
        Ok(BudgetedEval { channels: EvalChannels { outputs: outputs.into_inner(), inputs: inputs.into_inner() }, remaining, pending_extensions })
    }

    pub fn evaluate_channels_with(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HistoryFoldIndex<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
    ) -> Result<EvalChannels, EvalError> {
        let cache = ColdOwner::new(NeuralCache::new());
        cache.begin_epoch();
        let result = self.evaluate_channels_cached(tree, seeds, operator_infos, dispatch, &cache, &HashSet::new(), None);
        cache.sweep();
        result
    }

    #[allow(clippy::too_many_arguments, reason = "incremental cache eval needs tree+seeds+infos+dispatch+cache+dirty+previous together; splitting into a params struct would ripple into flow/core/rs call sites outside this ticket's scope")]
    pub fn evaluate_channels_cached(
        &self,
        tree: &Tree,
        seeds: &HashMap<String, Dictionary>,
        operator_infos: &HistoryFoldIndex<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
        cache: &NeuralCache,
        dirty: &HashSet<String>,
        previous: Option<&EvalChannels>,
    ) -> Result<EvalChannels, EvalError> {
        let levels = topo_levels(tree)?;
        let mut outputs = ColdOwner::new(seeds.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<HistoryFoldIndex<String, Dictionary>>());
        let mut inputs = ColdOwner::new(HistoryFoldIndex::<String, Dictionary>::new());
        for level in levels {
            let mut level_inputs = ColdOwner::new(HistoryFoldIndex::<String, Dictionary>::new());
            let mut level_outputs = ColdOwner::new(HistoryFoldIndex::<String, Dictionary>::new());
            let mut deferred_clusters = ColdOwner::new(Vec::<(String, Tree, Dictionary)>::new());
            let mut compute_jobs = ColdOwner::new(Vec::<(String, String, Dictionary)>::new());

            for neuron_id in &level {
                if !dirty.contains(neuron_id) {
                    if let Some(prev) = previous {
                        if let (Some(out), Some(inp)) = (prev.outputs.get(neuron_id), prev.inputs.get(neuron_id)) {
                            level_outputs.insert(neuron_id.clone(), out.clone()).retire_cold();
                            level_inputs.insert(neuron_id.clone(), inp.clone()).retire_cold();
                            continue;
                        }
                    }
                }
                let neuron = tree.neurons.iter().find(|n| n.id == *neuron_id).ok_or_else(|| EvalError::InvalidInput(format!("missing neuron {neuron_id}")))?;
                let operator_info = operator_info_for_neuron(neuron, operator_infos, self.registry.operator_info(&neuron.kind));
                let input = ColdOwner::new(collect_neuron_input(tree, &outputs, neuron_id, operator_info)?);
                level_inputs.insert(neuron_id.clone(), input.clone()).retire_cold();
                if let Some(seed) = seeds.get(neuron_id) {
                    level_outputs.insert(neuron_id.clone(), seed.clone()).retire_cold();
                    continue;
                }
                if let Some(sub_tree) = neuron.tree.as_deref().cloned() {
                    deferred_clusters.push((neuron_id.clone(), sub_tree, input.into_inner()));
                    continue;
                }
                if neuron.kind == INPUT_KIND || neuron.kind == OUTPUT_KIND {
                    level_outputs.insert(neuron_id.clone(), input.merge(&neuron.params)).retire_cold();
                    continue;
                }
                let literal = ColdOwner::new(unwired_params(tree, neuron));
                compute_jobs.push((neuron_id.clone(), neuron.kind.clone(), input.merge(&literal)));
            }

            for (neuron_id, kind, merged) in compute_jobs.into_inner() {
                let merged = ColdOwner::new(merged);
                let out = evaluate_cached_output(cache, &kind, &merged, || dispatch(&kind, &merged));
                level_outputs.insert(neuron_id, out).retire_cold();
            }

            for (neuron_id, sub_tree, input) in deferred_clusters.into_inner() {
                let sub_tree = ColdOwner::new(sub_tree); let input = ColdOwner::new(input);
                let out = self.evaluate_cluster(&sub_tree, &input, operator_infos, dispatch, cache)?;
                level_outputs.insert(neuron_id, out).retire_cold();
            }

            for (key, value) in level_inputs.into_inner() { inputs.insert(key, value).retire_cold(); }
            for (key, value) in level_outputs.into_inner() { outputs.insert(key, value).retire_cold(); }
        }
        Ok(EvalChannels { outputs: outputs.into_inner(), inputs: inputs.into_inner() })
    }

    /// 🧮️ Evaluates a tree as a function: in dictionary to out dictionary via boundary neurons.
    pub fn evaluate_function(&self, tree: &Tree, in_dict: &Dictionary) -> Result<Dictionary, EvalError> {
        self.evaluate_function_with(tree, in_dict, &HistoryFoldIndex::new(), &|kind, input| self.registry.dispatch(kind, input))
    }

    /// 🧮️ Evaluates a tree as a function with custom dispatch and operator metadata.
    pub fn evaluate_function_with(&self, tree: &Tree, in_dict: &Dictionary, operator_infos: &HistoryFoldIndex<String, OperatorInfo>, dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync)) -> Result<Dictionary, EvalError> {
        let seeds = ColdOwner::new(seed_input_boundaries(tree, in_dict));
        let channels = ColdOwner::new(self.evaluate_channels_with(tree, &seeds, operator_infos, dispatch)?);
        collect_output_boundaries(tree, &channels)
    }

    /// 🧮️ Evaluates a tree as a function with caching and custom dispatch.
    pub fn evaluate_function_cached(
        &self,
        tree: &Tree,
        in_dict: &Dictionary,
        operator_infos: &HistoryFoldIndex<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
        cache: &NeuralCache,
    ) -> Result<Dictionary, EvalError> {
        let seeds = ColdOwner::new(seed_input_boundaries(tree, in_dict));
        let channels = ColdOwner::new(self.evaluate_channels_cached(tree, &seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?);
        collect_output_boundaries(tree, &channels)
    }

    fn evaluate_cluster_sequential(
        &self,
        sub_tree: &Tree,
        parent_input: &Dictionary,
        operator_infos: &HistoryFoldIndex<String, OperatorInfo>,
        dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>,
        cache: &NeuralCache,
    ) -> Result<Dictionary, EvalError> {
        let sub_seeds = ColdOwner::new(seed_input_boundaries(sub_tree, parent_input));
        let sub_channels = ColdOwner::new(self.evaluate_channels_sequential_cached(sub_tree, &sub_seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?);
        collect_output_boundaries(sub_tree, &sub_channels)
    }

    fn evaluate_cluster(
        &self,
        sub_tree: &Tree,
        parent_input: &Dictionary,
        operator_infos: &HistoryFoldIndex<String, OperatorInfo>,
        dispatch: &(dyn Fn(&str, &Dictionary) -> Result<Dictionary, EvalError> + Sync),
        cache: &NeuralCache,
    ) -> Result<Dictionary, EvalError> {
        let sub_seeds = ColdOwner::new(seed_input_boundaries(sub_tree, parent_input));
        // 🧩️ Nested cluster subtrees are evaluated atomically (v1 limitation): any change inside
        // re-evaluates the whole cluster rather than propagating dirtiness within it. Never stale,
        // just not maximally incremental for nested clusters.
        let sub_channels = ColdOwner::new(self.evaluate_channels_cached(sub_tree, &sub_seeds, operator_infos, dispatch, cache, &HashSet::new(), None)?);
        collect_output_boundaries(sub_tree, &sub_channels)
    }
}

fn operator_info_for_neuron<'a>(neuron: &Neuron, operator_infos: &'a HistoryFoldIndex<String, OperatorInfo>, registry_info: Option<&'a OperatorInfo>) -> Option<&'a OperatorInfo> {
    if neuron.tree.is_some() {
        return None;
    }
    operator_infos.get(&neuron.kind).or(registry_info)
}

fn boundary_seed_dictionary(value: &Value) -> Dictionary {
    match value {
        Value::Dictionary(dict) => dict.clone(),
        other => Dictionary::new().insert("value", other.clone()),
    }
}

fn boundary_output_value(input: &Dictionary) -> Option<Value> {
    if input.len() == 1 {
        return input.keys().next().and_then(|key| input.get(key).cloned());
    }
    if input.is_empty() {
        return None;
    }
    Some(Value::Dictionary(input.clone()))
}

/// 🌱️ Seeds input boundary neurons from an in dictionary keyed by channel name.
pub fn seed_input_boundaries(tree: &Tree, in_dict: &Dictionary) -> HashMap<String, Dictionary> {
    let mut seeds = HashMap::new();
    for neuron in &tree.neurons {
        if neuron.kind != INPUT_KIND {
            continue;
        }
        let (channel_id, _) = contract_channel(neuron);
        let Some(value) = in_dict.get(&channel_id) else {
            continue;
        };
        seeds.insert(neuron.id.clone(), boundary_seed_dictionary(value));
    }
    seeds
}

/// 📤️ Collects output boundary neuron values into an out dictionary keyed by channel name.
pub fn collect_output_boundaries(tree: &Tree, channels: &EvalChannels) -> Result<Dictionary, EvalError> {
    let mut out = ColdDictionaryBuilder::new();
    for neuron in &tree.neurons {
        if neuron.kind != OUTPUT_KIND {
            continue;
        }
        let (channel_id, _) = contract_channel(neuron);
        let Some(neuron_input) = channels.inputs.get(&neuron.id) else {
            return Err(EvalError::MissingInput(format!("output boundary {channel_id}")));
        };
        let Some(value) = boundary_output_value(neuron_input) else {
            return Err(EvalError::MissingInput(format!("output boundary {channel_id}")));
        };
        out.insert(channel_id, value);
    }
    Ok(out.finish())
}

fn synapse_source_value(src_out: &Dictionary, from_port: &str) -> Value {
    if from_port.is_empty() || from_port == "out" {
        if from_port == "out" {
            if let Some(value) = src_out.get("out") {
                return value.clone();
            }
            if src_out.len() == 1 {
                if let Some(key) = src_out.keys().next() {
                    if let Some(value) = src_out.get(key) {
                        return value.clone();
                    }
                }
            }
        }
        return Value::Dictionary(src_out.clone());
    }
    src_out.get(from_port).cloned().unwrap_or_else(|| Value::Dictionary(Dictionary::new().insert("error", Value::Atom(Atom::String(format!("missing channel {from_port}"))))))
}

fn insert_variadic_slot(acc: Dictionary, slot_key: &str, port_id: &str, value: Value) -> Dictionary {
    let mut slots = acc.get(slot_key).and_then(|v| v.as_dictionary()).cloned().unwrap_or_default();
    slots = slots.insert(port_id.to_string(), value);
    acc.insert(slot_key.to_string(), Value::Dictionary(slots))
}

fn insert_fixed_port(acc: Dictionary, port_key: &str, value: Value) -> Dictionary {
    acc.insert(port_key.to_string(), value)
}

/// 🔌️ A neuron's recorded literals without the ones its wires shadow (design §20.10): a wire into port `P` shadows the
/// literal recorded for `P`, so a compute neuron evaluates wire > recorded literal > declared default. A dictionary wire
/// (empty `to_port`) shadows nothing.
fn unwired_params(tree: &Tree, neuron: &Neuron) -> Dictionary {
    neuron.params.iter().filter(|(key, _)| !tree.synapses.iter().any(|synapse| synapse.to == neuron.id && !synapse.to_port.is_empty() && synapse.to_port == **key)).fold(Dictionary::new(), |literal, (key, value)| literal.insert(key.clone(), value.clone()))
}

/// 💉️ Fills missing declared input keys from operator channel defaults.
pub fn inject_channel_defaults(acc: Dictionary, operator_info: &OperatorInfo) -> Dictionary {
    let mut acc = acc;
    for spec in &operator_info.inputs {
        if spec.name == "*" || acc.get(&spec.name).is_some() {
            continue;
        }
        if let Some(default) = &spec.default {
            acc = acc.insert(spec.name.clone(), default.clone());
        }
    }
    acc
}

fn inject_channel_defaults_for_operator(acc: Dictionary, operator_info: Option<&OperatorInfo>) -> Dictionary {
    match operator_info {
        Some(info) => inject_channel_defaults(acc, info),
        None => acc,
    }
}

fn collect_neuron_input(tree: &Tree, outputs: &HistoryFoldIndex<String, Dictionary>, neuron_id: &str, operator_info: Option<&OperatorInfo>) -> Result<Dictionary, EvalError> {
    let mut acc = Dictionary::new();
    let variadic = operator_info.and_then(|info| info.variadic_input.as_ref());
    for syn in &tree.synapses {
        if syn.to != neuron_id {
            continue;
        }
        let Some(src_out) = outputs.get(&syn.from) else { continue };
        let value = synapse_source_value(src_out, &syn.from_port);
        if let Some(spec) = variadic {
            let port_id = if syn.to_port.is_empty() { "0" } else { syn.to_port.as_str() };
            acc = insert_variadic_slot(acc, &spec.slot_key, port_id, value);
            continue;
        }
        if syn.to_port.is_empty() {
            if let Value::Dictionary(dict) = value {
                let next = acc.merge(&dict);
                acc.retire_cold(); dict.retire_cold(); acc = next;
            }
            continue;
        }
        acc = insert_fixed_port(acc, &syn.to_port, value);
    }
    let acc = ColdOwner::new(inject_channel_defaults_for_operator(acc, operator_info));
    validate_neuron_inputs(&acc, operator_info)?;
    Ok(acc.into_inner())
}

fn topo_order(tree: &Tree) -> Result<Vec<String>, EvalError> {
    Ok(topo_levels(tree)?.into_iter().flatten().collect())
}

// 🚧️ Not migrated to `graph::algorithms::topo_levels`: doing so would create a circular crate
// dependency (`neural_engine` → `semio-framework-graph` → `graph::manifest` → `neural_engine`,
// via manifest's `PropertyValue` → `neural_engine::Value` conversion). Fixing that requires relocating
// `Value`/`Atom` out of `neural_engine` into a shared lower crate — out of this ticket's scope.
fn topo_levels(tree: &Tree) -> Result<Vec<Vec<String>>, EvalError> {
    let ids: HashSet<String> = tree.neurons.iter().map(|n| n.id.clone()).collect();
    let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
    for id in &ids {
        incoming.insert(id.clone(), vec![]);
    }
    for syn in &tree.synapses {
        if !ids.contains(&syn.from) || !ids.contains(&syn.to) {
            continue;
        }
        incoming.entry(syn.to.clone()).or_default().push(syn.from.clone());
    }
    let mut indegree: HashMap<String, usize> = incoming.iter().map(|(k, v)| (k.clone(), v.len())).collect();
    let mut queue: VecDeque<String> = indegree.iter().filter(|(_, &d)| d == 0).map(|(k, _)| k.clone()).collect();
    queue.make_contiguous().sort();
    let mut levels = Vec::new();
    let mut visited = 0usize;
    while !queue.is_empty() {
        let mut level: Vec<String> = queue.drain(..).collect();
        level.sort();
        visited += level.len();
        for n in &level {
            for syn in &tree.synapses {
                if syn.from != *n {
                    continue;
                }
                if let Some(d) = indegree.get_mut(&syn.to) {
                    *d = d.saturating_sub(1);
                    if *d == 0 {
                        queue.push_back(syn.to.clone());
                    }
                }
            }
        }
        levels.push(level);
    }
    if visited != ids.len() {
        return Err(EvalError::CycleDetected);
    }
    Ok(levels)
}
// #endregion 🔖️Evaluator
// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🚦️owned-controls/🦀️.rs"]
mod owned_value_controls;

#[cfg(test)]
#[path = "🧪️tests/🌲️canonical/🦀️.rs"]
mod canonical_projection;
// #endregion 🔖️Tests
