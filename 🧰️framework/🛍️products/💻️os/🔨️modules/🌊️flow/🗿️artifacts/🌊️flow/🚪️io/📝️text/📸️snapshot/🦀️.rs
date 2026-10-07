//! 🚪️ Native artifact representation codecs.
use super::super::super::*;

/// 📜️ Handcrafted ArtifactDsl (P6): derive no longer emits ArtifactDsl/ArtifactPack.
impl crate::os_store::ArtifactDsl for FlowHostSnapshotDsl {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, crate::os_store::TextError> {
        let body = match crate::os_store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = crate::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as crate::os_store::ArtifactDsl>::envelope_id(), crate::os_store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        crate::os_store::semio_format::wrap_text(&envelope, &body)
    }
}

impl crate::os_store::ArtifactDsl for FlowHostSnapshot {
    const EXTENSION: &'static str = "flow";

    fn envelope_id() -> &'static str {
        <FlowHostSnapshotDsl as crate::os_store::ArtifactDsl>::envelope_id()
    }

    fn parse_dsl(text: &str) -> Result<Self, crate::os_store::TextError> {
        let dsl_fixture = <FlowHostSnapshotDsl as crate::os_store::ArtifactDsl>::parse_dsl(text)?;
        flow_host_snapshot_dsl_to_host_snapshot(dsl_fixture).map_err(|message| crate::os_store::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message, crate::os_store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <FlowHostSnapshotDsl as crate::os_store::ArtifactDsl>::print_dsl(&flow_host_snapshot_to_dsl(self))
    }
}

/// 🌱️ `Value`/`Atom`/`Dictionary`/`Tree`/`Neuron`/`Synapse` are all defined in `neural_engine`
/// (a foreign crate out of scope for this conversion), so none of them can carry a
/// `#[derive(crate::os_dsl::Dsl...)]` themselves — Rust's orphan rule requires the impl target type to live in
/// the crate that also owns the trait or the type, and neither is true here. `ValueDsl`/`TreeDsl`/
/// `NeuronNodeDsl` below are local structural twins that the real types convert to/from right at the
/// `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary — mirroring `imperative_core::ValueDsl`'s
/// identical fix for the same foreign-`Dictionary`/`Value`/`Atom` problem one-for-one (same crate,
/// same shapes).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct ValueDsl {
    /// 🕳️ Presence-only flag (the payload is never inspected) — `Atom::Null`'s tag.
    null: Option<bool>,
    #[dsl(key = "bool")]
    boolean: Option<bool>,
    #[dsl(key = "int")]
    integer: Option<i64>,
    decimal: Option<f64>,
    text: Option<String>,
    #[dsl(key = "dict")]
    dictionary: Option<BTreeMap<String, ValueDsl>>,
}

pub(crate) fn value_to_value_dsl(value: &NeuralValue) -> ValueDsl {
    let mut dsl_value = ValueDsl { null: None, boolean: None, integer: None, decimal: None, text: None, dictionary: None };
    match value {
        NeuralValue::Atom(Atom::Null) => dsl_value.null = Some(true),
        NeuralValue::Atom(Atom::Boolean(b)) => dsl_value.boolean = Some(*b),
        NeuralValue::Atom(Atom::Integer(i)) => dsl_value.integer = Some(*i),
        NeuralValue::Atom(Atom::Decimal(d)) => dsl_value.decimal = Some(*d),
        NeuralValue::Atom(Atom::String(s)) => dsl_value.text = Some(s.clone()),
        NeuralValue::Dictionary(dict) => dsl_value.dictionary = Some(dictionary_to_value_dsl_map(dict)),
    }
    dsl_value
}

pub(crate) fn value_dsl_to_value(dsl_value: &ValueDsl) -> NeuralValue {
    if dsl_value.null.is_some() {
        return NeuralValue::Atom(Atom::Null);
    }
    if let Some(b) = dsl_value.boolean {
        return NeuralValue::Atom(Atom::Boolean(b));
    }
    if let Some(i) = dsl_value.integer {
        return NeuralValue::Atom(Atom::Integer(i));
    }
    if let Some(d) = dsl_value.decimal {
        return NeuralValue::Atom(Atom::Decimal(d));
    }
    if let Some(s) = &dsl_value.text {
        return NeuralValue::Atom(Atom::String(s.clone()));
    }
    match &dsl_value.dictionary {
        Some(entries) => NeuralValue::Dictionary(value_dsl_map_to_dictionary(entries)),
        None => NeuralValue::Atom(Atom::Null),
    }
}

