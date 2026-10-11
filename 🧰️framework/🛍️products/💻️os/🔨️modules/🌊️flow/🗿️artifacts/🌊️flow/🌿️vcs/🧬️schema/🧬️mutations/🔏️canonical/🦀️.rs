//! 🔏️ Borrowed exact typed Flow mutation traversal: one indexed navigator serves operation identity, canonical sealing and the borrowed root.
use super::super::super::*;
use super::FlowMutation;
use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::neural::{Atom, Dictionary, Neuron, Tree, Value as NeuralValue};
use crate::os_store::{ArtifactCanonicalJson, ArtifactCanonicalJsonArray as A, ArtifactCanonicalJsonNode as N, ArtifactCanonicalJsonObject as O, ArtifactCanonicalJsonText as T, ArtifactCanonicalJsonValue as V, ArtifactPreparedOperationSource};

/// 🧭️ One original position inside a Flow mutation; every child is an index into the owner, never a copy.
#[derive(Clone, Copy)]
enum Node<'a> {
    Null, Bool(bool), UInt(u64), Float(f64), Text(&'a str),
    Mutation(&'a FlowMutation), Payload(&'a FlowMutation), Widget(&'a Widget), Synapse(&'a SynapseSpec), Layout(&'a WidgetLayout),
    Entries(&'a [FlowLayoutEntry]), Entry(&'a FlowLayoutEntry), Strings(&'a [String]), Set(&'a OrderedSet), Dictionary(&'a Dictionary), Value(&'a NeuralValue),
    Tree(&'a Tree), Neurons(&'a [Neuron]), Neuron(&'a Neuron), Edges(&'a [crate::neural::Synapse]), Edge(&'a crate::neural::Synapse),
    Flow(&'a FlowGui), Camera(&'a CameraJson), Nodes(&'a OrderedMap<FlowNodeGui>), NodeGui(&'a FlowNodeGui), Chrome(&'a NodeChrome),
    Previews(&'a [FlowPreviewGui]), Preview(&'a FlowPreviewGui), Channel(&'a FlowChannelRef),
}

fn invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "canonical-edit.invalid-typed-path") }

fn kind(mutation: &FlowMutation) -> &'static str {
    match mutation {
        FlowMutation::AddWidget(_) => "add-widget", FlowMutation::RemoveWidget(_) => "remove-widget", FlowMutation::MoveWidget(_) => "move-widget", FlowMutation::ChangeWidget(_) => "change-widget",
        FlowMutation::AddSynapse(_) => "add-synapse", FlowMutation::RemoveSynapse(_) => "remove-synapse", FlowMutation::MoveSynapse(_) => "move-synapse", FlowMutation::ChangeSynapse(_) => "change-synapse", FlowMutation::ChangeLayout(_) => "change-layout",
    }
}

impl<'a> Node<'a> {
    fn fields(self) -> Option<&'static [&'static str]> {
        Some(match self {
            Self::Mutation(_) => &["mutation", "payload"],
            Self::Payload(mutation) => match mutation {
                FlowMutation::AddWidget(_) => &["index", "widget"], FlowMutation::AddSynapse(_) => &["index", "synapse"],
                FlowMutation::RemoveWidget(_) | FlowMutation::RemoveSynapse(_) => &["id"], FlowMutation::MoveWidget(_) | FlowMutation::MoveSynapse(_) => &["id", "toIndex"],
                FlowMutation::ChangeWidget(_) => &["id", "widget"], FlowMutation::ChangeSynapse(_) => &["id", "synapse"], FlowMutation::ChangeLayout(_) => &["entries"],
            },
            Self::Widget(widget) => match widget {
                Widget::Neuron { .. } => &["id", "inputPorts", "kind", "neuronKind", "outputPorts", "params", "preview"], Widget::InputSlider { .. } => &["id", "kind", "label", "max", "min", "step", "value"],
                Widget::InputNote { .. } => &["id", "kind", "text"], Widget::InputImage { .. } => &["id", "kind", "src"], Widget::Variable { .. } => &["id", "kind", "name", "schema"],
                Widget::OutputPreview { .. } => &["expanded", "id", "kind", "preview"], Widget::OutputAction { .. } => &["action", "id", "kind"], Widget::OutputExport { .. } => &["format", "id", "kind"], Widget::Cluster { .. } => &["flow", "id", "kind", "name", "tree"],
            },
            Self::Synapse(_) | Self::Edge(_) => &["from", "fromPort", "id", "to", "toPort"], Self::Layout(_) => &["x", "y"], Self::Camera(_) => &["x", "y", "zoom"], Self::Entry(_) => &["id", "layout"],
            Self::Tree(_) => &["neurons", "synapses"], Self::Neuron(_) => &["id", "kind", "params", "tree"], Self::Flow(_) => &["camera", "nodes", "previews"], Self::NodeGui(_) => &["chrome", "layout"],
            Self::Chrome(chrome) => match chrome {
                NodeChrome::Plain { .. } => &["kind", "preview"], NodeChrome::Slider { .. } => &["kind", "label", "max", "min", "step", "value"], NodeChrome::Note { .. } => &["kind", "text"],
                NodeChrome::Image { .. } => &["kind", "src"], NodeChrome::Variable { .. } => &["kind", "name", "schema"],
            },
            Self::Preview(_) => &["expanded", "id", "layout", "mode", "preview", "source"], Self::Channel(_) => &["channel", "neuron"],
            _ => return None,
        })
    }

    fn node(self) -> N<'a> {
        match self {
            Self::Null => N::Null, Self::Bool(value) => N::Bool(value), Self::UInt(value) => N::U64(value), Self::Float(value) => N::F64(value), Self::Text(value) => N::String(value),
            Self::Entries(values) => N::Array(values.len()), Self::Strings(values) => N::Array(values.len()), Self::Set(values) => N::Array(values.len()), Self::Neurons(values) => N::Array(values.len()), Self::Edges(values) => N::Array(values.len()), Self::Previews(values) => N::Array(values.len()),
            Self::Dictionary(values) => N::Object(values.len()), Self::Nodes(values) => N::Object(values.len()),
            Self::Value(value) => match value {
                NeuralValue::Atom(Atom::Null) => N::Null, NeuralValue::Atom(Atom::Boolean(value)) => N::Bool(*value), NeuralValue::Atom(Atom::Integer(value)) => N::I64(*value),
                NeuralValue::Atom(Atom::Decimal(value)) => N::F64(*value), NeuralValue::Atom(Atom::String(value)) => N::String(value), NeuralValue::Dictionary(values) => N::Object(values.len()),
            },
            fixed => N::Object(fixed.fields().map_or(0, <[&str]>::len)),
        }
    }

    fn key(self, index: usize) -> Result<T<'a>, ValueError> {
        if let Some(fields) = self.fields() {
            return fields.get(index).map(|key| T::from(*key)).ok_or_else(invalid);
        }
        match self {
            Self::Dictionary(values) | Self::Value(NeuralValue::Dictionary(values)) => values.entry_at_rank(index).map(|(key, _)| T::from(key.as_str())).ok_or_else(invalid),
            Self::Nodes(values) => values.entry_at_rank(index).map(|(key, _)| T::from(key.as_str())).ok_or_else(invalid),
            _ => Err(invalid()),
        }
    }

    fn child(self, index: usize) -> Result<Self, ValueError> {
        if let Some(fields) = self.fields() {
            return self.field(*fields.get(index).ok_or_else(invalid)?);
        }
        Ok(match self {
            Self::Entries(values) => Self::Entry(values.get(index).ok_or_else(invalid)?), Self::Strings(values) => Self::Text(values.get(index).ok_or_else(invalid)?), Self::Set(values) => Self::Text(values.key_at_rank(index).ok_or_else(invalid)?),
            Self::Neurons(values) => Self::Neuron(values.get(index).ok_or_else(invalid)?), Self::Edges(values) => Self::Edge(values.get(index).ok_or_else(invalid)?), Self::Previews(values) => Self::Preview(values.get(index).ok_or_else(invalid)?),
            Self::Dictionary(values) | Self::Value(NeuralValue::Dictionary(values)) => Self::Value(values.entry_at_rank(index).ok_or_else(invalid)?.1), Self::Nodes(values) => Self::NodeGui(values.entry_at_rank(index).ok_or_else(invalid)?.1),
            _ => return Err(invalid()),
        })
    }

    fn field(self, key: &str) -> Result<Self, ValueError> {
        let layout = |value: &'a Option<WidgetLayout>| value.as_ref().map_or(Self::Null, Self::Layout);
        Ok(match (self, key) {
            (Self::Mutation(mutation), "mutation") => Self::Text(kind(mutation)), (Self::Mutation(mutation), "payload") => Self::Payload(mutation),
            (Self::Payload(FlowMutation::AddWidget(value)), "index") => Self::UInt(value.index.into()), (Self::Payload(FlowMutation::AddWidget(value)), "widget") => Self::Widget(&value.widget),
            (Self::Payload(FlowMutation::RemoveWidget(value)), "id") => Self::Text(&value.id),
            (Self::Payload(FlowMutation::MoveWidget(value)), "id") => Self::Text(&value.id), (Self::Payload(FlowMutation::MoveWidget(value)), "toIndex") => Self::UInt(value.to_index.into()),
            (Self::Payload(FlowMutation::ChangeWidget(value)), "id") => Self::Text(&value.id), (Self::Payload(FlowMutation::ChangeWidget(value)), "widget") => Self::Widget(&value.widget),
            (Self::Payload(FlowMutation::AddSynapse(value)), "index") => Self::UInt(value.index.into()), (Self::Payload(FlowMutation::AddSynapse(value)), "synapse") => Self::Synapse(&value.synapse),
            (Self::Payload(FlowMutation::RemoveSynapse(value)), "id") => Self::Text(&value.id),
            (Self::Payload(FlowMutation::MoveSynapse(value)), "id") => Self::Text(&value.id), (Self::Payload(FlowMutation::MoveSynapse(value)), "toIndex") => Self::UInt(value.to_index.into()),
            (Self::Payload(FlowMutation::ChangeSynapse(value)), "id") => Self::Text(&value.id), (Self::Payload(FlowMutation::ChangeSynapse(value)), "synapse") => Self::Synapse(&value.synapse),
            (Self::Payload(FlowMutation::ChangeLayout(value)), "entries") => Self::Entries(&value.entries),
            (Self::Entry(entry), "id") => Self::Text(&entry.id), (Self::Entry(entry), "layout") => layout(&entry.layout),
            (Self::Layout(value), "x") => Self::Float(value.x), (Self::Layout(value), "y") => Self::Float(value.y),
            (Self::Camera(value), "x") => Self::Float(value.x), (Self::Camera(value), "y") => Self::Float(value.y), (Self::Camera(value), "zoom") => Self::Float(value.zoom),
            (Self::Synapse(value), "from") => Self::Text(&value.from), (Self::Synapse(value), "fromPort") => Self::Text(&value.from_port), (Self::Synapse(value), "id") => Self::Text(&value.id), (Self::Synapse(value), "to") => Self::Text(&value.to), (Self::Synapse(value), "toPort") => Self::Text(&value.to_port),
            (Self::Edge(value), "from") => Self::Text(&value.from), (Self::Edge(value), "fromPort") => Self::Text(&value.from_port), (Self::Edge(value), "id") => Self::Text(&value.id), (Self::Edge(value), "to") => Self::Text(&value.to), (Self::Edge(value), "toPort") => Self::Text(&value.to_port),
            (Self::Tree(value), "neurons") => Self::Neurons(&value.neurons), (Self::Tree(value), "synapses") => Self::Edges(&value.synapses),
            (Self::Neuron(value), "id") => Self::Text(&value.id), (Self::Neuron(value), "kind") => Self::Text(&value.kind), (Self::Neuron(value), "params") => Self::Dictionary(&value.params), (Self::Neuron(value), "tree") => value.tree.as_deref().map_or(Self::Null, Self::Tree),
            (Self::Flow(value), "camera") => Self::Camera(&value.camera), (Self::Flow(value), "nodes") => Self::Nodes(&value.nodes), (Self::Flow(value), "previews") => Self::Previews(&value.previews),
            (Self::NodeGui(value), "chrome") => Self::Chrome(&value.chrome), (Self::NodeGui(value), "layout") => Self::Layout(&value.layout),
            (Self::Preview(value), "expanded") => Self::Set(&value.expanded), (Self::Preview(value), "id") => Self::Text(&value.id), (Self::Preview(value), "layout") => layout(&value.layout), (Self::Preview(value), "mode") => Self::Text(&value.mode),
            (Self::Preview(value), "preview") => Self::Dictionary(&value.preview), (Self::Preview(value), "source") => value.source.as_ref().map_or(Self::Null, Self::Channel),
            (Self::Channel(value), "channel") => Self::Text(&value.channel), (Self::Channel(value), "neuron") => Self::Text(&value.neuron),
            (Self::Chrome(chrome), key) => Self::chrome_field(chrome, key)?,
            (Self::Widget(widget), key) => Self::widget_field(widget, key)?,
            _ => return Err(invalid()),
        })
    }

    fn chrome_field(chrome: &'a NodeChrome, key: &str) -> Result<Self, ValueError> {
        Ok(match (chrome, key) {
            (NodeChrome::Plain { .. }, "kind") => Self::Text("plain"), (NodeChrome::Slider { .. }, "kind") => Self::Text("slider"), (NodeChrome::Note { .. }, "kind") => Self::Text("note"), (NodeChrome::Image { .. }, "kind") => Self::Text("image"), (NodeChrome::Variable { .. }, "kind") => Self::Text("variable"),
            (NodeChrome::Plain { preview }, "preview") => Self::Bool(*preview),
            (NodeChrome::Slider { label, .. }, "label") => Self::Text(label), (NodeChrome::Slider { max, .. }, "max") => Self::Float(*max), (NodeChrome::Slider { min, .. }, "min") => Self::Float(*min), (NodeChrome::Slider { step, .. }, "step") => Self::Float(*step), (NodeChrome::Slider { value, .. }, "value") => Self::Float(*value),
            (NodeChrome::Note { text }, "text") => Self::Text(text), (NodeChrome::Image { src }, "src") => Self::Text(src),
            (NodeChrome::Variable { name, .. }, "name") => Self::Text(name), (NodeChrome::Variable { schema, .. }, "schema") => Self::Text(schema),
            _ => return Err(invalid()),
        })
    }

    fn widget_field(widget: &'a Widget, key: &str) -> Result<Self, ValueError> {
        Ok(match (widget, key) {
            (Widget::Neuron { .. }, "kind") => Self::Text("neuron"), (Widget::InputSlider { .. }, "kind") => Self::Text("inputSlider"), (Widget::InputNote { .. }, "kind") => Self::Text("inputNote"), (Widget::InputImage { .. }, "kind") => Self::Text("inputImage"), (Widget::Variable { .. }, "kind") => Self::Text("variable"),
            (Widget::OutputPreview { .. }, "kind") => Self::Text("outputPreview"), (Widget::OutputAction { .. }, "kind") => Self::Text("outputAction"), (Widget::OutputExport { .. }, "kind") => Self::Text("outputExport"), (Widget::Cluster { .. }, "kind") => Self::Text("cluster"),
            (Widget::Neuron { id, .. }, "id") | (Widget::InputSlider { id, .. }, "id") | (Widget::InputNote { id, .. }, "id") | (Widget::InputImage { id, .. }, "id") | (Widget::Variable { id, .. }, "id") | (Widget::OutputPreview { id, .. }, "id") | (Widget::OutputAction { id, .. }, "id") | (Widget::OutputExport { id, .. }, "id") | (Widget::Cluster { id, .. }, "id") => Self::Text(id),
            (Widget::Neuron { input_ports, .. }, "inputPorts") => Self::Strings(input_ports), (Widget::Neuron { neuron_kind, .. }, "neuronKind") => Self::Text(neuron_kind), (Widget::Neuron { output_ports, .. }, "outputPorts") => Self::Strings(output_ports), (Widget::Neuron { params, .. }, "params") => Self::Dictionary(params), (Widget::Neuron { preview, .. }, "preview") => Self::Bool(*preview),
            (Widget::InputSlider { label, .. }, "label") => Self::Text(label), (Widget::InputSlider { max, .. }, "max") => Self::Float(*max), (Widget::InputSlider { min, .. }, "min") => Self::Float(*min), (Widget::InputSlider { step, .. }, "step") => Self::Float(*step), (Widget::InputSlider { value, .. }, "value") => Self::Float(*value),
            (Widget::InputNote { text, .. }, "text") => Self::Text(text), (Widget::InputImage { src, .. }, "src") => Self::Text(src),
            (Widget::Variable { name, .. }, "name") => Self::Text(name), (Widget::Variable { schema, .. }, "schema") => Self::Text(schema),
            (Widget::OutputPreview { expanded, .. }, "expanded") => Self::Set(expanded), (Widget::OutputPreview { preview, .. }, "preview") => Self::Dictionary(preview),
            (Widget::OutputAction { action, .. }, "action") => Self::Text(action), (Widget::OutputExport { format, .. }, "format") => Self::Text(format),
            (Widget::Cluster { flow, .. }, "flow") => Self::Flow(flow), (Widget::Cluster { name, .. }, "name") => Self::Text(name), (Widget::Cluster { tree, .. }, "tree") => Self::Tree(tree),
            _ => return Err(invalid()),
        })
    }

    fn walk(mut self, path: &[usize]) -> Result<Self, ValueError> {
        for &index in path {
            self = self.child(index)?;
        }
        Ok(self)
    }

    fn borrowed(self) -> V<'a> {
        match self.node() {
            N::Array(length) => V::Array(A::new((0..length).map(move |index| self.child(index).expect("flow canonical array child").borrowed()))),
            N::Object(length) => V::Object(O::new((0..length).map(move |index| (self.key(index).expect("flow canonical key"), self.child(index).expect("flow canonical member").borrowed())))),
            scalar => V::Scalar(scalar),
        }
    }
}

impl ArtifactCanonicalJson for FlowMutation {
    fn canonical_json_node(&self, path: &[usize]) -> Result<N<'_>, ValueError> {
        Ok(Node::Mutation(self).walk(path)?.node())
    }

    fn canonical_json_key(&self, object_path: &[usize], index: usize) -> Result<T<'_>, ValueError> {
        Node::Mutation(self).walk(object_path)?.key(index)
    }

    fn canonical_json_borrowed_root(&self) -> Result<Option<V<'_>>, ValueError> {
        Ok(Some(Node::Mutation(self).borrowed()))
    }
}

/// 🫳️ Borrows the original Flow mutation as its canonical JSON wire before publication.
pub fn prepared_operation_wire_source(operation: &FlowMutation) -> Option<ArtifactPreparedOperationSource<'_>> {
    Some(ArtifactPreparedOperationSource::CanonicalJson { header: b"flow", body: operation })
}
