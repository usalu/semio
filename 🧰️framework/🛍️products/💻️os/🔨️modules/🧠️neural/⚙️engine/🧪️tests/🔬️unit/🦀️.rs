
use super::*;

#[test]
fn dictionary_owned_cursor_preserves_order_and_nested_ownership() {
    let dictionary = Dictionary::new().insert("z", Value::Atom(Atom::String("last".into()))).insert("a", Value::Dictionary(Dictionary::new().insert("nested", Value::Atom(Atom::String("🌊".repeat(4096))))));
    assert_eq!(dictionary.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), ["a", "z"]);
    let before = dictionary.get("a").and_then(Value::as_dictionary).unwrap().get("nested").and_then(Value::as_atom).and_then(Atom::as_str).unwrap().as_ptr();
    let alias = dictionary.clone();
    let after = alias.get("a").unwrap().as_dictionary().unwrap().get("nested").and_then(Value::as_atom).and_then(Atom::as_str).unwrap().as_ptr();
    assert_eq!(before, after);
    drop(alias);
    retirement::retire_value_cold(dictionary.into_retirement());
}

struct Echo;

impl Operator for Echo {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let payload = input.get("x").and_then(|value| value.as_dictionary()).cloned().unwrap_or_else(|| input.clone());
        Ok(channel_output("x", payload))
    }
}

struct Double;

impl Operator for Double {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let value = input.get("number").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("number.value".into()))?;
        Ok(channel_output("doubled", number_dictionary(value * 2.0)))
    }
}

fn number_schema() -> Schema {
    Schema { id: "number".into(), module: "core".into(), name: "Number".into(), icon: "emoji:#".into(), summary: "Number dictionary".into(), fields: vec![FieldSpec::decimal_default("value", 0.0)] }
}

fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

fn echo_info() -> OperatorInfo {
    OperatorInfo {
        id: "echo".into(),
        extension: "test".into(),
        name: "Echo".into(),
        abbreviation: "Echo".into(),
        icon: "emoji:📣️".into(),
        summary: "Forwards input".into(),
        inputs: vec![ChannelSpec::any("x")],
        outputs: vec![ChannelSpec::named("X", "x", "x", "Echoed")],
        ..Default::default()
    }
}

fn double_info() -> OperatorInfo {
    OperatorInfo {
        id: "double".into(),
        extension: "test".into(),
        name: "Double".into(),
        abbreviation: "Dbl".into(),
        icon: "emoji:✖️".into(),
        summary: "Doubles number".into(),
        inputs: vec![ChannelSpec::number("number", &["double"])],
        outputs: vec![ChannelSpec::named("D", "Dbl", "doubled", "DoubledNumber")],
        ..Default::default()
    }
}

#[test]
fn dictionary_schema_round_trip() {
    let d = number_dictionary(3.1);
    let json = serde_json::to_string(&d).unwrap();
    let back: Dictionary = serde_json::from_str(&json).unwrap();
    assert_eq!(back.schema(), Some("number"));
    assert_eq!(d, back);
    d.retire_cold();
    back.retire_cold();
}

#[test]
fn schema_ids_and_refs_list_registered_schemas() {
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.finalize();
    assert_eq!(reg.schema_ids(), vec!["number".to_string()]);
    let refs = reg.schema_refs();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].id, "number");
    assert_eq!(refs[0].name, "Number");
    reg.retire_cold();
}

#[test]
fn registry_dispatches_operator() {
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    let out = reg.dispatch("echo", &ColdOwner::new(Dictionary::new().insert("x", Value::Dictionary(number_dictionary(1.0))))).unwrap();
    assert!(out.get("x").is_some());
    out.retire_cold();
    reg.retire_cold();
}

#[test]
fn registry_catalogue_lists_operators_and_schemas() {
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    assert_eq!(reg.schema_catalogue()[0].id, "number");
    assert_eq!(reg.operator_catalogue()[0].id, "double");
    assert_eq!(reg.operator_catalogue()[1].id, "echo");
    reg.retire_cold();
}