pub(crate) fn dictionary_to_value_dsl_map(dict: &Dictionary) -> BTreeMap<String, ValueDsl> {
    dict.keys().map(|key| (key.clone(), value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")))).collect()
}

pub(crate) fn value_dsl_map_to_dictionary(entries: &BTreeMap<String, ValueDsl>) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, (key, value)| dict.insert(key.clone(), value_dsl_to_value(value)))
}

/// 📦️ `None` when `dict` is empty, mirroring `imperative_core`'s identical printer convention —
/// omits an empty dictionary section rather than printing empty braces.
pub(crate) fn dictionary_to_option_dsl_map(dict: &Dictionary) -> Option<BTreeMap<String, ValueDsl>> {
    (!dict.is_empty()).then(|| dictionary_to_value_dsl_map(dict))
}

pub(crate) fn option_dsl_map_to_dictionary(entries: Option<BTreeMap<String, ValueDsl>>) -> Dictionary {
    entries.map(|entries| value_dsl_map_to_dictionary(&entries)).unwrap_or_default()
}

/// 🔢️ Ordered string membership uses the native DSL array representation — a
/// sorted `Vec<String>` is a lossless, order-independent stand-in at the DSL-text boundary since the
/// real field is reconstructed as a set on the way back in.
pub(crate) fn ordered_set_to_vec(set: &crate::OrderedSet) -> Vec<String> {
    set.iter().cloned().collect()
}

pub(crate) fn vec_to_ordered_set(items: Vec<String>) -> crate::OrderedSet {
    items.into_iter().collect()
}

/// 🌳️ Local twin of `neural::Tree` — mutually recursive with `NeuronNodeDsl` exactly like
/// `imperative_core::PathDsl`/`StepNodeDsl`, so `neurons` goes through `NeuronNodeDsl`'s
/// `crate::os_dsl::DslVariants` lazy `fn() -> RecordSpec` pointer instead of `TreeDsl` and `NeuronNodeDsl`
/// eagerly recursing into each other just to construct the schema.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct TreeDsl {
    #[dsl(statements, block)]
    neurons: Vec<NeuronNodeDsl>,
    #[dsl(table)]
    synapses: Vec<SynapseDsl>,
}

/// 🔵️ Local twin of `neural::Neuron` — a one-variant `crate::os_dsl::DslEnum` (not a plain `DslRecord`) purely
/// for the mutual-recursion reason documented on `TreeDsl`.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub(crate) enum NeuronNodeDsl {
    Neuron {
        id: String,
        kind: String,
        params: Option<BTreeMap<String, ValueDsl>>,
        #[dsl(block)]
        tree: Option<TreeDsl>,
    },
}

/// 🔌️ DSL-only mirror of `SynapseSpec` (and of `neural::Synapse`, its foreign twin embedded in
/// `Tree`) — models the `from`/`fromPort` -> `to`/`toPort` connection as a single unified
/// `crate::os_dsl::Wire` literal (`from@fromPort->to@toPort`) instead of four separate string fields, per
/// the unified syntax law for graph edges/connections. Converts at the `crate::os_store::ArtifactDsl`/
/// `crate::os_store::OpText` boundary through the shared intrinsic field lowering and artifact conversion,
/// plus `tree_to_tree_dsl`/`tree_dsl_to_tree` for the nested neural-tree case); `SynapseSpec`
/// itself (JSON shape, `tree_from_host_snapshot`, `flow_host_snapshot_operations`, every other consumer
/// matching on its `from`/`to`/`from_port`/`to_port` fields) is completely untouched.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct SynapseDsl {
    id: String,
    link: semio_framework_dsl_record::Wire,
}

pub(crate) fn synapse_to_dsl(synapse: &SynapseSpec) -> SynapseDsl {
    let from = semio_framework_dsl_record::WireNode { id: synapse.from.clone(), kind: None, port: (!synapse.from_port.is_empty()).then(|| synapse.from_port.clone()) };
    let to = semio_framework_dsl_record::WireNode { id: synapse.to.clone(), kind: None, port: (!synapse.to_port.is_empty()).then(|| synapse.to_port.clone()) };
    SynapseDsl { id: synapse.id.clone(), link: semio_framework_dsl_record::Wire(semio_framework_dsl_record::WireValue { from, edge: Some((true, to)), edge_label: semio_framework_dsl_record::WireEdgeLabel::default(), properties: semio_framework_value::DslValue::Object(Vec::new()) }) }
}

