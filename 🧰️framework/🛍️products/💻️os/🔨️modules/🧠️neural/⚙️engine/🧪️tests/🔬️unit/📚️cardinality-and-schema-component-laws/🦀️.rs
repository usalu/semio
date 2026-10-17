/// 📚️ Cardinality and schema component laws: cardinality symbols and counts, heterogeneous list refusal, null atoms and the tri-modal schema component.

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
    let mut outputs = HistoryFoldIndex::new();
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
    let input_ids: Vec<&str> = info.inputs.iter().map(|channel| channel.name.as_str()).collect();
    let output_ids: Vec<&str> = info.outputs.iter().map(|channel| channel.name.as_str()).collect();
    assert_eq!(output_ids, vec!["pointOut", "xOut", "yOut", "zOut", "errors"], "a schema component names what it PRODUCES apart from what it is GIVEN");
    assert!(!input_ids.iter().any(|id| output_ids.contains(id)), "one operator's input ids and output ids are disjoint: {input_ids:?} vs {output_ids:?}");
    assert_eq!(info.group, vec!["Schemas".to_string()]);
}

#[test]
fn schema_collection_channels_preserve_wire_and_item_types() {
    let schema = Schema { id: "points".into(), module: "math".into(), name: "Points".into(), icon: String::new(), summary: String::new(), fields: vec![FieldSpec { key: "items".into(), value: ValueType::List(Box::new(ValueType::Schema("point".into()))), default: None, label: None }] };
    let info = schema_component_info(&schema);
    for channel in [&info.inputs[1], &info.outputs[1]] {
        assert_eq!(channel.value_types, ["list"]);
        assert_eq!(channel.item_types, ["point"]);
        let encoded = channel.to_value();
        let round_trip = ChannelSpec::from_value(encoded.clone()).unwrap();
        let oracle: ChannelSpec = serde_json::from_value(serde_json::json!({"code":channel.code,"abbreviation":channel.abbreviation,"name":channel.name,"fullName":channel.full_name,"operators":channel.operators,"valueTypes":["list"],"itemTypes":["point"],"cardinality":"*"})).unwrap();
        assert_eq!(round_trip, oracle);
        round_trip.retire_cold();
        oracle.retire_cold();
    }
    info.retire_cold();
    schema.retire_cold();
}

#[test]
fn schema_component_construct_deconstruct_and_modify() {
    let mut registry = Registry::new();
    registry.register_schema(point_schema());
    registry.finalize();
    let construct = Dictionary::new().insert("x", Value::Dictionary(number_dictionary(1.0))).insert("y", Value::Dictionary(number_dictionary(2.0))).insert("z", Value::Dictionary(number_dictionary(3.0)));
    let built = registry.dispatch("math.point", &construct).unwrap();
    let point = built.get("pointOut").and_then(|value| value.as_dictionary()).expect("pointOut");
    assert_eq!(point.get("z").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(3.0));
    let deconstructed = registry.dispatch("math.point", &ColdOwner::new(Dictionary::new().insert("point", Value::Dictionary(point.clone())))).unwrap();
    assert_eq!(deconstructed.get("xOut").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("value")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(1.0));
    let modified = registry.dispatch("math.point", &ColdOwner::new(Dictionary::new().insert("point", Value::Dictionary(point.clone())).insert("x", Value::Dictionary(number_dictionary(9.0))))).unwrap();
    assert_eq!(modified.get("pointOut").and_then(|value| value.as_dictionary()).and_then(|dictionary| dictionary.get("x")).and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(9.0));
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
    assert!(output.get("pointOut").expect("pointOut").is_null());
    assert!(output.get("xOut").expect("xOut").is_null());
    let errors = output.get("errors").and_then(|value| value.as_dictionary()).expect("errors");
    assert_eq!(errors.schema(), Some("list"));
    assert!(errors.get("0").is_some());
    output.retire_cold();
    registry.retire_cold();
}