#[test]
fn evaluate_with_custom_dispatch() {
    let tree = Tree { neurons: vec![Neuron::with_kind("b", "double", Dictionary::new().insert("number", Value::Dictionary(number_dictionary(3.0))))], synapses: vec![] };
    let out = Evaluator::new(&Registry::new())
        .evaluate_with(&tree, &HashMap::new(), &HashMap::new(), &|kind, input| {
            assert_eq!(kind, "double");
            Double.evaluate(input)
        })
        .unwrap();
    assert_eq!(out.get("b").and_then(|d| d.get("doubled")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(6.0));
    out.retire_cold();
    tree.retire_cold();
}

#[test]
fn two_neuron_pipeline() {
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("b", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let out = Evaluator::new(&reg).evaluate(&tree, &HashMap::new()).unwrap();
    assert_eq!(out.get("b").and_then(|d| d.get("doubled")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(4.0));
    out.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

#[test]
fn evaluate_channels_returns_resolved_inputs_per_neuron() {
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let tree = Tree { neurons: vec![Neuron::with_kind("add", "double", Dictionary::new())], synapses: vec![Synapse { id: "s1".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "number".into() }] };
    let mut seeds = HashMap::new();
    seeds.insert("slider".into(), channel_output("number", number_dictionary(3.0)));
    let channels = Evaluator::new(&reg).evaluate_channels(&tree, &seeds, &HashMap::from([(double_info().id.clone(), double_info())])).unwrap();
    assert_eq!(channels.inputs.get("add").and_then(|d| d.get("number")).and_then(|v| v.as_dictionary()).and_then(|d| d.schema()), Some("number"));
    assert_eq!(channels.outputs.get("add").and_then(|d| d.get("doubled")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(6.0));
    channels.retire_cold();
    seeds.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

#[test]
fn collect_routes_fixed_port_by_key() {
    let tree = Tree {
        neurons: vec![Neuron::with_kind("add", "math.add", Dictionary::new())],
        synapses: vec![
            Synapse { id: "s1".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "a".into() },
            Synapse { id: "s2".into(), from: "note".into(), to: "add".into(), from_port: "number".into(), to_port: "b".into() },
        ],
    };
    let mut outputs = BTreeMap::new();
    outputs.insert("slider".into(), channel_output("number", number_dictionary(2.0)));
    outputs.insert("note".into(), channel_output("number", number_dictionary(3.0)));
    let input = collect_neuron_input(&tree, &outputs, "add", None).unwrap();
    assert_eq!(input.get("a").and_then(|v| v.as_dictionary()).and_then(|d| d.schema()), Some("number"));
    assert_eq!(input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.schema()), Some("number"));
    input.retire_cold();
    outputs.retire_cold();
    tree.retire_cold();
}

#[test]
fn collect_routes_variadic_slots_in_order() {
    let operator = OperatorInfo {
        id: "dictionary.merge".into(),
        extension: "dictionary".into(),
        name: "Merge".into(),
        abbreviation: "Merge".into(),
        icon: "emoji:🔀️".into(),
        summary: "Merge".into(),
        inputs: vec![],
        outputs: vec![ChannelSpec::named("D", "Dic", "dictionary", "MergedDictionary")],
        variadic_input: Some(VariadicSpec { slot_key: "items".into(), min: 2, max: None }),
        ..Default::default()
    };
    let tree = Tree {
        neurons: vec![Neuron::with_kind("merge", "dictionary.merge", Dictionary::new())],
        synapses: vec![
            Synapse { id: "s1".into(), from: "a".into(), to: "merge".into(), from_port: "dictionary".into(), to_port: "0".into() },
            Synapse { id: "s2".into(), from: "b".into(), to: "merge".into(), from_port: "dictionary".into(), to_port: "1".into() },
        ],
    };
    let mut outputs = BTreeMap::new();
    outputs.insert("a".into(), channel_output("dictionary", Dictionary::with_schema("dictionary")));
    outputs.insert("b".into(), channel_output("dictionary", Dictionary::with_schema("dictionary")));
    let input = collect_neuron_input(&tree, &outputs, "merge", Some(&operator)).unwrap();
    let items = input.get("items").and_then(|v| v.as_dictionary()).expect("items");
    assert!(items.get("0").is_some());
    assert!(items.get("1").is_some());
    input.retire_cold();
    outputs.retire_cold();
    tree.retire_cold();
    operator.retire_cold();
}

struct AddNumbers;

impl Operator for AddNumbers {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let a = input.get("a").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("a".into()))?;
        let b = input.get("b").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput("b".into()))?;
        Ok(channel_output("sum", number_dictionary(a + b)))
    }
}

fn add_info() -> OperatorInfo {
    OperatorInfo {
        id: "math.add".into(),
        extension: "math".into(),
        name: "Add".into(),
        abbreviation: "Add".into(),
        icon: "emoji:➕️".into(),
        summary: "Adds numbers".into(),
        inputs: vec![ChannelSpec::number("a", &["math.add"]), ChannelSpec::number("b", &["math.add"])],
        outputs: vec![ChannelSpec::named("S", "Sum", "sum", "Sum")],
        ..Default::default()
    }
}

fn input_boundary(id: &str, channel: &str) -> Neuron {
    Neuron::with_kind(id, INPUT_KIND, Dictionary::new().insert("channel", Value::Atom(Atom::String(channel.into()))).insert("operators", Value::Atom(Atom::String("math.add".into()))))
}

fn output_boundary(id: &str, channel: &str) -> Neuron {
    Neuron::with_kind(id, OUTPUT_KIND, Dictionary::new().insert("channel", Value::Atom(Atom::String(channel.into()))).insert("operators", Value::Atom(Atom::String("math.add".into()))))
}

#[test]
fn cluster_contract_derives_channels() {
    let tree = Tree { neurons: vec![input_boundary("in_a", "a"), input_boundary("in_b", "b"), output_boundary("out_sum", "sum")], synapses: vec![] };
    let (inputs, outputs) = tree.contract();
    assert_eq!(inputs.len(), 2);
    assert_eq!(outputs.len(), 1);
    assert_eq!(inputs[0].name, "a");
    assert_eq!(outputs[0].name, "sum");
    let info = cluster_operator_info("cluster-1", "Add cluster", &tree);
    assert_eq!(info.inputs.len(), 2);
    assert_eq!(info.outputs[0].name, "sum");
    inputs.retire_cold();
    outputs.retire_cold();
    info.retire_cold();
    tree.retire_cold();
}

#[test]
fn cluster_runs_inner_tree() {
    let inner = Tree {
        neurons: vec![input_boundary("in_a", "a"), input_boundary("in_b", "b"), Neuron::with_kind("add", "math.add", Dictionary::new()), output_boundary("out_sum", "sum")],
        synapses: vec![
            Synapse { id: "s1".into(), from: "in_a".into(), to: "add".into(), from_port: String::new(), to_port: "a".into() },
            Synapse { id: "s2".into(), from: "in_b".into(), to: "add".into(), from_port: String::new(), to_port: "b".into() },
            Synapse { id: "s3".into(), from: "add".into(), to: "out_sum".into(), from_port: "sum".into(), to_port: String::new() },
        ],
    };
    let tree = Tree {
        neurons: vec![
            Neuron::with_kind("a_src", "core.number", Dictionary::new()),
            Neuron::with_kind("b_src", "core.number", Dictionary::new()),
            Neuron { id: "cluster".into(), kind: CLUSTER_KIND.into(), params: Dictionary::new(), tree: Some(Box::new(inner)) },
        ],
        synapses: vec![
            Synapse { id: "s_a".into(), from: "a_src".into(), to: "cluster".into(), from_port: "number".into(), to_port: "a".into() },
            Synapse { id: "s_b".into(), from: "b_src".into(), to: "cluster".into(), from_port: "number".into(), to_port: "b".into() },
        ],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(add_info(), vec![OperatorImpl { schemas: vec!["number".into(), "number".into()], operator: Box::new(AddNumbers) }], &["number"]);
    let mut seeds = HashMap::new();
    seeds.insert("a_src".into(), channel_output("number", number_dictionary(2.0)));
    seeds.insert("b_src".into(), channel_output("number", number_dictionary(3.0)));
    let out = Evaluator::new(&reg).evaluate(&tree, &seeds).unwrap();
    assert_eq!(out.get("cluster").and_then(|d| d.get("sum")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(5.0));
    out.retire_cold();
    seeds.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

#[test]
fn evaluate_function_top_level() {
    let tree = Tree {
        neurons: vec![input_boundary("in_a", "a"), input_boundary("in_b", "b"), Neuron::with_kind("add", "math.add", Dictionary::new()), output_boundary("out_sum", "sum")],
        synapses: vec![
            Synapse { id: "s1".into(), from: "in_a".into(), to: "add".into(), from_port: String::new(), to_port: "a".into() },
            Synapse { id: "s2".into(), from: "in_b".into(), to: "add".into(), from_port: String::new(), to_port: "b".into() },
            Synapse { id: "s3".into(), from: "add".into(), to: "out_sum".into(), from_port: "sum".into(), to_port: String::new() },
        ],
    };
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(add_info(), vec![OperatorImpl { schemas: vec!["number".into(), "number".into()], operator: Box::new(AddNumbers) }], &["number"]);
    let in_dict = Dictionary::new().insert("a", Value::Dictionary(number_dictionary(2.0))).insert("b", Value::Dictionary(number_dictionary(3.0)));
    let out = Evaluator::new(&reg).evaluate_function(&tree, &in_dict).unwrap();
    assert_eq!(out.get("sum").and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(5.0));
    out.retire_cold();
    in_dict.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

#[test]
fn cluster_shakability_round_trip() {
    let inner = Tree { neurons: vec![input_boundary("in_a", "a"), output_boundary("out_a", "a")], synapses: vec![Synapse { id: "s1".into(), from: "in_a".into(), to: "out_a".into(), from_port: String::new(), to_port: String::new() }] };
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a_src", "core.number", Dictionary::new()), Neuron { id: "cluster".into(), kind: CLUSTER_KIND.into(), params: Dictionary::new(), tree: Some(Box::new(inner)) }],
        synapses: vec![Synapse { id: "s0".into(), from: "a_src".into(), to: "cluster".into(), from_port: "number".into(), to_port: "a".into() }],
    };
    let json = serde_json::to_string(&tree).unwrap();
    let back: Tree = serde_json::from_str(&json).unwrap();
    let mut seeds = HashMap::new();
    seeds.insert("a_src".into(), channel_output("number", number_dictionary(7.0)));
    let out = Evaluator::new(&Registry::new()).evaluate(&back, &seeds).unwrap();
    assert_eq!(out.get("cluster").and_then(|d| d.get("a")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(7.0));
    out.retire_cold();
    seeds.retire_cold();
    back.retire_cold();
    tree.retire_cold();
}

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
    let input = collect_neuron_input(&tree, &BTreeMap::new(), "get", Some(&operator)).unwrap();
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
    evaluator.evaluate_channels_cached(&tree, &HashMap::new(), &HashMap::new(), &dispatch, &cache, &HashSet::new(), None).unwrap().retire_cold();
    assert_eq!(calls.load(Ordering::Relaxed), 2);
    cache.begin_epoch();
    evaluator.evaluate_channels_cached(&tree, &HashMap::new(), &HashMap::new(), &dispatch, &cache, &HashSet::new(), None).unwrap().retire_cold();
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
    let channels = Evaluator::new(&reg).evaluate_channels(&tree, &HashMap::new(), &HashMap::from([(add_info().id.clone(), add_info())])).unwrap();
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
    evaluator.evaluate_channels_cached(&tree, &HashMap::new(), &HashMap::new(), &dispatch, &cache, &HashSet::new(), None).unwrap().retire_cold();
    assert_eq!(calls.load(Ordering::Relaxed), 3);
    let mut tree_changed = tree.clone();
    tree_changed.neurons[0] = Neuron::with_kind("a", "echo", number_dictionary(3.0));
    cache.begin_epoch();
    evaluator.evaluate_channels_cached(&tree_changed, &HashMap::new(), &HashMap::new(), &dispatch, &cache, &HashSet::new(), None).unwrap().retire_cold();
    assert_eq!(calls.load(Ordering::Relaxed), 5);
    tree_changed.retire_cold();
    cache.retire_cold();
    tree.retire_cold();
    reg.retire_cold();
}

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
    let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &dirty, None, EvalStepBudget::PROBE).unwrap();
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
    let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::PROBE).unwrap();
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
    let tick1 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::dispatches(1)).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(tick1.remaining, vec!["b".to_string()]);
    // ⏱️ Tick 2: "a" is now a cache hit (free), so this budget-1 call reaches and computes "b".
    let tick2 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::dispatches(1)).unwrap();
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
    let tick1 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, overrun).unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 1, "an expired deadline must still admit one dispatch — a walk that computes nothing can never converge");
    assert_eq!(tick1.remaining, vec!["b".to_string()], "the walk yields at the next cache miss and names it as the blocker");
    let tick2 = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, overrun).unwrap();
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
    let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::until(512, absent_now_us, 0)).unwrap();
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
    let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::until(0, always_expired_now_us, 0)).unwrap();
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
    let result = evaluator.evaluate_channels_budgeted(&tree, &HashMap::new(), &HashMap::new(), &mut dispatch, &cache, &HashSet::new(), None, EvalStepBudget::UNBOUNDED).unwrap();
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

