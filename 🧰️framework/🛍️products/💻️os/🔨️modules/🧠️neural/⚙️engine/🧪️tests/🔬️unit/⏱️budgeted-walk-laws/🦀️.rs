/// ⏱️ Budgeted walk laws: remaining and probe walks, resumption across calls, failed dispatch service from the cache, the wall clock and node count caps and the cache epoch refresh before sweep.

#[test]
fn evaluate_channels_budgeted_remaining_excludes_clean_branches() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new()), Neuron::with_kind("c", "echo", number_dictionary(9.0))],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let evaluator = Evaluator::new(&reg);
    let cache = NeuralCache::new();
    let calls = AtomicUsize::new(0);
    let mut dispatch = |kind: &str, input: &Dictionary| {
        calls.fetch_add(1, Ordering::Relaxed);
        reg.dispatch(kind, input)
    };
    let dirty: HashSet<String> = ["b".to_string()].into_iter().collect();
    cache.begin_epoch();
    let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &dirty, None, EvalStepBudget::PROBE,&|_|true).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(result.remaining, vec!["b".to_string()], "clean branch node \"c\" must not appear in remaining");
    result.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

#[test]
fn evaluate_channels_budgeted_probe_computes_nothing() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let evaluator = Evaluator::new(&reg);
    let cache = NeuralCache::new();
    let calls = AtomicUsize::new(0);
    let mut dispatch = |kind: &str, input: &Dictionary| {
        calls.fetch_add(1, Ordering::Relaxed);
        reg.dispatch(kind, input)
    };
    cache.begin_epoch();
    let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::PROBE,&|_|true).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 0, "a budget-0 probe must never dispatch");
    assert_eq!(result.remaining, vec!["a".to_string(), "b".to_string()], "nothing computed yet — every neuron is still pending, in topo order");
    result.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

#[test]
fn evaluate_channels_budgeted_resumes_across_calls_until_complete() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let evaluator = Evaluator::new(&reg);
    let cache = NeuralCache::new();
    let calls = AtomicUsize::new(0);
    let mut dispatch = |kind: &str, input: &Dictionary| {
        calls.fetch_add(1, Ordering::Relaxed);
        reg.dispatch(kind, input)
    };
    cache.begin_epoch();
    // ⏱️ Tick 1: budget for exactly one cache miss — stops at "a", "b" hasn't run yet.
    let tick1 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::dispatches(1),&|_|true).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(tick1.remaining, vec!["b".to_string()]);
    // ⏱️ Tick 2: "a" is now a cache hit (free), so this budget-1 call reaches and computes "b".
    let tick2 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::dispatches(1),&|_|true).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 2, "resuming must not recompute the already-cached \"a\"");
    assert!(tick2.remaining.is_empty(), "the walk reached the end of the topo order");
    let doubled = tick2.channels.outputs.get("b").and_then(|dict| dict.get("doubled")).and_then(|value| value.as_dictionary()).and_then(|dict| dict.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64());
    assert_eq!(doubled, Some(4.0));
    tick1.retire_cold();
    tick2.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

/// ⚖️ LAW: a node whose dispatch FAILS is answered once per input, like a success. The resumed walk
/// serves the failure from the cache and reaches the nodes behind it; before, the failing node was
/// re-dispatched first on every call, a one-dispatch budget never got past it, and the run never
/// converged (`sphere-cut-with-torus` ticked a refused boolean for 900 s).
#[test]
fn evaluate_channels_budgeted_serves_a_failed_dispatch_from_the_cache_on_resume() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "double", Dictionary::new()), Neuron::with_kind("b", "echo", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "doubled".into(), to_port: "x".into() }],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let evaluator = Evaluator::new(&reg);
    let cache = NeuralCache::new();
    let calls = AtomicUsize::new(0);
    let mut dispatch = |kind: &str, input: &Dictionary| {
        calls.fetch_add(1, Ordering::Relaxed);
        reg.dispatch(kind, input)
    };
    cache.begin_epoch();
    let tick1 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::dispatches(1),&|_|true).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert!(tick1.channels.outputs.get("a").and_then(|dict| dict.get("error")).is_some(), "\"a\" has no number input, so its dispatch fails");
    assert_eq!(tick1.remaining, vec!["b".to_string()]);
    let tick2 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::dispatches(1),&|_|true).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 2, "resuming must serve the failed \"a\" from the cache and spend the budget on \"b\"");
    assert!(tick2.remaining.is_empty(), "the walk got past the failed node and converged");
    assert!(tick2.channels.outputs.get("a").and_then(|dict| dict.get("error")).is_some(), "the cached answer is the same failure");
    tick1.retire_cold();
    tick2.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

/// 🕰️ A clock that is always past ANY deadline — the worst case a wall-clock budget must survive.
fn always_expired_now_us() -> Option<u64> {
    Some(u64::MAX)
}

