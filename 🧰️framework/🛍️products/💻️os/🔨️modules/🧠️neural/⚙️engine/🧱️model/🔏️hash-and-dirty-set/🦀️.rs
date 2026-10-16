//! 🔏️ Hash and dirty set: the neuron and subtree hashing and the dirty-set propagation.
//!
//! Holds the `🔖️Cache`, `🔖️DirtyPropagation` regions of the module; the root re-exports its items.

use super::*;

// #region 🔖️Cache
fn hash_str<H: Hasher>(hasher: &mut H, value: &str) {
    value.hash(hasher);
}

fn hash_atom<H: Hasher>(hasher: &mut H, atom: &Atom) {
    match atom {
        Atom::Null => 0u8.hash(hasher),
        Atom::Boolean(value) => value.hash(hasher),
        Atom::Integer(value) => value.hash(hasher),
        Atom::Decimal(value) => value.to_bits().hash(hasher),
        Atom::String(value) => hash_str(hasher, value),
    }
}

fn hash_value<H: Hasher>(hasher: &mut H, value: &Value) {
    match value {
        Value::Atom(atom) => hash_atom(hasher, atom),
        Value::Dictionary(dict) => {
            0u8.hash(hasher);
            hash_dictionary(hasher, dict);
        }
    }
}

fn hash_dictionary<H: Hasher>(hasher: &mut H, dictionary: &Dictionary) {
    for (key, value) in &dictionary.pairs {
        hash_str(hasher, key);
        hash_value(hasher, value);
    }
}

/// 🔑️ Content-addressable cache key from operator kind and resolved input dictionary.
pub fn node_hash(kind: &str, input: &Dictionary) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hash_str(&mut hasher, kind);
    hash_dictionary(&mut hasher, input);
    hasher.finish()
}
// #endregion 🔖️Cache
// #region 🔖️DirtyPropagation
fn hash_neuron_key<H: Hasher>(hasher: &mut H, neuron: &Neuron) {
    hash_str(hasher, &neuron.kind);
    hash_dictionary(hasher, &neuron.params);
    if let Some(sub_tree) = neuron.tree.as_deref() {
        1u8.hash(hasher);
        hash_subtree(hasher, sub_tree);
    } else {
        0u8.hash(hasher);
    }
}

fn hash_subtree<H: Hasher>(hasher: &mut H, tree: &Tree) {
    let mut neurons: Vec<&Neuron> = tree.neurons.iter().collect();
    neurons.sort_by(|a, b| a.id.cmp(&b.id));
    for neuron in neurons {
        hash_str(hasher, &neuron.id);
        hash_neuron_key(hasher, neuron);
    }
    let mut synapses: Vec<&Synapse> = tree.synapses.iter().collect();
    synapses.sort_by(|a, b| (&a.from, &a.from_port, &a.to, &a.to_port).cmp(&(&b.from, &b.from_port, &b.to, &b.to_port)));
    for syn in synapses {
        hash_str(hasher, &syn.from);
        hash_str(hasher, &syn.from_port);
        hash_str(hasher, &syn.to);
        hash_str(hasher, &syn.to_port);
    }
}

fn neuron_key_hash(neuron: &Neuron) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hash_neuron_key(&mut hasher, neuron);
    hasher.finish()
}

fn incoming_edges_signature(tree: &Tree, neuron_id: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let mut incoming: Vec<&Synapse> = tree.synapses.iter().filter(|syn| syn.to == neuron_id).collect();
    incoming.sort_by(|a, b| (&a.from, &a.from_port, &a.to_port).cmp(&(&b.from, &b.from_port, &b.to_port)));
    for syn in incoming {
        hash_str(&mut hasher, &syn.from);
        hash_str(&mut hasher, &syn.from_port);
        hash_str(&mut hasher, &syn.to_port);
    }
    hasher.finish()
}

/// 📸️ Retains the exact original tree and seed source allocations of an evaluation.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeSnapshot {
    pub(crate) tree: Arc<Tree>,
    pub(crate) seeds: Arc<HistoryFoldIndex<String,Dictionary>>,
}
impl Default for TreeSnapshot{fn default()->Self{Self{tree:Arc::new(Tree::default()),seeds:Arc::new(HistoryFoldIndex::new())}}}
impl TreeSnapshot {
    /// 🧊️ Captures synchronous diagnostic inputs within their existing cold caller scope.
    pub fn capture(tree:&Tree,seeds:&HashMap<String,Dictionary>)->Self{Self{tree:Arc::new(tree.clone()),seeds:Arc::new(seeds.iter().map(|(key,value)|(key.clone(),value.clone())).collect())}}
    /// 🔗️ Moves the exact preborn immutable sources under the original admitted handoff.
    pub fn capture_retained(tree:Arc<Tree>,seeds:Arc<HistoryFoldIndex<String,Dictionary>>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,Arc<Tree>,Arc<HistoryFoldIndex<String,Dictionary>>)>{
        if grant.maximum_items==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original snapshot handoff requires admitted item"),tree,seeds))}
        if grant.maximum_depth==0{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original snapshot handoff requires admitted depth"),tree,seeds))}
        Ok((Self{tree,seeds},RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    pub fn tree_source(&self)->&Tree{&self.tree}
    pub fn seed_source(&self)->&HistoryFoldIndex<String,Dictionary>{&self.seeds}
}
/// 🧭️ Diagnoses original structural, wiring, seed, removal, and downstream changes synchronously.
pub fn compute_dirty_set(previous:Option<&TreeSnapshot>,current:&TreeSnapshot)->HashSet<String>{
    let Some(previous)=previous else{return current.tree.neurons.iter().map(|node|node.id.clone()).collect()};
    let contains=|id:&str|current.tree.neurons.iter().any(|node|node.id==id);
    let mut direct=HashSet::new();
    for node in &current.tree.neurons{
        let original=previous.tree.neurons.iter().find(|old|old.id==node.id);
        let changed=original.is_none_or(|old|neuron_key_hash(old)!=neuron_key_hash(node)||incoming_edges_signature(&previous.tree,&old.id)!=incoming_edges_signature(&current.tree,&node.id));
        if changed||previous.seeds.get(&node.id)!=current.seeds.get(&node.id){direct.insert(node.id.clone());}
    }
    for removed in &previous.tree.neurons{if !contains(&removed.id){for edge in &previous.tree.synapses{if edge.from==removed.id&&contains(&edge.to){direct.insert(edge.to.clone());}}}}
    let mut dirty=HashSet::new();let mut queue:VecDeque<String>=direct.into_iter().collect();
    while let Some(id)=queue.pop_front(){if !dirty.insert(id.clone()){continue}for edge in &current.tree.synapses{if edge.from==id&&contains(&edge.to)&&!dirty.contains(&edge.to){queue.push_back(edge.to.clone());}}}
    dirty
}
// #endregion 🔖️DirtyPropagation