pub(crate) fn synapse_from_dsl(synapse: SynapseDsl) -> Result<SynapseSpec, String> {
    let semio_framework_dsl_record::WireValue { from, edge, .. } = synapse.link.0;
    let (directed, to) = edge.ok_or_else(|| "synapse wire literal must have a target".to_string())?;
    if !directed {
        return Err("synapse wire literal must be directed".into());
    }
    Ok(SynapseSpec { id: synapse.id, from: from.id, to: to.id, from_port: from.port.unwrap_or_default(), to_port: to.port.unwrap_or_default() })
}

pub(crate) fn tree_to_tree_dsl(tree: &Tree) -> TreeDsl {
    TreeDsl {
        neurons: tree.neurons.iter().map(neuron_to_neuron_node_dsl).collect(),
        synapses: tree.synapses.iter().map(|synapse| synapse_to_dsl(&SynapseSpec { id: synapse.id.clone(), from: synapse.from.clone(), to: synapse.to.clone(), from_port: synapse.from_port.clone(), to_port: synapse.to_port.clone() })).collect(),
    }
}

pub(crate) fn tree_dsl_to_tree(tree: TreeDsl) -> Result<Tree, String> {
    Ok(Tree {
        neurons: tree.neurons.into_iter().map(neuron_node_dsl_to_neuron).collect::<Result<Vec<_>, _>>()?,
        synapses: tree.synapses.into_iter().map(|dsl_synapse| synapse_from_dsl(dsl_synapse).map(|spec| Synapse { id: spec.id, from: spec.from, to: spec.to, from_port: spec.from_port, to_port: spec.to_port })).collect::<Result<Vec<_>, _>>()?,
    })
}

pub(crate) fn neuron_to_neuron_node_dsl(neuron: &Neuron) -> NeuronNodeDsl {
    NeuronNodeDsl::Neuron { id: neuron.id.clone(), kind: neuron.kind.clone(), params: dictionary_to_option_dsl_map(&neuron.params), tree: neuron.tree.as_deref().map(tree_to_tree_dsl) }
}

pub(crate) fn neuron_node_dsl_to_neuron(node: NeuronNodeDsl) -> Result<Neuron, String> {
    let NeuronNodeDsl::Neuron { id, kind, params, tree } = node;
    let tree = match tree {
        Some(tree) => Some(Box::new(tree_dsl_to_tree(tree)?)),
        None => None,
    };
    Ok(Neuron { id, kind, params: option_dsl_map_to_dictionary(params), tree })
}

/// 🎛️ Local twin of `Widget` — a tagged `crate::os_dsl::DslEnum` mirroring its serde `kind` tags one-for-one.
/// `Cluster`'s `flow: FlowGui` is deliberately printed via the engine's `serde_json::Value` escape
/// hatch (untyped but byte-for-byte round-tripping JSON), not its own nested DSL grammar: `FlowGui`/
/// `FlowNodeGui`/`NodeChrome`/`FlowPreviewGui` are GUI-only view state (see each type's own doc
/// comment) that never feeds neural evaluation — `tree_from_host_snapshot`'s `Cluster` handling reads only
/// `tree`, never `flow` — the same "derived read-view, not a DSL-typed field" reasoning `FlowArtifact`
/// itself gets relative to `FlowHostSnapshot`, just one level further in.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub(crate) enum WidgetDsl {
    Neuron {
        id: String,
        neuron_kind: String,
        params: Option<BTreeMap<String, ValueDsl>>,
        input_ports: Vec<String>,
        output_ports: Vec<String>,
        preview: bool,
    },
    InputSlider {
        id: String,
        label: String,
        value: f64,
        min: f64,
        max: f64,
        step: f64,
    },
    InputNote {
        id: String,
        text: String,
    },
    InputImage {
        id: String,
        src: String,
    },
    Variable {
        id: String,
        name: String,
        schema: String,
    },
    OutputPreview {
        id: String,
        preview: Option<BTreeMap<String, ValueDsl>>,
        expanded: Vec<String>,
    },
    OutputAction {
        id: String,
        action: String,
    },
    OutputExport {
        id: String,
        format: String,
    },
    Cluster {
        id: String,
        name: String,
        #[dsl(block)]
        tree: TreeDsl,
        flow: semio_framework_value::DslValue,
    },
}