#[test]
fn cardinality_symbol_round_trips_json() {
    let channel = ChannelSpec::list("items", &["list.pack"]).with_cardinality(Cardinality::OneOrMore);
    let json = serde_json::to_string(&channel).unwrap();
    assert!(json.contains("\"cardinality\":\"+\""));
    let parsed: ChannelSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.cardinality, Cardinality::OneOrMore);
}

#[test]
fn cardinality_accepts_expected_counts() {
    assert!(Cardinality::ExactlyOne.accepts(1));
    assert!(!Cardinality::ExactlyOne.accepts(0));
    assert!(Cardinality::ZeroOrMore.accepts(0));
    assert!(Cardinality::Exactly(2).accepts(2));
    assert!(!Cardinality::Exactly(2).accepts(1));
}

#[test]
fn heterogeneous_list_input_is_rejected() {
    let operator = OperatorInfo {
        id: "list.size".into(),
        extension: "list".into(),
        name: "Size".into(),
        abbreviation: "Size".into(),
        icon: "emoji:📋️".into(),
        summary: "Size".into(),
        inputs: vec![ChannelSpec::list("list", &["list.size"])],
        outputs: vec![ChannelSpec::named("C", "Cnt", "count", "ListCount")],
        ..Default::default()
    };
    let tree = Tree { neurons: vec![Neuron::with_kind("size", "list.size", Dictionary::new())], synapses: vec![Synapse { id: "s1".into(), from: "src".into(), to: "size".into(), from_port: "list".into(), to_port: "list".into() }] };
    let mut outputs = BTreeMap::new();
    outputs.insert(
        "src".into(),
        channel_output("list", Dictionary::with_schema("list").insert("0", Value::Dictionary(number_dictionary(1.0))).insert("1", Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String("x".into())))))),
    );
    let err = collect_neuron_input(&tree, &outputs, "size", Some(&operator)).unwrap_err();
    assert!(matches!(err, EvalError::HeterogeneousList(_)));
    outputs.retire_cold();
    tree.retire_cold();
    operator.retire_cold();
}

