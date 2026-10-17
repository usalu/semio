/// 🔁️ Dictionary and registry laws: the observed system, the dictionary cursor, the echo and double operators, schema round trips, registry dispatch and the custom dispatch pipelines.

#[global_allocator]
static OBSERVED_SYSTEM:semio_framework_os_kernel_neural_engine_model::testkit::ObservedSystem=semio_framework_os_kernel_neural_engine_model::testkit::ObservedSystem;

#[test]
fn dictionary_owned_cursor_preserves_order_and_nested_ownership() {
    let dictionary = Dictionary::new().insert("z", Value::Atom(Atom::String("last".into()))).insert("a", Value::Dictionary(Dictionary::new().insert("nested", Value::Atom(Atom::String("🌊".repeat(4096))))));
    assert_eq!(dictionary.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), ["a", "z"]);
    let before = dictionary.get("a").and_then(Value::as_dictionary).unwrap().get("nested").and_then(Value::as_atom).and_then(Atom::as_str).unwrap().as_ptr();
    let alias = dictionary.clone();
    let after = alias.get("a").unwrap().as_dictionary().unwrap().get("nested").and_then(Value::as_atom).and_then(Atom::as_str).unwrap().as_ptr();
    assert_eq!(before, after);
    retirement::retire_value_cold(alias.into_retirement());
    retirement::retire_value_cold(dictionary.into_retirement());
}

struct Echo;

impl Operator for Echo {
    fn step_plan(&self,input:Dictionary,grant:RetainedCloneGrant)->Result<(OperatorPlanAdmission,RetainedCloneProgress),(EvalError,Dictionary)>{OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_copy:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(1)}

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let payload = input.get("x").and_then(|value| value.as_dictionary()).cloned().unwrap_or_else(|| input.clone());
        Ok(channel_output("x", payload))
    }
}

struct Double;

impl Operator for Double {
    fn step_plan(&self,input:Dictionary,grant:RetainedCloneGrant)->Result<(OperatorPlanAdmission,RetainedCloneProgress),(EvalError,Dictionary)>{OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_copy:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(1)}

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
        .evaluate_with(&tree, &HashMap::new(), &HistoryFoldIndex::new(), &|kind, input| {
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
