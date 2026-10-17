/// 🌊️ Dirty set and wave laws: the dirty set for unchanged snapshots, descendants, removals and seeds, and the budgeted wave walks over contributed graphs.

#[test]
fn compute_dirty_set_is_empty_for_unchanged_snapshots() {
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(1.0)), Neuron::with_kind("b", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let seeds = HashMap::new();
    let snapshot = TreeSnapshot::capture(&tree, &seeds);
    assert!(compute_dirty_set(Some(&snapshot), &snapshot).is_empty());
    snapshot.retire_cold();
    tree.retire_cold();
}

#[test]
fn compute_dirty_set_propagates_only_to_descendants_of_changed_leaf() {
    // Two independent branches: a -> b, c -> d. Changing `a`'s params should dirty `a` and
    // `b` (its descendant), but never touch `c`/`d` (a disjoint branch).
    let make_tree = |a_value: f64| Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(a_value)), Neuron::with_kind("b", "double", Dictionary::new()), Neuron::with_kind("c", "echo", number_dictionary(9.0)), Neuron::with_kind("d", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }, Synapse { id: "s2".into(), from: "c".into(), to: "d".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let seeds = HashMap::new();
    let previous = TreeSnapshot::capture(&ColdOwner::new(make_tree(1.0)), &seeds);
    let current = TreeSnapshot::capture(&ColdOwner::new(make_tree(2.0)), &seeds);
    let dirty = compute_dirty_set(Some(&previous), &current);
    assert_eq!(dirty, HashSet::from(["a".to_string(), "b".to_string()]));
    previous.retire_cold();
    current.retire_cold();
}

#[test]
fn compute_dirty_set_marks_surviving_dependents_of_removed_neuron() {
    // a -> b -> c. Removing `b` and rewiring `a` directly into `c` must dirty `c`, since it
    // otherwise wouldn't be discovered as changed by iterating the *current* tree alone.
    let before = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(1.0)), Neuron::with_kind("b", "double", Dictionary::new()), Neuron::with_kind("c", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }, Synapse { id: "s2".into(), from: "b".into(), to: "c".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let after = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(1.0)), Neuron::with_kind("c", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s3".into(), from: "a".into(), to: "c".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let seeds = HashMap::new();
    let previous = TreeSnapshot::capture(&before, &seeds);
    let current = TreeSnapshot::capture(&after, &seeds);
    let dirty = compute_dirty_set(Some(&previous), &current);
    assert!(dirty.contains("c"), "surviving dependent of a removed neuron must be dirtied");
    assert!(!dirty.contains("a"), "unrelated unchanged neuron must stay clean");
    previous.retire_cold();
    current.retire_cold();
    before.retire_cold();
    after.retire_cold();
}

#[test]
fn compute_dirty_set_treats_seed_change_as_dirty() {
    let tree = Tree { neurons: vec![Neuron::with_kind("a", "echo", Dictionary::new())], synapses: vec![] };
    let mut before_seeds = HashMap::new();
    before_seeds.insert("a".to_string(), number_dictionary(1.0));
    let mut after_seeds = HashMap::new();
    after_seeds.insert("a".to_string(), number_dictionary(2.0));
    let previous = TreeSnapshot::capture(&tree, &before_seeds);
    let current = TreeSnapshot::capture(&tree, &after_seeds);
    let dirty = compute_dirty_set(Some(&previous), &current);
    assert_eq!(dirty, HashSet::from(["a".to_string()]));
    previous.retire_cold();
    current.retire_cold();
    before_seeds.retire_cold();
    after_seeds.retire_cold();
    tree.retire_cold();
}

/// 🌊️ A graph with two INDEPENDENT contributed nodes (`left`, `right`) feeding one that needs both
/// (`join`), plus a locally computable seed (`root`) — the smallest shape that can tell "one hop per
/// contributed node" apart from "one hop per dependency LEVEL".
fn wave_tree() -> Tree {
    Tree {
        neurons: vec![
            Neuron::with_kind("root", "echo", number_dictionary(2.0)),
            Neuron::with_kind("left", "plugin.left", Dictionary::new()),
            Neuron::with_kind("right", "plugin.right", Dictionary::new()),
            Neuron::with_kind("join", "plugin.join", Dictionary::new()),
        ],
        synapses: vec![
            Synapse { id: "s1".into(), from: "root".into(), to: "left".into(), from_port: "x".into(), to_port: "number".into() },
            Synapse { id: "s2".into(), from: "root".into(), to: "right".into(), from_port: "x".into(), to_port: "number".into() },
            Synapse { id: "s3".into(), from: "left".into(), to: "join".into(), from_port: "out".into(), to_port: "a".into() },
            Synapse { id: "s4".into(), from: "right".into(), to: "join".into(), from_port: "out".into(), to_port: "b".into() },
        ],
    }
}

