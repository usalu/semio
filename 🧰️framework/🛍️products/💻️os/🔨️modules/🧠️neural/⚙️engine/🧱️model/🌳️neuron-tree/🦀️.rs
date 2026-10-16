//! 🌳️ Neuron tree: the neuron and synapse tree with its channel contract constants.
//!
//! Holds the `🔖️Tree`, `🔖️Contract` regions of the module; the root re-exports its items.

use super::*;

// #region 🔖️Tree
/// 🌳️ Directed acyclic graph of neurons and synapses. `serde` is TEST-ONLY — see `Dictionary`'s
/// docstring; mutually recursive with `Neuron` via `Neuron.tree: Option<Box<Tree>>`, so both moved
/// off serde together.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct Tree {
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
}

/// 🔵️ Neuron instance bound to a kind. `serde` is TEST-ONLY — see `Tree`'s docstring above.
#[derive(Clone, Debug, PartialEq, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
pub struct Neuron {
    pub id: String,
    pub kind: String,
    #[cfg_attr(test, serde(default))]
    pub params: Dictionary,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub tree: Option<Box<Tree>>,
}

impl Neuron {
    pub fn with_kind(id: impl Into<String>, kind: impl Into<String>, params: Dictionary) -> Self {
        Self { id: id.into(), kind: kind.into(), params, tree: None }
    }
}

impl Tree {
    /// 📜️ Derives contract input and output channels from boundary neurons.
    pub fn contract(&self) -> (Vec<ChannelSpec>, Vec<ChannelSpec>) {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for neuron in &self.neurons {
            let (channel_id, operators) = contract_channel(neuron);
            if neuron.kind == INPUT_KIND {
                inputs.push(ChannelSpec::requires(channel_id, &operators));
            } else if neuron.kind == OUTPUT_KIND {
                outputs.push(ChannelSpec::provides(channel_id, operators));
            }
        }
        (inputs, outputs)
    }
}

fn default_from_port() -> String {
    String::new()
}

fn default_to_port() -> String {
    String::new()
}

/// 🔗️ Directed connection between two port endpoints.
#[derive(Clone, Debug, PartialEq, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Synapse {
    pub id: String,
    pub from: String,
    pub to: String,
    #[cfg_attr(test, serde(default = "default_from_port"))]
    pub from_port: String,
    #[cfg_attr(test, serde(default = "default_to_port"))]
    pub to_port: String,
}

impl ToValue for Synapse {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { native_controlled::encode_synapse(self, control) }
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("from".into(), self.from.to_value()),
            ("to".into(), self.to.to_value()),
            ("fromPort".into(), self.from_port.to_value()),
            ("toPort".into(), self.to_port.to_value()),
        ])
    }
}

impl FromValue for Synapse {
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { native_controlled::decode_synapse(value, control) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Synapse")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "id"))?,
            from: value.get("from").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "from"))?,
            to: value.get("to").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "to"))?,
            from_port: value.get("fromPort").cloned().map(String::from_value).transpose()?.unwrap_or_else(default_from_port),
            to_port: value.get("toPort").cloned().map(String::from_value).transpose()?.unwrap_or_else(default_to_port),
        })
    }
}

impl ToValue for Neuron {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { native_controlled::encode_neuron(self, control) }
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("kind".into(), self.kind.to_value()),
            ("params".into(), self.params.to_value()),
            ("tree".into(), self.tree.to_value()),
        ])
    }
}

impl FromValue for Neuron {
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { native_controlled::decode_neuron(value, control) }
    fn retire_decoded(self) { native_controlled::retire_neuron(self) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Neuron")) };
        let id = value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "id"))?;
        let kind = value.get("kind").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "kind"))?;
        let params = ColdOwner::new(value.get("params").cloned().map(Dictionary::from_value).transpose()?.unwrap_or_default());
        let tree = value.get("tree").cloned().map(Option::<Box<Tree>>::from_value).transpose()?.unwrap_or_default();
        Ok(Self { id, kind, params: params.into_inner(), tree })
    }
}

impl ToValue for Tree {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { native_controlled::encode_tree(self, control) }
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("neurons".into(), self.neurons.to_value()), ("synapses".into(), self.synapses.to_value())])
    }
}

impl FromValue for Tree {
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { native_controlled::decode_tree(value, control) }
    fn retire_decoded(self) { native_controlled::retire_tree(self) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Tree")) };
        let mut neurons = ColdOwner::new(Vec::new());
        if let Some(entries) = value.get("neurons") {
            let DslValue::Array(entries) = entries else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an array for neurons")); };
            for (index, entry) in entries.iter().enumerate() {
                neurons.push(Neuron::from_value(entry.clone()).map_err(|error| error.under(index))?);
            }
        }
        let synapses = value.get("synapses").cloned().map(Vec::<Synapse>::from_value).transpose()?.unwrap_or_default();
        Ok(Self { neurons: neurons.into_inner(), synapses })
    }
}
// #endregion 🔖️Tree
// #region 🔖️Contract
pub const INPUT_KIND: &str = "input";
pub const OUTPUT_KIND: &str = "output";
pub const CLUSTER_KIND: &str = "cluster";

pub fn contract_channel(neuron: &Neuron) -> (String, Vec<String>) {
    let channel_id = neuron.params.get("channel").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).unwrap_or(neuron.id.as_str()).to_string();
    let operators = neuron.params.get("operators").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(|raw| raw.split(',').map(str::trim).filter(|entry| !entry.is_empty()).map(str::to_string).collect()).unwrap_or_default();
    (channel_id, operators)
}

/// 🧩️ Builds operator metadata for a cluster neuron from its inner contract.
pub fn cluster_operator_info(id: &str, name: &str, tree: &Tree) -> OperatorInfo {
    let (inputs, outputs) = tree.contract();
    OperatorInfo { id: id.into(), extension: "flow".into(), name: name.into(), abbreviation: name.into(), icon: "emoji:🧩️".into(), summary: "Nested tree operator".into(), inputs, outputs, ..Default::default() }
}
// #endregion 🔖️Contract
