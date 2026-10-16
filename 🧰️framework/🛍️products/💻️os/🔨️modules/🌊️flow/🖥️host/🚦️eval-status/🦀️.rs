//! 🚦️ Evaluation publication decisions, session-wired host construction, unserved kinds and per-node status JSON.

use super::*;

/// 🧬 Stable `nodeHash` for an extension tessellate request (mirrored through ShellHost).
/// 📤️ The publication one surface owes, given the evaluation bytes it ALREADY retains.
///
/// Every caller that used to write `session.eval_json().to_string()` unconditionally goes through
/// here. A `flowEvalTick` self-redispatches until the graph settles, and most of those ticks move no
/// node at all — republishing identical bytes costs a fresh `String` now and a 4096-bytes-per-
/// rotation retirement later, and since the retirement cursor is byte-proportional while the
/// maintenance rotation revisits the transient store only once per 24 turns, an unconditional
/// republication queues retirement faster than the rotation can drain it (the multi-millisecond
/// stage-22 outliers in `📓️runtime-hotpath-audit-2026-09-10.md` §2).
pub fn flow_eval_publication_for(session: &FlowEvalSession, retained: Option<&str>) -> FlowEvalPublication {
    let evaluated = (!session.painted_eval_json().is_empty()).then(|| session.painted_eval_json());
    let bytes = evaluated.map_or(0, str::len) as u64;
    let unchanged = retained == evaluated;
    FLOW_EVAL_PUBLICATION_COUNTERS[0].fetch_add(1, AtomicOrdering::Relaxed);
    FLOW_EVAL_PUBLICATION_COUNTERS[1].fetch_add(bytes, AtomicOrdering::Relaxed);
    if !unchanged {
        FLOW_EVAL_PUBLICATION_COUNTERS[2].fetch_add(1, AtomicOrdering::Relaxed);
        FLOW_EVAL_PUBLICATION_COUNTERS[3].fetch_add(bytes, AtomicOrdering::Relaxed);
    }
    if unchanged {
        return FlowEvalPublication::Retained;
    }
    FlowEvalPublication::Changed(evaluated.map(str::to_string))
}

pub fn preview_tessellate_node_hash(handle: &str, tolerance_bits: u64) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    handle.hash(&mut hasher);
    tolerance_bits.hash(&mut hasher);
    hasher.finish()
}

/// 🏠 Builds a host wired to `session`'s shared cache and converged baseline.
pub fn flow_host_with_session(mut host:FlowHost,session:&FlowEvalSession,grant:RetainedCloneGrant)->Result<(FlowHost,RetainedCloneProgress),(ValueError,FlowHost)>{
    match session.install_baseline_into(&mut host,grant){
        Ok(progress)=>{if let Some(registry)=session.operator_registry.as_ref(){host.operator_registry=Some(registry.clone());}Ok((host,progress))},
        Err(error)=>Err((error,host)),
    }
}

/// 🚧️ The operator kinds `fixture` needs that the live flow extension registry cannot serve — the
/// typed, up-front form of the evaluator's per-node `unknown kind: …`.
///
/// A miss here is NOT something an evaluation tick can work off: `Evaluator::dispatch` answers
/// `EvalError::UnknownKind` for an unregistered kind, the node publishes an error dictionary, and
/// the very next tick recomputes the same miss because an error is never cached. Only a REGISTRY
/// change — a host `setContributions` push, which bumps
/// [`flow_extension_registry_generation`] and re-arms every attached surface through
/// [`FlowEvalSession::invalidate_for_flow_extension_registry`] — can move it. A surface that arms a
/// chain anyway spins forever at its own refresh cadence, which is exactly what the served
/// generation3d editor did for the whole 45 s of a live boot with no contributions installed
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// Scope is deliberately the fixture's own `Widget::Neuron` kinds: those are the kinds a contributed
/// extension pack registers and the ones a host push makes resolvable. A cluster's inner tree is not
/// walked — its boundary kinds are structural (`core.input`/`core.output`), never contributed — so
/// this answer never invents a block that a contribution could not lift.
pub fn unserved_flow_operator_kinds(host_snapshot: &FlowHostSnapshot, registry: &neural::Registry) -> Vec<String> {
    let mut unserved = BTreeSet::new();
    for widget in &host_snapshot.widgets {
        if let Widget::Neuron { neuron_kind, .. } = widget {
            if registry.operator(neuron_kind).is_none() {
                unserved.insert(neuron_kind.clone());
            }
        }
    }
    unserved.into_iter().collect()
}

fn node_eval_status_json(status: &NodeEvalStatus) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(status))
}

