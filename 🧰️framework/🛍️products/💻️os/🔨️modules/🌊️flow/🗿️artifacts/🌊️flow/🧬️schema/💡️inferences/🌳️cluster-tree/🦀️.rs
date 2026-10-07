//! 🌳️ Owned cluster-tree properties retain exact neural atoms and nested dictionaries.
use graph::manifest::PropertyValue;
use neural_engine::{Atom, ColdDictionaryBuilder, Dictionary, Neuron, Synapse, Tree, Value};
use semio_framework_value::{FromValue, ToValue};

#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct ClusterTree {
    neurons: Vec<ClusterNeuron>,
    synapses: Vec<Synapse>,
}

#[derive(ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct ClusterNeuron {
    id: String,
    kind: String,
    params: Vec<ClusterEntry>,
    tree: Option<Box<ClusterTree>>,
}

#[derive(ToValue, FromValue)]
#[value(deny_unknown_fields)]
struct ClusterEntry {
    key: String,
    value: ClusterValue,
}

#[derive(ToValue, FromValue)]
#[value(tag = "kind", content = "value", rename_all = "camelCase", deny_unknown_fields)]
enum ClusterValue {
    Null,
    Boolean(bool),
    Integer(String),
    Decimal(f64),
    String(String),
    Dictionary(Vec<ClusterEntry>),
}

fn project_dictionary(dictionary: &Dictionary) -> Vec<ClusterEntry> {
    dictionary.iter().map(|(key, value)| ClusterEntry { key: key.clone(), value: match value {
        Value::Atom(Atom::Null) => ClusterValue::Null,
        Value::Atom(Atom::Boolean(value)) => ClusterValue::Boolean(*value),
        Value::Atom(Atom::Integer(value)) => ClusterValue::Integer(value.to_string()),
        Value::Atom(Atom::Decimal(value)) => ClusterValue::Decimal(*value),
        Value::Atom(Atom::String(value)) => ClusterValue::String(value.clone()),
        Value::Dictionary(value) => ClusterValue::Dictionary(project_dictionary(value)),
    } }).collect()
}

fn project_tree(tree: &Tree) -> ClusterTree {
    ClusterTree { neurons: tree.neurons.iter().map(|neuron| ClusterNeuron {
        id: neuron.id.clone(), kind: neuron.kind.clone(), params: project_dictionary(&neuron.params), tree: neuron.tree.as_deref().map(project_tree).map(Box::new),
    }).collect(), synapses: tree.synapses.clone() }
}

fn validate_entries(entries: &[ClusterEntry]) -> Result<(), String> {
    let mut keys = std::collections::HashSet::new();
    for entry in entries {
        if !keys.insert(entry.key.as_str()) { return Err("cluster dictionary repeats a key".into()); }
        match &entry.value {
            ClusterValue::Integer(value) => { value.parse::<i64>().map_err(|_| "cluster integer exceeds i64".to_string())?; }
            ClusterValue::Dictionary(value) => validate_entries(value)?,
            _ => {}
        }
    }
    Ok(())
}

fn validate_tree(tree: &ClusterTree) -> Result<(), String> {
    for neuron in &tree.neurons {
        validate_entries(&neuron.params)?;
        if let Some(tree) = &neuron.tree { validate_tree(tree)?; }
    }
    Ok(())
}

fn reconstruct_dictionary(entries: Vec<ClusterEntry>) -> Dictionary {
    let mut dictionary = ColdDictionaryBuilder::new();
    for entry in entries {
        let value = match entry.value {
            ClusterValue::Null => Value::Atom(Atom::Null),
            ClusterValue::Boolean(value) => Value::Atom(Atom::Boolean(value)),
            ClusterValue::Integer(value) => Value::Atom(Atom::Integer(value.parse().expect("validated cluster integer"))),
            ClusterValue::Decimal(value) => Value::Atom(Atom::Decimal(value)),
            ClusterValue::String(value) => Value::Atom(Atom::String(value)),
            ClusterValue::Dictionary(value) => Value::Dictionary(reconstruct_dictionary(value)),
        };
        dictionary.insert(entry.key, value);
    }
    dictionary.finish()
}

fn reconstruct_tree(tree: ClusterTree) -> Tree {
    Tree { neurons: tree.neurons.into_iter().map(|neuron| Neuron {
        id: neuron.id, kind: neuron.kind, params: reconstruct_dictionary(neuron.params), tree: neuron.tree.map(|tree| Box::new(reconstruct_tree(*tree))),
    }).collect(), synapses: tree.synapses }
}

/// 🌳️ Projects a neural tree into an owned graph property without encoding text.
pub fn cluster_tree_property(tree: &Tree) -> PropertyValue {
    PropertyValue::from_value(project_tree(tree).to_value()).expect("cluster property uses intrinsic graph values")
}

/// 🌲️ Reconstructs a neural tree from its typed graph property.
pub fn cluster_tree_from_property(property: &PropertyValue) -> Result<Tree, String> {
    let tree = ClusterTree::from_value(property.to_value()).map_err(|error| error.to_string())?;
    validate_tree(&tree)?;
    Ok(reconstruct_tree(tree))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