/// 🌉️ `#[derive(crate::os_dsl::DslEnum)]` only gives `WidgetDsl` a `crate::os_dsl::DslVariants` binding, not
/// `crate::os_dsl::DslField` — so it can't sit directly in a plain (non-`Vec`) field on its own.
/// Direct add/change widget payloads contain REQUIRED, never-collection single
/// values; this hand impl reuses the exact same "exactly one tagged statement" idiom
/// `process_3d::SolidSpec` uses for the identical shape, so those fields stay a bare `WidgetDsl`
/// rather than a `Box<WidgetDsl>`.
impl semio_framework_dsl_record::DslField for WidgetDsl {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{Ok(semio_framework_dsl_record::Shape::Statements(<Self as semio_framework_dsl_record::DslVariants>::variants_controlled(control)?))}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Statements(<WidgetDsl as semio_framework_dsl_record::DslVariants>::variants())
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Statements(vec![<WidgetDsl as semio_framework_dsl_record::DslVariants>::to_named_record(self)])
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Statements(items) if items.len() == 1 => <WidgetDsl as semio_framework_dsl_record::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|e| e.message),
            other => Err(format!("expected exactly 1 tagged widget value, found {other:?}")),
        }
    }
}

pub(crate) fn widget_to_widget_dsl(widget: &Widget) -> WidgetDsl {
    match widget {
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            WidgetDsl::Neuron { id: id.clone(), neuron_kind: neuron_kind.clone(), params: dictionary_to_option_dsl_map(params), input_ports: input_ports.clone(), output_ports: output_ports.clone(), preview: *preview }
        }
        Widget::InputSlider { id, label, value, min, max, step } => WidgetDsl::InputSlider { id: id.clone(), label: label.clone(), value: *value, min: *min, max: *max, step: *step },
        Widget::InputNote { id, text } => WidgetDsl::InputNote { id: id.clone(), text: text.clone() },
        Widget::InputImage { id, src } => WidgetDsl::InputImage { id: id.clone(), src: src.clone() },
        Widget::Variable { id, name, schema } => WidgetDsl::Variable { id: id.clone(), name: name.clone(), schema: schema.clone() },
        Widget::OutputPreview { id, preview, expanded } => WidgetDsl::OutputPreview { id: id.clone(), preview: dictionary_to_option_dsl_map(preview), expanded: ordered_set_to_vec(expanded) },
        Widget::OutputAction { id, action } => WidgetDsl::OutputAction { id: id.clone(), action: action.clone() },
        Widget::OutputExport { id, format } => WidgetDsl::OutputExport { id: id.clone(), format: format.clone() },
        Widget::Cluster { id, name, tree, flow } => WidgetDsl::Cluster { id: id.clone(), name: name.clone(), tree: tree_to_tree_dsl(tree), flow: semio_framework_value::ToValue::to_value(flow) },
    }
}

pub(crate) fn widget_dsl_to_widget(widget: WidgetDsl) -> Result<Widget, String> {
    Ok(match widget {
        WidgetDsl::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => Widget::Neuron { id, neuron_kind, params: option_dsl_map_to_dictionary(params), input_ports, output_ports, preview },
        WidgetDsl::InputSlider { id, label, value, min, max, step } => Widget::InputSlider { id, label, value, min, max, step },
        WidgetDsl::InputNote { id, text } => Widget::InputNote { id, text },
        WidgetDsl::InputImage { id, src } => Widget::InputImage { id, src },
        WidgetDsl::Variable { id, name, schema } => Widget::Variable { id, name, schema },
        WidgetDsl::OutputPreview { id, preview, expanded } => Widget::OutputPreview { id, preview: option_dsl_map_to_dictionary(preview), expanded: vec_to_ordered_set(expanded) },
        WidgetDsl::OutputAction { id, action } => Widget::OutputAction { id, action },
        WidgetDsl::OutputExport { id, format } => Widget::OutputExport { id, format },
        WidgetDsl::Cluster { id, name, tree, flow } => Widget::Cluster { id, name, tree: tree_dsl_to_tree(tree)?, flow: semio_framework_value::FromValue::from_value(flow).map_err(|error| error.to_string())? },
    })
}

/// 📄️ Local mirror of `FlowHostSnapshot` — see this region's opening doc comment for why `widgets:
/// Vec<Widget>` (which embeds foreign `Dictionary`/`Tree` types) can't stay as-is under a direct
/// `#[derive(crate::os_dsl::DslArtifact)]`. `FlowArtifact` (the derived read-view built by
/// `FlowHostSnapshot::to_artifact()`) deliberately does NOT get this treatment — it's a computed
/// snapshot for rendering, never itself round-tripped through DSL text.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, crate::os_dsl::DslArtifact)]
#[artifact(id = "flow.flow")]
#[dsl(layout = "lines")]
pub(crate) struct FlowHostSnapshotDsl {
    schema: String,
    #[dsl(block)]
    camera: CameraJson,
    #[dsl(statements, block)]
    widgets: Vec<WidgetDsl>,
    #[dsl(table)]
    synapses: Vec<SynapseDsl>,
    layout: BTreeMap<String, WidgetLayout>,
}