/// 📊️ The per-node census one evaluation publishes — the ONE quantity that only ever grows inside a
/// chain, and therefore the only honest denominator a progress ratio may use
/// ([`FlowEvalSession::preview_chain_status`]).
///
/// 🌊️ `Computing` is EVERY node whose request is outstanding at its plugin right now, not the head
/// of `remaining`: a coalesced tick parks a whole topological wave, and naming one of them would
/// leave the rest of a live wave painted as merely queued.
///
/// 📈 A dirty node the walk has already passed is `Ok`, never `Stale`. `dirty` is measured against
/// the chain's FROZEN baseline — it advances only when the chain completes — so every node this
/// evaluation recomputed stays in it until the very last hop. Calling those nodes stale held
/// `nodes_done` at the count of untouched nodes for the whole evaluation: monotone, and flat at the
/// same fraction from the first hop to the last, which is a progress bar that never moves
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A node leaves `remaining` exactly once per chain and
/// never returns to it, which is what makes the census monotone.
/// 🖼️ `live` with every CURRENT widget it did not ANSWER filled in from `converged`.
///
/// ⚖️ An answered node has an output or a fault. `build_channel_eval_json` writes a row for every
/// widget of the document whether the walk reached it or not, so the mark of a node whose request is
/// still crossing is an EMPTY `out` with no `error` beside it — while the last converged walk did
/// produce one. A node the walk answered, including one it answered with an `error`, keeps its live
/// answer, so this can never paint over a real failure; and a node the document no longer declares is
/// never filled in at all, so switching example replaces the graph instead of painting the previous
/// example's leftovers over the new one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn merge_unanswered_eval_entries(live: &str, converged: &str, host_snapshot: &FlowHostSnapshot) -> String {
    if converged.is_empty() || live.is_empty() || is_global_eval_error_json(live) {
        return live.to_string();
    }
    let (Ok(live_value), Ok(converged_value)) = (semio_framework_pack_json::parse(live, semio_framework_pack_json::JsonMemberPolicy::Reject), semio_framework_pack_json::parse(converged, semio_framework_pack_json::JsonMemberPolicy::Reject)) else {
        return live.to_string();
    };
    let (Some(live_map), Some(converged_map)) = (live_value.as_object(), converged_value.as_object()) else {
        return live.to_string();
    };
    let mut merged = live_map.clone();
    let mut filled = false;
    for widget in &host_snapshot.widgets {
        let id = widget_id_for(widget);
        let Some(entry) = converged_map.get(id) else { continue };
        if !eval_entry_is_unanswered(live_map.get(id)) || eval_entry_is_unanswered(Some(entry)) {
            continue;
        }
        merged.insert(id.to_string(), entry.clone());
        filled = true;
    }
    if !filled {
        return live.to_string();
    }
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(merged))
}

/// 🖼️ Whether one evaluation row carries NO answer at all — no output channel and no fault. The row
/// a budgeted walk leaves behind for a node whose extension request it parked.
fn eval_entry_is_unanswered(entry: Option<&semio_framework_pack_json::Value>) -> bool {
    let Some(entry) = entry else { return true };
    if entry.get("error").is_some() {
        return false;
    }
    entry.get("out").and_then(semio_framework_pack_json::Value::as_object).is_none_or(|out| out.iter().next().is_none())
}

pub fn build_flow_status_json(host: &FlowHost, remaining: &[String]) -> String {
    let eval = semio_framework_pack_json::parse(&host.last_eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| semio_framework_pack_json::Value::Object(semio_framework_pack_json::Object::new()));
    let tree = host.build_tree_for_status();
    let seeds = host.build_seeds_for_status();
    let wave: BTreeSet<&str> = host.pending_extension_evals.iter().map(|pending| pending.neuron_id.as_str()).collect();
    let active = remaining.first().map(String::as_str);
    let mut widgets = semio_framework_pack_json::Object::new();
    for widget in &host.host_snapshot.widgets {
        let id = widget_id_for(widget);
        if matches!(widget, Widget::InputSlider { .. } | Widget::InputNote { .. } | Widget::InputImage { .. } | Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } | Widget::Cluster { .. }) {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Ok));
            continue;
        }
        if let Some(entry) = eval.get(id) {
            if let Some(message) = entry.get("error").and_then(semio_framework_pack_json::Value::as_str) {
                widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Error { message: message.to_string() }));
                continue;
            }
        }
        let blocked = host.widget_blocked_ports(id);
        if !blocked.is_empty() {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Blocked { ports: blocked }));
            continue;
        }
        if wave.contains(id) || (wave.is_empty() && active == Some(id)) {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Computing));
            continue;
        }
        if remaining.iter().any(|entry| entry == id) {
            widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Queued));
            continue;
        }
        widgets.insert(id.to_string(), node_eval_status_json(&NodeEvalStatus::Ok));
    }
    tree.retire_cold();
    seeds.retire_cold();
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Object(widgets))
}