fn point_schema() -> Schema {
    Schema {
        id: "point".into(),
        module: "math".into(),
        name: "Point".into(),
        icon: "emoji:📍️".into(),
        summary: "Point with x, y, z".into(),
        fields: vec![FieldSpec::decimal_default("x", 0.0), FieldSpec::decimal_default("y", 0.0), FieldSpec::decimal_default("z", 0.0)],
    }
}

#[test]
fn null_atom_round_trips_through_json() {
    let value = Value::null();
    let json = serde_json::to_string(&value).unwrap();
    assert_eq!(json, "null");
    let back: Value = serde_json::from_str(&json).unwrap();
    assert!(back.is_null());
}

#[test]
fn schema_component_info_declares_tri_modal_ports() {
    let info = schema_component_info(&point_schema());
    assert_eq!(info.id, "math.point");
    assert_eq!(info.inputs.len(), 4);
    assert_eq!(info.outputs.len(), 5);
    assert_eq!(info.inputs[0].cardinality, Cardinality::ZeroOrOne);
    assert_eq!(info.outputs.last().expect("errors").name, "errors");
    assert_eq!(info.group, vec!["Schemas".to_string()]);
}

#[test]
fn schema_component_construct_deconstruct_and_modify() {
    let mut registry = Registry::new();
    registry.register_schema(point_schema());
    registry.finalize();
    let construct = Dictionary::new().insert("x", Value::Dictionary(number_dictionary(1.0))).insert("y", Value::Dictionary(number_dictionary(2.0))).insert("z", Value::Dictionary(number_dictionary(3.0)));
    let built = registry.dispatch("math.point", &construct).unwrap();
    let point = built.get("point").and_then(|value| value.as_dictionary()).expect("point");
    assert_eq!(point.get("z").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(3.0));
    let deconstructed = registry.dispatch("math.point", &ColdOwner::new(Dictionary::new().insert("point", Value::Dictionary(point.clone())))).unwrap();
    assert_eq!(deconstructed.get("x").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(1.0));
    let modified = registry.dispatch("math.point", &ColdOwner::new(Dictionary::new().insert("point", Value::Dictionary(point.clone())).insert("x", Value::Dictionary(number_dictionary(9.0))))).unwrap();
    assert_eq!(modified.get("point").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("x")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(9.0));
    modified.retire_cold();
    deconstructed.retire_cold();
    built.retire_cold();
    construct.retire_cold();
    registry.retire_cold();
}

#[test]
fn schema_component_error_emits_null_outputs_and_errors() {
    let mut registry = Registry::new();
    registry.register_schema(point_schema());
    registry.finalize();
    let output = registry.dispatch("math.point", &Dictionary::new()).unwrap();
    assert!(output.get("point").expect("point").is_null());
    assert!(output.get("x").expect("x").is_null());
    let errors = output.get("errors").and_then(|value| value.as_dictionary()).expect("errors");
    assert_eq!(errors.schema(), Some("list"));
    assert!(errors.get("0").is_some());
    output.retire_cold();
    registry.retire_cold();
}

#[test]
fn compute_dirty_set_is_empty_for_unchanged_snapshots() {
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(1.0)), Neuron::with_kind("b", "double", Dictionary::new())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let seeds = HashMap::new();
    let snapshot = TreeSnapshot::capture(&tree, &seeds);
    assert!(compute_dirty_set(Some(&snapshot), &snapshot).is_empty());
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
    before_seeds.retire_cold();
    after_seeds.retire_cold();
    tree.retire_cold();
}