pub(crate) fn flow_host_snapshot_to_dsl(host_snapshot: &FlowHostSnapshot) -> FlowHostSnapshotDsl {
    FlowHostSnapshotDsl {
        schema: host_snapshot.schema.clone(),
        camera: host_snapshot.camera.clone(),
        widgets: host_snapshot.widgets.iter().map(widget_to_widget_dsl).collect(),
        synapses: host_snapshot.synapses.iter().map(synapse_to_dsl).collect(),
        layout: host_snapshot.layout.iter().map(|(key, value)| (key.clone(), value.clone())).collect(),
    }
}

pub(crate) fn flow_host_snapshot_dsl_to_host_snapshot(dsl: FlowHostSnapshotDsl) -> Result<FlowHostSnapshot, String> {
    Ok(FlowHostSnapshot {
        schema: dsl.schema,
        camera: dsl.camera,
        widgets: dsl.widgets.into_iter().map(widget_dsl_to_widget).collect::<Result<Vec<_>, _>>()?,
        synapses: dsl.synapses.into_iter().map(synapse_from_dsl).collect::<Result<Vec<_>, _>>()?,
        layout: dsl.layout.into_iter().collect(),
    })
}







//#endregion 🔖️Dsl

impl semio_framework_dsl_record::BorrowedDslField for Widget {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Statements(<WidgetDsl as semio_framework_dsl_record::BorrowedDslVariants>::VARIANTS);
}

/// 🎛️ Actual widget payloads share the intrinsic widget DSL lowering.
impl semio_framework_dsl_record::DslField for Widget {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{<WidgetDsl as semio_framework_dsl_record::DslField>::shape_controlled(control)}
    fn shape() -> semio_framework_dsl_record::Shape { <WidgetDsl as semio_framework_dsl_record::DslField>::shape() }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue { <WidgetDsl as semio_framework_dsl_record::DslField>::to_value(&widget_to_widget_dsl(self)) }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        widget_dsl_to_widget(<WidgetDsl as semio_framework_dsl_record::DslField>::from_value(value)?)
    }
}

impl semio_framework_dsl_record::BorrowedDslField for SynapseSpec {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<SynapseDsl>);
}

/// 🔌️ Actual synapse payloads reuse the intrinsic wire-literal lowering.
impl semio_framework_dsl_record::DslField for SynapseSpec {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{<SynapseDsl as semio_framework_dsl_record::DslField>::shape_controlled(control)}
    fn shape() -> semio_framework_dsl_record::Shape { <SynapseDsl as semio_framework_dsl_record::DslField>::shape() }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue { <SynapseDsl as semio_framework_dsl_record::DslField>::to_value(&synapse_to_dsl(self)) }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        synapse_from_dsl(<SynapseDsl as semio_framework_dsl_record::DslField>::from_value(value)?)
    }
}

impl semio_framework_dsl_record::BorrowedDslField for FlowHostSnapshot {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<FlowHostSnapshotDsl>);
}

/// 📄️ Explicit import payloads share the artifact's intrinsic DSL schema.
impl semio_framework_dsl_record::DslField for FlowHostSnapshot {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_dsl_record::FieldValue, semio_framework_value::ValueError> { crate::io::sqlite::snapshot::flow_native_carrier::encode_field(self, control) }
    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, semio_framework_value::ValueError> { crate::io::sqlite::snapshot::flow_native_carrier::decode_field(value, control) }
    fn shape() -> semio_framework_dsl_record::Shape { semio_framework_dsl_record::Shape::Record(FlowHostSnapshotDsl::__dsl_spec_producer()) }
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(FlowHostSnapshotDsl::__dsl_spec_producer()))}
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue { semio_framework_dsl_record::FieldValue::Record(flow_host_snapshot_to_dsl(self).__dsl_to_record()) }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) => flow_host_snapshot_dsl_to_host_snapshot(FlowHostSnapshotDsl::__dsl_from_record(record).map_err(|error| error.message)?),
            other => Err(format!("expected Flow fixture record, found {other:?}")),
        }
    }
}