fn wave_registry() -> Registry {
    let mut registry = Registry::new();
    registry.register_schema(number_schema());
    registry.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    registry
}

/// ⚖️ LAW: one budgeted walk parks EVERY contributed node whose inputs are ready — a whole
/// topological wave — and NEVER one that sits behind a parked answer.
///
/// 🩸️ The walk used to `return` at the first `PendingExtension`, so a chain cost one host round trip
/// per contributed node: measured as 7 `flowEvalTick` hops for the four-operator `sphere-cut-with-torus`
/// and 8 for `face-sweep-extrude`, at roughly a second of host round trip apiece
/// (`📓️react-perf-ceilings-audit-2026-09-14.md` §1, §2d). Independence is what makes the wave safe:
/// no member consumes another member's output, so their answers may be outstanding at once.
///
/// 🚨️ The second half of the law is the one that keeps the answers CORRECT: `collect_neuron_input`
/// silently skips a source with no output, so dispatching `join` while `left` is parked would hand
/// the operator a half-built input and cache the wrong answer under a hash claiming to be the right
/// one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn a_budgeted_walk_parks_one_whole_wave_and_never_a_node_behind_a_parked_answer() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let tree = wave_tree();
    let registry = wave_registry();
    let evaluator = Evaluator::new(&registry);
    let cache = NeuralCache::new();
    let join_dispatches = AtomicUsize::new(0);
    let mut dispatch = |kind: &str, input: &Dictionary| -> Result<Dictionary, EvalError> {
        if kind == "plugin.join" {
            join_dispatches.fetch_add(1, Ordering::Relaxed);
        }
        if let Some(operator_id) = kind.strip_prefix("plugin.") {
            return Err(EvalError::PendingExtension { extension_id: "geometry".into(), operator_id: operator_id.into(), node_hash: node_hash(kind, input) });
        }
        registry.dispatch(kind, input)
    };
    cache.begin_epoch();
    let wave = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::UNBOUNDED,&|_|true).unwrap();
    let parked: Vec<&str> = wave.pending_extensions.iter().map(|pending| pending.neuron_id.as_str()).collect();
    assert_eq!(parked, ["left", "right"], "both ready contributed nodes park on the SAME hop");
    assert_eq!(join_dispatches.load(Ordering::Relaxed), 0, "a node behind a parked answer must never be dispatched");
    assert!(wave.remaining.contains(&"join".to_string()), "the blocked node is still owed");
    assert!(wave.remaining.contains(&"left".to_string()) && wave.remaining.contains(&"right".to_string()), "a parked node is owed until its answer lands");
    assert!(!wave.remaining.contains(&"root".to_string()), "a node this walk computed is not owed again");
    wave.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    registry.retire_cold();
}

/// ⚖️ LAW: a two-level contributed graph converges in exactly TWO waves, not three round trips —
/// the hop count a coalesced chain pays is the number of dependency LEVELS, never the node count.
#[test]
fn a_two_level_contributed_graph_converges_in_two_waves() {
    let tree = wave_tree();
    let registry = wave_registry();
    let evaluator = Evaluator::new(&registry);
    let cache = NeuralCache::new();
    let answered: std::cell::RefCell<HashSet<String>> = std::cell::RefCell::new(HashSet::new());
    let mut waves: Vec<Vec<String>> = Vec::new();
    for _ in 0..4 {
        let mut dispatch = |kind: &str, input: &Dictionary| -> Result<Dictionary, EvalError> {
            let Some(operator_id) = kind.strip_prefix("plugin.") else { return registry.dispatch(kind, input) };
            if answered.borrow().contains(operator_id) {
                return Ok(channel_output("out", number_dictionary(1.0)));
            }
            Err(EvalError::PendingExtension { extension_id: "geometry".into(), operator_id: operator_id.into(), node_hash: node_hash(kind, input) })
        };
        cache.begin_epoch();
        let step = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::UNBOUNDED,&|_|true).unwrap();
        let parked: Vec<String> = step.pending_extensions.iter().map(|pending| pending.neuron_id.clone()).collect();
        let converged = step.remaining.is_empty();
        step.retire_cold();
        if converged {
            break;
        }
        for id in &parked {
            answered.borrow_mut().insert(id.clone());
        }
        waves.push(parked);
    }
    assert_eq!(waves, vec![vec!["left".to_string(), "right".to_string()], vec!["join".to_string()]], "four nodes, two dependency levels, two waves");
    cache.retire_cold();
    tree.retire_cold();
    registry.retire_cold();
}