/// 🕰️ A target with no installed clock answers nothing; the node count must remain the only cap.
fn absent_now_us() -> Option<u64> {
    None
}

/// ⚖️ LAW: a wall-clock budget preempts the dag walk BETWEEN neurons, and never before the walk has
/// dispatched at least one — so a single expensive operator yields the turn after itself instead of
/// parking the reactor for the whole graph, and a walk that starts already over budget still
/// converges one node per call rather than re-arming forever with no progress
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-guest-tick-cost-2026-09-12.md` §4 rank 3).
#[test]
fn evaluate_channels_budgeted_yields_on_the_wall_clock_after_one_dispatch() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let evaluator = Evaluator::new(&reg);
    let cache = NeuralCache::new();
    let calls = AtomicUsize::new(0);
    let mut dispatch = |kind: &str, input: &Dictionary| {
        calls.fetch_add(1, Ordering::Relaxed);
        reg.dispatch(kind, input)
    };
    // ⏱️ A node budget that would swallow the whole graph in one call, paired with a deadline that
    // has already passed: the deadline, not the node count, is what stops the walk.
    let overrun = EvalStepBudget::until(512, always_expired_now_us, 0);
    cache.begin_epoch();
    let tick1 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, overrun,&|_|true).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 1, "an expired deadline must still admit one dispatch — a walk that computes nothing can never converge");
    assert_eq!(tick1.remaining, vec!["b".to_string()], "the walk yields at the next cache miss and names it as the blocker");
    let tick2 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, overrun,&|_|true).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 2, "resuming past an expired deadline dispatches exactly one more node");
    assert!(tick2.remaining.is_empty(), "two calls converge the two-node chain even with the deadline permanently expired");
    let doubled = tick2.channels.outputs.get("b").and_then(|dict| dict.get("doubled")).and_then(|value| value.as_dictionary()).and_then(|dict| dict.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64());
    assert_eq!(doubled, Some(4.0), "preempting the walk must not change what it computes");
    tick1.retire_cold();
    tick2.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

/// ⚖️ LAW: a deadline whose clock answers nothing (bare wasm with no host clock installed) never
/// preempts — the node count stays the only cap, exactly as before the deadline existed.
#[test]
fn evaluate_channels_budgeted_without_a_clock_keeps_the_node_count_as_the_only_cap() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let evaluator = Evaluator::new(&reg);
    let cache = NeuralCache::new();
    let calls = AtomicUsize::new(0);
    let mut dispatch = |kind: &str, input: &Dictionary| {
        calls.fetch_add(1, Ordering::Relaxed);
        reg.dispatch(kind, input)
    };
    cache.begin_epoch();
    let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::until(512, absent_now_us, 0),&|_|true).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 2, "both neurons run in one call when the clock cannot say the deadline passed");
    assert!(result.remaining.is_empty());
    result.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

/// ⚖️ LAW: `PROBE` outranks any deadline — a probe dispatches nothing whatever the clock says.
#[test]
fn evaluate_channels_budgeted_probe_dispatches_nothing_even_past_a_deadline() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let evaluator = Evaluator::new(&reg);
    let cache = NeuralCache::new();
    let calls = AtomicUsize::new(0);
    let mut dispatch = |kind: &str, input: &Dictionary| {
        calls.fetch_add(1, Ordering::Relaxed);
        reg.dispatch(kind, input)
    };
    cache.begin_epoch();
    let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::until(0, always_expired_now_us, 0),&|_|true).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 0, "a zero-dispatch budget never dispatches, deadline or not");
    assert_eq!(result.remaining, vec!["a".to_string(), "b".to_string()]);
    result.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

#[test]
fn evaluate_channels_budgeted_unlimited_matches_full_evaluation() {
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let evaluator = Evaluator::new(&reg);
    let cache = NeuralCache::new();
    let mut dispatch = |kind: &str, input: &Dictionary| reg.dispatch(kind, input);
    cache.begin_epoch();
    let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::UNBOUNDED,&|_|true).unwrap();
    assert!(result.remaining.is_empty());
    let doubled = result.channels.outputs.get("b").and_then(|dict| dict.get("doubled")).and_then(|value| value.as_dictionary()).and_then(|dict| dict.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64());
    assert_eq!(doubled, Some(4.0));
    result.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

#[test]
fn neural_cache_get_and_contains_refresh_epoch_before_sweep() {
    let cache = NeuralCache::new();
    cache.begin_epoch();
    cache.get_or_insert_with(42, || Ok(Dictionary::new())).expect("seed cache");
    cache.begin_epoch();
    assert!(cache.contains(42), "contains must refresh the entry epoch on a hit");
    cache.sweep();
    assert!(cache.contains(42), "swept cache must retain entries touched by contains/get in the new epoch");
    cache.get(42);
    cache.sweep();
    assert_eq!(cache.len(), 1, "get must also refresh epoch so a completing eval does not evict its own hits");
    cache.retire_cold();
}
