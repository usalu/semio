/// 🧵️ Cluster and top-level laws: the add operator, boundary neurons, cluster contracts and inner trees, top-level evaluation and shakability.

struct AddNumbers;

impl Operator for AddNumbers {
    fn step_plan(&self,input:Dictionary,grant:RetainedCloneGrant)->Result<(OperatorPlanAdmission,RetainedCloneProgress),(EvalError,Dictionary)>{OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_copy:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(1)}

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
