/// 🗂️ Cached evaluation laws: declared defaults, stable node hashes, cache hits, per-node errors and branch recomputation.

#[test]
fn collect_injects_declared_defaults_for_unconnected_inputs() {
    let operator = OperatorInfo {
        id: "list.get".into(),
        extension: "list".into(),
        name: "Get".into(),
        abbreviation: "Get".into(),
        icon: "emoji:🔍️".into(),
        summary: "Get".into(),
        inputs: vec![ChannelSpec::list("list", &["list.get"]), ChannelSpec::number_default("index", 0.0, &["list.get"]), ChannelSpec::boolean_default("wrap", false, &["list.get"])],
        outputs: vec![ChannelSpec::named("V", "Val", "value", "ListValue")],
        ..Default::default()
    };
    let tree = Tree { neurons: vec![Neuron::with_kind("get", "list.get", Dictionary::new())], synapses: vec![] };
    let input = collect_neuron_input(&tree, &HistoryFoldIndex::new(), "get", Some(&operator)).unwrap();
    assert_eq!(input.get("index").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(0.0));
    assert_eq!(input.get("wrap").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_bool()), Some(false));
    input.retire_cold();
    tree.retire_cold();
    operator.retire_cold();
}

#[test]
fn node_hash_is_stable_for_identical_inputs() {
    let input = number_dictionary(3.0);
    assert_eq!(node_hash("double", &input), node_hash("double", &input));
    assert_ne!(node_hash("double", &input), node_hash("echo", &input));
    input.retire_cold();
}

#[test]
fn cached_evaluate_skips_dispatch_on_hit() {
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
    let dispatch = |kind: &str, input: &Dictionary| {
        calls.fetch_add(1, Ordering::Relaxed);
        reg.dispatch(kind, input)
    };
    cache.begin_epoch();
    evaluator.evaluate_channels_cached(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &dispatch, &cache, &HashSet::new(), None).unwrap().retire_cold();
    assert_eq!(calls.load(Ordering::Relaxed), 2);
    cache.begin_epoch();
    evaluator.evaluate_channels_cached(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &dispatch, &cache, &HashSet::new(), None).unwrap().retire_cold();
    assert_eq!(calls.load(Ordering::Relaxed), 2);
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

#[test]
fn missing_required_input_records_per_node_error() {
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "echo", number_dictionary(5.0)), Neuron::with_kind("add", "math.add", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "add".into(), from_port: "x".into(), to_port: "a".into() }],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(add_info(), vec![OperatorImpl { schemas: vec!["number".into(), "number".into()], operator: Box::new(AddNumbers) }], &["number"]);
    let channels = Evaluator::new(&reg).evaluate_channels(&tree, &HashMap::new(), &HistoryFoldIndex::from([(add_info().id.clone(), add_info())])).unwrap();
    let add_out = channels.outputs.get("add").expect("add output");
    assert!(add_out.get("error").is_some() || add_out.get("sum").is_none());
    assert!(channels.outputs.contains_key("a"));
    channels.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

#[test]
fn cached_evaluate_recomputes_only_changed_branch() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "echo", number_dictionary(5.0)), Neuron::with_kind("add", "math.add", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "add".into(), from_port: "x".into(), to_port: "a".into() }, Synapse { id: "s2".into(), from: "b".into(), to: "add".into(), from_port: "x".into(), to_port: "b".into() }],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(add_info(), vec![OperatorImpl { schemas: vec!["number".into(), "number".into()], operator: Box::new(AddNumbers) }], &["number"]);
    let evaluator = Evaluator::new(&reg);
    let cache = NeuralCache::new();
    let calls = AtomicUsize::new(0);
    let dispatch = |kind: &str, input: &Dictionary| {
        calls.fetch_add(1, Ordering::Relaxed);
        reg.dispatch(kind, input)
    };
    cache.begin_epoch();
    evaluator.evaluate_channels_cached(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &dispatch, &cache, &HashSet::new(), None).unwrap().retire_cold();
    assert_eq!(calls.load(Ordering::Relaxed), 3);
    let mut tree_changed = tree.clone();
    std::mem::replace(&mut tree_changed.neurons[0], Neuron::with_kind("a", "echo", number_dictionary(3.0))).retire_cold();
    cache.begin_epoch();
    evaluator.evaluate_channels_cached(&tree_changed, &HashMap::new(), &HistoryFoldIndex::new(), &dispatch, &cache, &HashSet::new(), None).unwrap().retire_cold();
    assert_eq!(calls.load(Ordering::Relaxed), 5);
    tree_changed.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}
