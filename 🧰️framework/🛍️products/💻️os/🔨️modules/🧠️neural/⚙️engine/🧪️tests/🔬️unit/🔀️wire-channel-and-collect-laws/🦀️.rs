/// 🔀️ Wire, channel and collect laws: wire precedence over recorded literals, resolved channels per neuron and fixed and variadic port routing.

/// ⚖️ LAW (design §20.10): a wire into a port shadows the literal its neuron records for that port, while an unwired
/// recorded literal still feeds its port — precedence wire > recorded literal > declared default, on the parallel and the
/// sequential walk alike.
#[test]
fn a_wire_shadows_the_literal_its_neuron_records_for_that_port() {
    let mut reg = Registry::new();
    reg.register_schema(number_schema());
    reg.register_operator(echo_info(), vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }], &[]);
    reg.register_operator(double_info(), vec![OperatorImpl { schemas: vec!["number".into()], operator: Box::new(Double) }], &["number"]);
    let recorded = || Dictionary::new().insert("number", Value::Dictionary(number_dictionary(10.0)));
    let tree = Tree {
        neurons: vec![Neuron::with_kind("a", "echo", number_dictionary(2.0)), Neuron::with_kind("wired", "double", recorded()), Neuron::with_kind("unwired", "double", recorded())],
        synapses: vec![Synapse { id: "s1".into(), from: "a".into(), to: "wired".into(), from_port: "x".into(), to_port: "number".into() }],
    };
    let doubled = |outputs: &HistoryFoldIndex<String, Dictionary>, id: &str| outputs.get(id).and_then(|d| d.get("doubled")).and_then(|v| v.as_dictionary()).and_then(|d| d.get("value")).and_then(|v| v.as_atom()).and_then(|a| a.as_f64());
    let parallel = Evaluator::new(&reg).evaluate(&tree, &HashMap::new()).unwrap();
    assert_eq!((doubled(&parallel, "wired"), doubled(&parallel, "unwired")), (Some(4.0), Some(20.0)), "the parallel walk evaluates the wire, and the literal only where nothing is wired");
    let infos = HistoryFoldIndex::new();
    let sequential = Evaluator::new(&reg).evaluate_channels_sequential_with(&tree, &HashMap::new(), &infos, &mut |kind, input| reg.dispatch(kind, input)).unwrap();
    assert_eq!((doubled(&sequential.outputs, "wired"), doubled(&sequential.outputs, "unwired")), (Some(4.0), Some(20.0)), "the sequential walk agrees");
    parallel.retire_cold();
    sequential.retire_cold();
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
    let channels = Evaluator::new(&reg).evaluate_channels(&tree, &seeds, &HistoryFoldIndex::from([(double_info().id.clone(), double_info())])).unwrap();
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
    let mut outputs = HistoryFoldIndex::new();
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
    let mut outputs = HistoryFoldIndex::new();
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
